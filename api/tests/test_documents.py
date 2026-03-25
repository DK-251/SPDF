"""Endpoint tests for the SPDF document API.

Covers happy paths, error responses, edge cases, and content-type validation
for all 6 endpoints: health, generate, validate, render, parse, extract.
"""

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


def test_health_method_not_allowed(client: TestClient) -> None:
    resp = client.post("/api/v1/health")
    assert resp.status_code == 405


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
    assert "content-disposition" in resp.headers


@needs_engine
def test_generate_with_defaults(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200
    assert len(resp.content) > 0


@needs_engine
def test_generate_with_custom_layers(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    resp = client.post(
        "/api/v1/documents/generate",
        json={
            "semantic": sample_semantic,
            "layout": {"page_size": "A4", "margins": {"top": 72}},
            "styles": {"font": "Helvetica", "heading_size": 24},
            "metadata": {"author": "Test Suite", "department": "Engineering"},
            "audit": {"entries": [{"action": "created", "timestamp": "2026-03-25T00:00:00Z"}]},
        },
    )
    assert resp.status_code == 200
    assert resp.content[:4] == b"PK\x03\x04"


@needs_engine
def test_generate_non_invoice_document(
    client: TestClient, sample_non_invoice_semantic: dict[str, Any]
) -> None:
    resp = client.post(
        "/api/v1/documents/generate", json={"semantic": sample_non_invoice_semantic}
    )
    assert resp.status_code == 200
    assert resp.content[:4] == b"PK\x03\x04"


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


def test_generate_missing_semantic_field_returns_422(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/generate",
        json={"layout": {"layout": "default"}},
    )
    assert resp.status_code == 422


def test_generate_not_json_returns_422(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/generate",
        content=b"not json at all",
        headers={"content-type": "application/json"},
    )
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
    assert body["fatal_count"] == 0


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
    assert isinstance(body["manifest_errors"], list)
    assert isinstance(body["document_errors"], list)


def test_validate_corrupt_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("bad.spdf", b"not a zip file at all", "application/octet-stream")},
    )
    assert resp.status_code == 400
    assert resp.json()["error"] == "INVALID_CONTAINER"


def test_validate_empty_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("empty.spdf", b"", "application/octet-stream")},
    )
    assert resp.status_code == 400


def test_validate_no_file_field_returns_422(client: TestClient) -> None:
    resp = client.post("/api/v1/documents/validate")
    assert resp.status_code == 422


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
    assert "content-disposition" in resp.headers


def test_render_corrupt_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/render",
        files={"file": ("bad.spdf", b"garbage data here", "application/octet-stream")},
    )
    assert resp.status_code == 400


def test_render_empty_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/render",
        files={"file": ("empty.spdf", b"", "application/octet-stream")},
    )
    assert resp.status_code == 400


def test_render_no_file_field_returns_422(client: TestClient) -> None:
    resp = client.post("/api/v1/documents/render")
    assert resp.status_code == 422


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
def test_parse_preserves_financial_values(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    resp = client.post(
        "/api/v1/documents/parse",
        json={"semantic_json": json.dumps(sample_semantic)},
    )
    assert resp.status_code == 200
    doc = resp.json()["document"]
    payment = None
    for page in doc["pages"]:
        for elem in page["elements"]:
            if elem.get("element_type") == "PaymentTerms":
                payment = elem
    assert payment is not None
    assert payment["total"] == "132750.00"
    assert payment["subtotal"] == "125000.00"


@needs_engine
def test_parse_invalid_json_returns_422(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/parse",
        json={"semantic_json": "{not valid json!!!}"},
    )
    assert resp.status_code == 422


@needs_engine
def test_parse_wrong_schema_returns_422(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/parse",
        json={"semantic_json": '{"valid_json": true, "but_wrong": "schema"}'},
    )
    assert resp.status_code == 422


def test_parse_missing_field_returns_422(client: TestClient) -> None:
    resp = client.post("/api/v1/documents/parse", json={})
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
    assert body["subtotal"] == "125000.00"
    assert body["vendor"]["name"] == "SPDF Corp"
    assert body["client"]["name"] == "Acme Industries"
    assert body["currency"] == "INR"
    assert len(body["line_items"]) == 2


@needs_engine
def test_extract_financial_values_are_strings(
    client: TestClient, sample_spdf_bytes: bytes
) -> None:
    resp = client.post(
        "/api/v1/documents/extract",
        files={"file": ("test.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    body = resp.json()
    for field in ("subtotal", "total", "tax_amount", "discount"):
        val = body.get(field)
        if val is not None:
            assert isinstance(val, str), f"{field} should be string, got {type(val)}"


@needs_engine
def test_extract_non_invoice_returns_nulls(
    client: TestClient,
    sample_non_invoice_semantic: dict[str, Any],
    engine_available: bool,
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    gen_resp = client.post(
        "/api/v1/documents/generate", json={"semantic": sample_non_invoice_semantic}
    )
    assert gen_resp.status_code == 200
    resp = client.post(
        "/api/v1/documents/extract",
        files={"file": ("plain.spdf", gen_resp.content, "application/octet-stream")},
    )
    assert resp.status_code == 200
    body = resp.json()
    assert body["invoice_number"] is None
    assert body["total"] is None
    assert body["line_items"] == []


def test_extract_corrupt_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/extract",
        files={"file": ("bad.spdf", b"not a zip", "application/octet-stream")},
    )
    assert resp.status_code == 400


def test_extract_empty_file_returns_400(client: TestClient) -> None:
    resp = client.post(
        "/api/v1/documents/extract",
        files={"file": ("empty.spdf", b"", "application/octet-stream")},
    )
    assert resp.status_code == 400


def test_extract_no_file_field_returns_422(client: TestClient) -> None:
    resp = client.post("/api/v1/documents/extract")
    assert resp.status_code == 422


# ---------------------------------------------------------------------------
# File size limit
# ---------------------------------------------------------------------------


def test_upload_too_large_returns_413(client: TestClient) -> None:
    huge = b"PK\x03\x04" + (b"\x00" * (100 * 1024 * 1024 + 1))
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("huge.spdf", huge, "application/octet-stream")},
    )
    assert resp.status_code == 413


# ---------------------------------------------------------------------------
# 404 for unknown routes
# ---------------------------------------------------------------------------


def test_unknown_route_returns_404(client: TestClient) -> None:
    resp = client.get("/api/v1/documents/nonexistent")
    assert resp.status_code in (404, 405)
