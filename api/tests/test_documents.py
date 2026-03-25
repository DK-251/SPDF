"""Endpoint tests for the SPDF document API."""

from __future__ import annotations

import json
from typing import Any

import pytest
from fastapi.testclient import TestClient


def _engine_check() -> bool:
    try:
        import spdf_native  # noqa: F401

        return True
    except ImportError:
        return False


needs_engine = pytest.mark.skipif(
    not _engine_check(),
    reason="spdf_native not available",
)


# ---------------------------------------------------------------------------
# Health
# ---------------------------------------------------------------------------


def test_health_returns_ok(client: TestClient) -> None:
    resp = client.get("/api/v1/health")
    assert resp.status_code == 200
    body = resp.json()
    assert body["status"] == "ok"
    assert "engine_version" in body
    assert body["api_version"] == "0.1.0"


# ---------------------------------------------------------------------------
# Generate
# ---------------------------------------------------------------------------


@needs_engine
def test_generate_returns_spdf_bytes(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200
    assert resp.headers["content-type"] == "application/octet-stream"
    assert resp.content[:4] == b"PK\x03\x04"


@needs_engine
def test_generate_with_defaults(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200
    assert len(resp.content) > 0


@needs_engine
def test_generate_invalid_semantic_returns_422(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/generate", json={"semantic": {"bad": "data"}}
    )
    assert resp.status_code == 422
    body = resp.json()
    assert body["error"] == "INVALID_PAYLOAD"


def test_generate_empty_body_returns_422(client: TestClient) -> None:
    resp = client.post("/api/v1/documents/generate", content=b"{}")
    assert resp.status_code == 422


# ---------------------------------------------------------------------------
# Validate
# ---------------------------------------------------------------------------


@needs_engine
def test_validate_valid_spdf(client: TestClient, sample_spdf_bytes: bytes) -> None:
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("test.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    assert resp.status_code == 200
    body = resp.json()
    assert body["valid"] is True
    assert body["error_count"] == 0


@needs_engine
def test_validate_returns_report_structure(
    client: TestClient, sample_spdf_bytes: bytes
) -> None:
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("test.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    body = resp.json()
    for key in ("valid", "manifest_errors", "document_errors", "error_count", "fatal_count"):
        assert key in body


def test_validate_corrupt_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("bad.spdf", b"not a zip file at all", "application/octet-stream")},
    )
    assert resp.status_code == 400
    assert resp.json()["error"] == "INVALID_CONTAINER"


# ---------------------------------------------------------------------------
# Render
# ---------------------------------------------------------------------------


@needs_engine
def test_render_returns_pdf(client: TestClient, sample_spdf_bytes: bytes) -> None:
    resp = client.post(
        "/api/v1/documents/render",
        files={"file": ("test.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    assert resp.status_code == 200
    assert resp.headers["content-type"] == "application/pdf"
    assert resp.content[:5] == b"%PDF-"


def test_render_corrupt_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/render",
        files={"file": ("bad.spdf", b"garbage data here", "application/octet-stream")},
    )
    assert resp.status_code == 400


# ---------------------------------------------------------------------------
# Parse
# ---------------------------------------------------------------------------


@needs_engine
def test_parse_valid_semantic(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    resp = client.post(
        "/api/v1/documents/parse",
        json={"semantic_json": json.dumps(sample_semantic)},
    )
    assert resp.status_code == 200
    body = resp.json()
    assert "document" in body
    assert body["document"]["title"] == "Invoice INV-2026-200"


@needs_engine
def test_parse_invalid_json_returns_422(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/parse",
        json={"semantic_json": "{not valid json!!!}"},
    )
    assert resp.status_code == 422


# ---------------------------------------------------------------------------
# Extract
# ---------------------------------------------------------------------------


@needs_engine
def test_extract_returns_invoice_data(
    client: TestClient, sample_spdf_bytes: bytes
) -> None:
    resp = client.post(
        "/api/v1/documents/extract",
        files={"file": ("test.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    assert resp.status_code == 200
    body = resp.json()
    assert body["invoice_number"] == "INV-2026-200"
    assert body["total"] == "132750.00"
    assert body["vendor"]["name"] == "SPDF Corp"


def test_extract_corrupt_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/extract",
        files={"file": ("bad.spdf", b"not a zip", "application/octet-stream")},
    )
    assert resp.status_code == 400


# ---------------------------------------------------------------------------
# File size limit
# ---------------------------------------------------------------------------


def test_upload_too_large_returns_413(client: TestClient) -> None:
    # ZIP magic + 100MB of zeros
    huge = b"PK\x03\x04" + (b"\x00" * (100 * 1024 * 1024 + 1))
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("huge.spdf", huge, "application/octet-stream")},
    )
    assert resp.status_code == 413
