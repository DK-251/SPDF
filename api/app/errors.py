"""Structured error handling for the SPDF API.

Maps Rust engine errors (surfaced as ValueError via PyO3) to proper HTTP
status codes and JSON error bodies. All error responses include request_id.
"""

from __future__ import annotations

import logging
from typing import Any

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from pydantic import BaseModel

logger = logging.getLogger(__name__)


class ErrorResponse(BaseModel):
    error: str
    detail: str
    request_id: str | None = None


class SpdfApiError(Exception):
    """Structured API error with HTTP status code and error code."""

    def __init__(self, status_code: int, error_code: str, detail: str) -> None:
        self.status_code = status_code
        self.error_code = error_code
        self.detail = detail
        super().__init__(detail)


RATE_LIMIT_EXCEEDED = "RATE_LIMIT_EXCEEDED"
UNAUTHORIZED = "UNAUTHORIZED"


def handle_engine_error(exc: Exception) -> SpdfApiError:
    """Translate a Rust engine ValueError into an appropriate SpdfApiError."""
    msg = str(exc).lower()

    if any(kw in msg for kw in ("checksum", "corrupt", "missing entry", "decompression")):
        return SpdfApiError(400, "INVALID_CONTAINER", str(exc))

    if any(kw in msg for kw in ("json", "parse", "invalid", "expected")):
        return SpdfApiError(422, "INVALID_PAYLOAD", str(exc))

    if "size exceeds" in msg or "file size" in msg:
        return SpdfApiError(413, "FILE_TOO_LARGE", str(exc))

    return SpdfApiError(500, "ENGINE_ERROR", str(exc))


def _get_request_id(request: Request) -> str | None:
    return getattr(request.state, "request_id", None)


def _error_json(error: str, detail: str, status_code: int, request_id: str | None = None) -> JSONResponse:
    content: dict[str, Any] = {"error": error, "detail": detail}
    if request_id:
        content["request_id"] = request_id
    return JSONResponse(status_code=status_code, content=content)


def register_exception_handlers(app: FastAPI) -> None:
    """Attach all exception handlers to the FastAPI app."""

    @app.exception_handler(SpdfApiError)
    async def spdf_api_error_handler(
        request: Request, exc: SpdfApiError
    ) -> JSONResponse:
        return _error_json(exc.error_code, exc.detail, exc.status_code, _get_request_id(request))

    @app.exception_handler(ValueError)
    async def value_error_handler(
        request: Request, exc: ValueError
    ) -> JSONResponse:
        mapped = handle_engine_error(exc)
        return _error_json(mapped.error_code, mapped.detail, mapped.status_code, _get_request_id(request))

    @app.exception_handler(Exception)
    async def generic_error_handler(
        request: Request, exc: Exception
    ) -> JSONResponse:
        logger.exception("Unhandled exception: %s", exc)
        return _error_json("INTERNAL_ERROR", "An unexpected error occurred.", 500, _get_request_id(request))
