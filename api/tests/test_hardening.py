"""Tests for API hardening: request IDs, CORS, error response standardization."""

from __future__ import annotations

import uuid

import pytest
from fastapi.testclient import TestClient


# ---------- Request ID middleware ----------


def test_request_id_generated_on_response(client: TestClient) -> None:
    resp = client.get("/api/v1/health")
    assert "x-request-id" in resp.headers
    # Should be a valid UUID
    uuid.UUID(resp.headers["x-request-id"])


def test_request_id_passthrough(client: TestClient) -> None:
    custom_id = "custom-req-12345"
    resp = client.get("/api/v1/health", headers={"X-Request-Id": custom_id})
    assert resp.headers["x-request-id"] == custom_id


def test_request_id_unique_per_request(client: TestClient) -> None:
    resp1 = client.get("/api/v1/health")
    resp2 = client.get("/api/v1/health")
    assert resp1.headers["x-request-id"] != resp2.headers["x-request-id"]


def test_request_id_on_error_response(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/account/api-key")
    assert resp.status_code == 401
    body = resp.json()
    assert "request_id" in body
    assert body["request_id"] != ""


def test_request_id_on_auth_error(raw_client: TestClient) -> None:
    resp = raw_client.get(
        "/api/v1/account/usage",
        headers={"Authorization": "Bearer invalid_token"},
    )
    assert resp.status_code == 401
    assert "x-request-id" in resp.headers


# ---------- CORS ----------


def test_cors_default_origin(client: TestClient) -> None:
    resp = client.options(
        "/api/v1/health",
        headers={
            "Origin": "http://localhost:5173",
            "Access-Control-Request-Method": "GET",
        },
    )
    # Should allow the default origin
    assert resp.status_code in (200, 204)


def test_cors_rejects_unknown_origin(client: TestClient) -> None:
    resp = client.options(
        "/api/v1/health",
        headers={
            "Origin": "https://evil.example.com",
            "Access-Control-Request-Method": "GET",
        },
    )
    # CORS preflight should NOT include the evil origin in allow-origin
    allow_origin = resp.headers.get("access-control-allow-origin", "")
    assert "evil.example.com" not in allow_origin


# ---------- Error response standardization ----------


def test_error_response_has_error_field(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/account/api-key")
    body = resp.json()
    assert "error" in body
    assert "detail" in body


def test_error_response_has_request_id_field(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/account/api-key")
    body = resp.json()
    assert "request_id" in body


def test_rate_limit_error_has_request_id(
    client: TestClient, auth_headers: dict[str, str]
) -> None:
    """Exhaust rate limit and verify error response includes request_id."""
    from app.services.stores import rate_limit_store, user_store
    from app.services.api_keys import TEST_USER_ID

    # Fill up rate limit for 'other' family
    today = __import__("datetime").datetime.now(
        __import__("datetime").timezone.utc
    ).strftime("%Y-%m-%d")
    for _ in range(500):
        rate_limit_store.increment(TEST_USER_ID, "other", today)

    resp = client.get("/api/v1/account/usage")
    assert resp.status_code == 429
    body = resp.json()
    assert "request_id" in body
    assert body["error"] == "RATE_LIMIT_EXCEEDED"


def test_engine_error_has_request_id(
    client: TestClient, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("bad.spdf", b"PK\x03\x04corrupted", "application/octet-stream")},
    )
    assert resp.status_code in (400, 422, 500)
    body = resp.json()
    assert "request_id" in body
    assert "error" in body


# ---------- Health endpoint ----------


def test_health_no_auth_required(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/health")
    assert resp.status_code == 200
    assert resp.json()["status"] == "ok"


def test_health_has_request_id(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/health")
    assert "x-request-id" in resp.headers
