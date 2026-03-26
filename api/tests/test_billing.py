"""Tests for billing endpoints: subscription, checkout, portal."""

from __future__ import annotations

from fastapi.testclient import TestClient

from app.services.api_keys import TEST_USER_ID
from app.services.stores import TIER_LIMITS, rate_limit_store, user_store
from app.services.subscriptions import subscription_store


# ---------------------------------------------------------------------------
# GET /api/v1/billing/subscription
# ---------------------------------------------------------------------------


def test_subscription_returns_free_by_default(client: TestClient) -> None:
    resp = client.get("/api/v1/billing/subscription")
    assert resp.status_code == 200
    body = resp.json()
    assert body["plan"] == "FREE"
    assert body["status"] == "ACTIVE"


def test_subscription_returns_active_plan(client: TestClient) -> None:
    subscription_store.create(TEST_USER_ID, "PRO", stripe_subscription_id="sub_123")
    resp = client.get("/api/v1/billing/subscription")
    body = resp.json()
    assert body["plan"] == "PRO"
    assert body["status"] == "ACTIVE"
    assert body["current_period_start"] is not None
    assert body["current_period_end"] is not None


def test_subscription_401_without_auth(raw_client: TestClient) -> None:
    resp = raw_client.get("/api/v1/billing/subscription")
    assert resp.status_code == 401


# ---------------------------------------------------------------------------
# Subscription lifecycle + tier enforcement
# ---------------------------------------------------------------------------


def test_subscription_creation_updates_tier(client: TestClient) -> None:
    subscription_store.create(TEST_USER_ID, "PRO")
    user_store.update_tier(TEST_USER_ID, "PRO")
    resp = client.get("/api/v1/account/usage")
    assert resp.json()["tier"] == "PRO"
    assert resp.json()["limits"]["generate"] == TIER_LIMITS["PRO"]["generate"]


def test_subscription_cancellation_reverts_to_free(client: TestClient) -> None:
    subscription_store.create(TEST_USER_ID, "PRO")
    user_store.update_tier(TEST_USER_ID, "PRO")
    subscription_store.cancel(TEST_USER_ID)
    user_store.update_tier(TEST_USER_ID, "FREE")
    resp = client.get("/api/v1/billing/subscription")
    assert resp.json()["status"] == "CANCELLED"
    usage_resp = client.get("/api/v1/account/usage")
    assert usage_resp.json()["tier"] == "FREE"


def test_tier_upgrade_increases_rate_limits(client: TestClient) -> None:
    free_limit = TIER_LIMITS["FREE"]["generate"]
    for _ in range(free_limit):
        rate_limit_store.increment(TEST_USER_ID, "generate")
    resp_before = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp_before.status_code == 429
    user_store.update_tier(TEST_USER_ID, "PRO")
    resp_after = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp_after.status_code != 429


# ---------------------------------------------------------------------------
# POST /api/v1/billing/checkout
# ---------------------------------------------------------------------------


def test_checkout_returns_url(client: TestClient) -> None:
    resp = client.post("/api/v1/billing/checkout", json={"plan": "PRO"})
    assert resp.status_code == 200
    body = resp.json()
    assert "checkout_url" in body
    assert "stripe.com" in body["checkout_url"]


def test_checkout_invalid_plan_returns_400(client: TestClient) -> None:
    resp = client.post("/api/v1/billing/checkout", json={"plan": "INVALID"})
    assert resp.status_code == 400


def test_checkout_401_without_auth(raw_client: TestClient) -> None:
    resp = raw_client.post("/api/v1/billing/checkout", json={"plan": "PRO"})
    assert resp.status_code == 401


# ---------------------------------------------------------------------------
# POST /api/v1/billing/portal
# ---------------------------------------------------------------------------


def test_portal_returns_url(client: TestClient) -> None:
    resp = client.post("/api/v1/billing/portal")
    assert resp.status_code == 200
    body = resp.json()
    assert "portal_url" in body
    assert "stripe.com" in body["portal_url"]


def test_portal_401_without_auth(raw_client: TestClient) -> None:
    resp = raw_client.post("/api/v1/billing/portal")
    assert resp.status_code == 401
