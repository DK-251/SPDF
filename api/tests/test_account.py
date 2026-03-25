"""Tests for account management endpoints: API key and usage."""

from __future__ import annotations

from typing import Any

import pytest
from fastapi.testclient import TestClient

from app.services.api_keys import TEST_API_KEY, TEST_USER_ID
from app.services.stores import rate_limit_store, user_store


# ---------------------------------------------------------------------------
# GET /api/v1/account/api-key
# ---------------------------------------------------------------------------


def test_get_api_key_returns_prefix(client: TestClient) -> None:
    resp = client.get("/api/v1/account/api-key")
    assert resp.status_code == 200
    body = resp.json()
    assert "key_prefix" in body
    assert body["key_prefix"] == TEST_API_KEY[:16]
    assert "created_at" in body
    assert "last_used_at" in body


def test_get_api_key_never_returns_full_key(client: TestClient) -> None:
    resp = client.get("/api/v1/account/api-key")
    body = resp.json()
    assert TEST_API_KEY not in str(body)
    assert len(body["key_prefix"]) == 16


def test_get_api_key_401_without_auth(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/account/api-key")
    assert resp.status_code == 401
    assert resp.json()["error"] == "UNAUTHORIZED"


def test_get_api_key_401_with_invalid_key(raw_client: TestClient) -> None:
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": "Bearer sk_test_invalid_key_00000000"},
    )
    assert resp.status_code == 401


def test_get_api_key_401_malformed_header(raw_client: TestClient) -> None:
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": "Token sk_test_dev_000000000000000000"},
    )
    assert resp.status_code == 401


def test_get_api_key_method_not_allowed(client: TestClient) -> None:
    resp = client.delete("/api/v1/account/api-key")
    assert resp.status_code == 405


# ---------------------------------------------------------------------------
# POST /api/v1/account/api-key/rotate
# ---------------------------------------------------------------------------


def test_rotate_returns_full_key(client: TestClient) -> None:
    resp = client.post("/api/v1/account/api-key/rotate")
    assert resp.status_code == 200
    body = resp.json()
    assert "api_key" in body
    assert body["api_key"].startswith("sk_live_")
    assert len(body["api_key"]) > 16
    assert body["key_prefix"] == body["api_key"][:16]
    assert "created_at" in body
    assert "warning" in body


def test_rotate_new_key_works(client: TestClient, raw_client: TestClient) -> None:
    resp = client.post("/api/v1/account/api-key/rotate")
    new_key = resp.json()["api_key"]
    resp2 = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {new_key}"},
    )
    assert resp2.status_code == 200
    assert resp2.json()["key_prefix"] == new_key[:16]


def test_rotate_old_key_invalidated(client: TestClient, raw_client: TestClient) -> None:
    client.post("/api/v1/account/api-key/rotate")
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {TEST_API_KEY}"},
    )
    assert resp.status_code == 401


def test_rotate_401_without_auth(raw_client: TestClient) -> None:
    resp = raw_client.post("/api/v1/account/api-key/rotate")
    assert resp.status_code == 401


def test_rotate_method_not_allowed(client: TestClient) -> None:
    resp = client.get("/api/v1/account/api-key/rotate")
    assert resp.status_code == 405


# ---------------------------------------------------------------------------
# GET /api/v1/account/usage
# ---------------------------------------------------------------------------


def test_usage_returns_zero_initially(client: TestClient) -> None:
    resp = client.get("/api/v1/account/usage")
    assert resp.status_code == 200
    body = resp.json()
    assert body["user_id"] == TEST_USER_ID
    assert body["tier"] == "FREE"
    assert "limits" in body
    assert "next_reset_at" in body


def test_usage_includes_correct_limits(client: TestClient) -> None:
    resp = client.get("/api/v1/account/usage")
    limits = resp.json()["limits"]
    assert limits["generate"] == 50
    assert limits["extract"] == 100
    assert limits["other"] == 500


def test_usage_increments_after_requests(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    resp = client.get("/api/v1/account/usage")
    usage = resp.json()["usage"]
    assert usage.get("generate", 0) >= 2


def test_usage_401_without_auth(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/account/usage")
    assert resp.status_code == 401


def test_usage_method_not_allowed(client: TestClient) -> None:
    resp = client.post("/api/v1/account/usage")
    assert resp.status_code == 405
