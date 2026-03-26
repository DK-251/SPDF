"""Tests for Stripe webhook handler: event dispatch, tier mutations."""

from __future__ import annotations

import json

from fastapi.testclient import TestClient

from app.services.api_keys import TEST_USER_ID
from app.services.stores import user_store
from app.services.subscriptions import subscription_store


def _webhook_event(event_type: str, obj: dict) -> dict:
    return {"type": event_type, "data": {"object": obj}}


def _sub_object(
    sub_id: str = "sub_test_123",
    customer: str = "cus_test_456",
    price_id: str = "price_pro",
) -> dict:
    return {
        "id": sub_id,
        "customer": customer,
        "items": {"data": [{"price": {"id": price_id}}]},
    }


def _post_webhook(client: TestClient, event: dict) -> object:
    return client.post(
        "/api/v1/webhooks/stripe",
        content=json.dumps(event),
        headers={"content-type": "application/json", "stripe-signature": "t=123,v1=abc"},
    )


# ---------------------------------------------------------------------------
# Subscription lifecycle events
# ---------------------------------------------------------------------------


def test_subscription_created_updates_tier(raw_client: TestClient) -> None:
    user_store.get_user(TEST_USER_ID)["stripe_customer_id"] = "cus_test_456"
    event = _webhook_event("customer.subscription.created", _sub_object())
    resp = _post_webhook(raw_client, event)
    assert resp.status_code == 200
    assert user_store.get_user(TEST_USER_ID)["tier"] == "PRO"


def test_subscription_updated_changes_plan(raw_client: TestClient) -> None:
    user_store.get_user(TEST_USER_ID)["stripe_customer_id"] = "cus_test_456"
    subscription_store.create(
        TEST_USER_ID, "PRO", stripe_subscription_id="sub_test_123"
    )
    event = _webhook_event(
        "customer.subscription.updated",
        _sub_object(price_id="price_team"),
    )
    resp = _post_webhook(raw_client, event)
    assert resp.status_code == 200
    assert user_store.get_user(TEST_USER_ID)["tier"] == "TEAM"


def test_subscription_deleted_reverts_to_free(raw_client: TestClient) -> None:
    subscription_store.create(
        TEST_USER_ID, "PRO", stripe_subscription_id="sub_test_123"
    )
    user_store.update_tier(TEST_USER_ID, "PRO")
    event = _webhook_event("customer.subscription.deleted", {"id": "sub_test_123"})
    resp = _post_webhook(raw_client, event)
    assert resp.status_code == 200
    assert user_store.get_user(TEST_USER_ID)["tier"] == "FREE"
    assert subscription_store.get(TEST_USER_ID)["status"] == "CANCELLED"


# ---------------------------------------------------------------------------
# Invoice events
# ---------------------------------------------------------------------------


def test_invoice_paid_sets_active(raw_client: TestClient) -> None:
    subscription_store.create(
        TEST_USER_ID, "PRO", stripe_subscription_id="sub_test_123"
    )
    subscription_store.update_status(TEST_USER_ID, "PAST_DUE")
    event = _webhook_event("invoice.paid", {"subscription": "sub_test_123"})
    resp = _post_webhook(raw_client, event)
    assert resp.status_code == 200
    assert subscription_store.get(TEST_USER_ID)["status"] == "ACTIVE"


def test_invoice_payment_failed_sets_past_due(raw_client: TestClient) -> None:
    subscription_store.create(
        TEST_USER_ID, "PRO", stripe_subscription_id="sub_test_123"
    )
    event = _webhook_event("invoice.payment_failed", {"subscription": "sub_test_123"})
    resp = _post_webhook(raw_client, event)
    assert resp.status_code == 200
    assert subscription_store.get(TEST_USER_ID)["status"] == "PAST_DUE"


# ---------------------------------------------------------------------------
# Edge cases
# ---------------------------------------------------------------------------


def test_unknown_event_type_returns_200(raw_client: TestClient) -> None:
    event = _webhook_event("charge.succeeded", {"id": "ch_123"})
    resp = _post_webhook(raw_client, event)
    assert resp.status_code == 200


def test_invalid_json_returns_400(raw_client: TestClient) -> None:
    resp = raw_client.post(
        "/api/v1/webhooks/stripe",
        content=b"not json",
        headers={"content-type": "application/json", "stripe-signature": "t=1,v1=a"},
    )
    assert resp.status_code == 400


def test_missing_event_type_returns_400(raw_client: TestClient) -> None:
    resp = raw_client.post(
        "/api/v1/webhooks/stripe",
        content=json.dumps({"data": {}}).encode(),
        headers={"content-type": "application/json", "stripe-signature": "t=1,v1=a"},
    )
    assert resp.status_code == 400


def test_webhook_does_not_require_bearer_auth(raw_client: TestClient) -> None:
    event = _webhook_event("charge.succeeded", {})
    resp = _post_webhook(raw_client, event)
    assert resp.status_code == 200


def test_duplicate_event_is_idempotent(raw_client: TestClient) -> None:
    subscription_store.create(
        TEST_USER_ID, "PRO", stripe_subscription_id="sub_test_123"
    )
    user_store.update_tier(TEST_USER_ID, "PRO")
    event = _webhook_event("customer.subscription.deleted", {"id": "sub_test_123"})
    _post_webhook(raw_client, event)
    resp2 = _post_webhook(raw_client, event)
    assert resp2.status_code == 200
    assert user_store.get_user(TEST_USER_ID)["tier"] == "FREE"
