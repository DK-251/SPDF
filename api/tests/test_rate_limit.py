"""Tests for rate-limiting middleware: auth enforcement, quota checks, headers."""

from __future__ import annotations

from typing import Any

import pytest
from fastapi.testclient import TestClient

from app.services.api_keys import TEST_API_KEY, TEST_USER_ID
from app.services.stores import TIER_LIMITS, rate_limit_store, user_store


# ---------------------------------------------------------------------------
# Auth enforcement
# ---------------------------------------------------------------------------


def test_missing_auth_returns_401(raw_client: TestClient) -> None:
    resp = raw_client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp.status_code == 401
    body = resp.json()
    assert body["error"] == "UNAUTHORIZED"


def test_malformed_auth_returns_401(raw_client: TestClient) -> None:
    resp = raw_client.post(
        "/api/v1/documents/generate",
        json={"semantic": {}},
        headers={"Authorization": "Token some_value"},
    )
    assert resp.status_code == 401


def test_empty_bearer_returns_401(raw_client: TestClient) -> None:
    resp = raw_client.post(
        "/api/v1/documents/generate",
        json={"semantic": {}},
        headers={"Authorization": "Bearer "},
    )
    assert resp.status_code == 401


def test_invalid_key_returns_401(raw_client: TestClient) -> None:
    resp = raw_client.post(
        "/api/v1/documents/generate",
        json={"semantic": {}},
        headers={"Authorization": "Bearer sk_test_totally_bogus_key_here"},
    )
    assert resp.status_code == 401


# ---------------------------------------------------------------------------
# Health endpoint bypasses auth
# ---------------------------------------------------------------------------


def test_health_bypasses_auth(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/health")
    assert resp.status_code == 200
    assert resp.json()["status"] == "ok"


def test_health_has_no_rate_limit_headers(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/health")
    assert "X-RateLimit-Limit" not in resp.headers


# ---------------------------------------------------------------------------
# Rate limit headers present on authenticated responses
# ---------------------------------------------------------------------------


def test_rate_limit_headers_on_success(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200
    assert "X-RateLimit-Limit" in resp.headers
    assert "X-RateLimit-Remaining" in resp.headers
    assert "X-RateLimit-Reset" in resp.headers


def test_remaining_decrements(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    r1 = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    r2 = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    rem1 = int(r1.headers["X-RateLimit-Remaining"])
    rem2 = int(r2.headers["X-RateLimit-Remaining"])
    assert rem2 == rem1 - 1


# ---------------------------------------------------------------------------
# Quota enforcement
# ---------------------------------------------------------------------------


def test_free_tier_exceed_limit_returns_429(client: TestClient) -> None:
    limit = TIER_LIMITS["FREE"]["generate"]
    for _ in range(limit):
        rate_limit_store.increment(TEST_USER_ID, "generate")
    resp = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp.status_code == 429
    body = resp.json()
    assert body["error"] == "RATE_LIMIT_EXCEEDED"
    assert "Retry-After" in resp.headers
    assert resp.headers["X-RateLimit-Remaining"] == "0"


def test_under_limit_succeeds(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    limit = TIER_LIMITS["FREE"]["generate"]
    for _ in range(limit - 2):
        rate_limit_store.increment(TEST_USER_ID, "generate")
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200


# ---------------------------------------------------------------------------
# Different families tracked separately
# ---------------------------------------------------------------------------


def test_families_tracked_independently(client: TestClient) -> None:
    gen_limit = TIER_LIMITS["FREE"]["generate"]
    for _ in range(gen_limit):
        rate_limit_store.increment(TEST_USER_ID, "generate")
    resp_gen = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp_gen.status_code == 429
    resp_other = client.post("/api/v1/documents/parse", json={"semantic_json": "{}"})
    assert resp_other.status_code != 429


# ---------------------------------------------------------------------------
# Enterprise tier unlimited
# ---------------------------------------------------------------------------


def test_enterprise_tier_unlimited(client: TestClient, auth_headers: dict[str, str]) -> None:
    user = user_store.get_user(TEST_USER_ID)
    assert user is not None
    user["tier"] = "ENTERPRISE"
    for _ in range(200):
        rate_limit_store.increment(TEST_USER_ID, "generate")
    resp = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp.status_code != 429
    assert resp.headers.get("X-RateLimit-Limit") == "unlimited"


# ---------------------------------------------------------------------------
# 429 response structure
# ---------------------------------------------------------------------------


def test_429_response_body_structure(client: TestClient) -> None:
    limit = TIER_LIMITS["FREE"]["other"]
    for _ in range(limit):
        rate_limit_store.increment(TEST_USER_ID, "other")
    resp = client.post("/api/v1/documents/parse", json={"semantic_json": "{}"})
    assert resp.status_code == 429
    body = resp.json()
    assert "error" in body
    assert "detail" in body
    assert body["error"] == "RATE_LIMIT_EXCEEDED"
