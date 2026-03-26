"""Rate-limiting middleware with per-user, per-endpoint-family, per-day quotas.

Adds authentication (API key verification) and rate limit enforcement.
Attaches the authenticated user dict to ``request.state.user``.
"""

from __future__ import annotations

import json
import math
from datetime import datetime, timezone

from starlette.middleware.base import BaseHTTPMiddleware, RequestResponseEndpoint
from starlette.requests import Request
from starlette.responses import Response

from app.errors import RATE_LIMIT_EXCEEDED, UNAUTHORIZED
from app.services.api_keys import verify_api_key
from app.services.jwt_auth import decode_jwt
from app.services.stores import (
    ENDPOINT_FAMILIES,
    PUBLIC_PATHS,
    TIER_LIMITS,
    rate_limit_store,
    user_store,
)


def _seconds_until_midnight_utc() -> int:
    now = datetime.now(timezone.utc)
    midnight = now.replace(hour=0, minute=0, second=0, microsecond=0)
    from datetime import timedelta

    next_midnight = midnight + timedelta(days=1)
    return int((next_midnight - now).total_seconds())


def _midnight_utc_epoch() -> int:
    now = datetime.now(timezone.utc)
    midnight = now.replace(hour=0, minute=0, second=0, microsecond=0)
    from datetime import timedelta

    next_midnight = midnight + timedelta(days=1)
    return int(next_midnight.timestamp())


def _today_utc() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%d")


def _error_response(
    status_code: int,
    error: str,
    detail: str,
    headers: dict[str, str] | None = None,
    request_id: str | None = None,
) -> Response:
    body_dict: dict[str, str] = {"error": error, "detail": detail}
    if request_id:
        body_dict["request_id"] = request_id
    body = json.dumps(body_dict)
    h = {"content-type": "application/json"}
    if request_id:
        h["X-Request-Id"] = request_id
    if headers:
        h.update(headers)
    return Response(content=body, status_code=status_code, headers=h)


class RateLimitMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request: Request, call_next: RequestResponseEndpoint) -> Response:
        path = request.url.path
        request_id = getattr(request.state, "request_id", None)

        # Public paths skip auth and rate limiting
        if path in PUBLIC_PATHS:
            return await call_next(request)

        # --- Authentication (API key OR JWT) ---
        auth_header = request.headers.get("authorization", "")
        if not auth_header.startswith("Bearer "):
            return _error_response(401, UNAUTHORIZED, "Missing or malformed Authorization header.", request_id=request_id)

        token = auth_header[7:]  # strip "Bearer "
        if not token:
            return _error_response(401, UNAUTHORIZED, "Token is empty.", request_id=request_id)

        user = None

        if token.startswith("sk_"):
            # API key authentication
            for u in user_store.iter_users():
                if u["api_key_hash"] and verify_api_key(token, u["api_key_hash"]):
                    user = u
                    break
        elif token.startswith("eyJ"):
            # JWT authentication
            try:
                claims = decode_jwt(token)
                email = claims.get("sub", "")
                if email:
                    user = user_store.find_by_email(email)
            except Exception:
                return _error_response(401, UNAUTHORIZED, "Invalid or expired JWT token.", request_id=request_id)

        if not user:
            return _error_response(401, UNAUTHORIZED, "Invalid credentials.", request_id=request_id)

        request.state.user = user
        user_store.touch_last_used(user["id"])

        # --- Rate limiting ---
        family = ENDPOINT_FAMILIES.get(path, "other")
        tier = user["tier"]
        today = _today_utc()

        # Enterprise tier is unlimited
        if tier == "ENTERPRISE":
            response = await call_next(request)
            response.headers["X-RateLimit-Limit"] = "unlimited"
            response.headers["X-RateLimit-Remaining"] = "unlimited"
            response.headers["X-RateLimit-Reset"] = str(_midnight_utc_epoch())
            return response

        tier_limits = TIER_LIMITS.get(tier, TIER_LIMITS["FREE"])
        limit = tier_limits.get(family, tier_limits.get("other", 500))

        current_count = rate_limit_store.get_count(user["id"], family, today)

        if current_count >= limit:
            retry_after = _seconds_until_midnight_utc()
            return _error_response(
                429,
                RATE_LIMIT_EXCEEDED,
                f"Rate limit exceeded for {family}. Resets at UTC midnight.",
                request_id=request_id,
                headers={
                    "X-RateLimit-Limit": str(limit),
                    "X-RateLimit-Remaining": "0",
                    "X-RateLimit-Reset": str(_midnight_utc_epoch()),
                    "Retry-After": str(retry_after),
                },
            )

        # Increment counter and proceed
        new_count = rate_limit_store.increment(user["id"], family, today)
        response = await call_next(request)

        remaining = max(0, limit - new_count)
        response.headers["X-RateLimit-Limit"] = str(limit)
        response.headers["X-RateLimit-Remaining"] = str(remaining)
        response.headers["X-RateLimit-Reset"] = str(_midnight_utc_epoch())

        return response
