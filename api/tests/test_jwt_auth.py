"""Tests for JWT authentication: token validation, dual auth, error cases."""

from __future__ import annotations

from typing import Any

import pytest
from fastapi.testclient import TestClient

from app.services.api_keys import TEST_API_KEY
from app.services.jwt_auth import TEST_JWT_ISSUER, TEST_JWT_SECRET, create_test_token


# ---------------------------------------------------------------------------
# Valid JWT auth
# ---------------------------------------------------------------------------


def test_jwt_auth_returns_200(jwt_client: TestClient) -> None:
    resp = jwt_client.get("/api/v1/account/api-key")
    assert resp.status_code == 200


def test_jwt_auth_resolves_correct_user(jwt_client: TestClient) -> None:
    resp = jwt_client.get("/api/v1/account/usage")
    assert resp.status_code == 200
    body = resp.json()
    assert body["tier"] == "FREE"
    assert body["user_id"] is not None


def test_jwt_auth_rate_limit_headers(jwt_client: TestClient) -> None:
    resp = jwt_client.get("/api/v1/account/api-key")
    assert "X-RateLimit-Limit" in resp.headers
    assert "X-RateLimit-Remaining" in resp.headers
    assert "X-RateLimit-Reset" in resp.headers


# ---------------------------------------------------------------------------
# Dual auth: both API key and JWT work
# ---------------------------------------------------------------------------


def test_api_key_still_works(client: TestClient) -> None:
    resp = client.get("/api/v1/account/api-key")
    assert resp.status_code == 200


def test_both_auth_methods_on_same_endpoint(
    client: TestClient, jwt_client: TestClient
) -> None:
    resp_key = client.get("/api/v1/account/usage")
    resp_jwt = jwt_client.get("/api/v1/account/usage")
    assert resp_key.status_code == 200
    assert resp_jwt.status_code == 200
    assert resp_key.json()["user_id"] == resp_jwt.json()["user_id"]


# ---------------------------------------------------------------------------
# Expired JWT
# ---------------------------------------------------------------------------


def test_expired_jwt_returns_401(raw_client: TestClient) -> None:
    token = create_test_token("test@spdf.dev", exp_delta_seconds=-60)
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 401


# ---------------------------------------------------------------------------
# Bad signature
# ---------------------------------------------------------------------------


def test_bad_signature_returns_401(raw_client: TestClient) -> None:
    token = create_test_token("test@spdf.dev", secret="wrong-secret")
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 401


# ---------------------------------------------------------------------------
# Missing / invalid claims
# ---------------------------------------------------------------------------


def test_unknown_email_returns_401(raw_client: TestClient) -> None:
    token = create_test_token("nobody@example.com")
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 401


def test_wrong_issuer_returns_401(raw_client: TestClient) -> None:
    token = create_test_token("test@spdf.dev", issuer="wrong-issuer")
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 401


# ---------------------------------------------------------------------------
# Health still public
# ---------------------------------------------------------------------------


def test_health_bypasses_jwt_auth(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/health")
    assert resp.status_code == 200
    assert resp.json()["status"] == "ok"


# ---------------------------------------------------------------------------
# Malformed JWT
# ---------------------------------------------------------------------------


def test_garbage_jwt_returns_401(raw_client: TestClient) -> None:
    resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": "Bearer eyJgarbage.not.valid"},
    )
    assert resp.status_code == 401
