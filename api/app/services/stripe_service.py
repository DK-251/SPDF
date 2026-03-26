"""Stripe webhook event processing.

Handles subscription lifecycle events and updates in-memory stores.
Signature verification is skipped in test mode (no real Stripe account).
"""

from __future__ import annotations

import hmac
import hashlib
import os
import time
from typing import Any

from app.services.stores import user_store
from app.services.subscriptions import PLAN_FROM_PRICE, subscription_store

STRIPE_WEBHOOK_SECRET = os.environ.get("STRIPE_WEBHOOK_SECRET", "whsec_test_secret")


def verify_stripe_signature(payload: bytes, sig_header: str, secret: str | None = None) -> bool:
    """Verify Stripe webhook signature (v1 scheme).

    In test mode with default secret, always returns True to allow
    testing without real Stripe credentials.
    """
    secret = secret or STRIPE_WEBHOOK_SECRET
    if secret == "whsec_test_secret":
        return True

    parts = dict(p.split("=", 1) for p in sig_header.split(",") if "=" in p)
    timestamp = parts.get("t", "")
    v1_sig = parts.get("v1", "")

    if not timestamp or not v1_sig:
        return False

    signed_payload = f"{timestamp}.{payload.decode()}"
    expected = hmac.new(
        secret.encode(), signed_payload.encode(), hashlib.sha256
    ).hexdigest()

    return hmac.compare_digest(expected, v1_sig)


def process_webhook_event(event: dict[str, Any]) -> str:
    """Dispatch a Stripe webhook event and return a status message."""
    event_type = event.get("type", "")
    data_obj = event.get("data", {}).get("object", {})

    handler = _EVENT_HANDLERS.get(event_type)
    if handler:
        return handler(data_obj)
    return f"Unhandled event type: {event_type}"


def _handle_subscription_created(obj: dict[str, Any]) -> str:
    return _upsert_subscription(obj)


def _handle_subscription_updated(obj: dict[str, Any]) -> str:
    return _upsert_subscription(obj)


def _handle_subscription_deleted(obj: dict[str, Any]) -> str:
    stripe_sub_id = obj.get("id", "")
    result = subscription_store.get_by_stripe_sub_id(stripe_sub_id)
    if result:
        user_id, _ = result
        subscription_store.cancel(user_id)
        user_store.update_tier(user_id, "FREE")
        return f"Subscription cancelled for user {user_id}"
    return "Subscription not found"


def _handle_invoice_paid(obj: dict[str, Any]) -> str:
    stripe_sub_id = obj.get("subscription", "")
    if not stripe_sub_id:
        return "No subscription in invoice"
    result = subscription_store.get_by_stripe_sub_id(stripe_sub_id)
    if result:
        user_id, _ = result
        subscription_store.update_status(user_id, "ACTIVE")
        return f"Subscription activated for user {user_id}"
    return "Subscription not found"


def _handle_invoice_payment_failed(obj: dict[str, Any]) -> str:
    stripe_sub_id = obj.get("subscription", "")
    if not stripe_sub_id:
        return "No subscription in invoice"
    result = subscription_store.get_by_stripe_sub_id(stripe_sub_id)
    if result:
        user_id, _ = result
        subscription_store.update_status(user_id, "PAST_DUE")
        return f"Subscription past due for user {user_id}"
    return "Subscription not found"


def _upsert_subscription(obj: dict[str, Any]) -> str:
    stripe_sub_id = obj.get("id", "")
    stripe_customer_id = obj.get("customer", "")
    stripe_price_id = ""
    items = obj.get("items", {}).get("data", [])
    if items:
        stripe_price_id = items[0].get("price", {}).get("id", "")

    plan = PLAN_FROM_PRICE.get(stripe_price_id, "PRO")

    result = subscription_store.get_by_stripe_sub_id(stripe_sub_id)
    if result:
        user_id, _ = result
        subscription_store.update_status(user_id, "ACTIVE", plan=plan)
        user_store.update_tier(user_id, plan)
        return f"Subscription updated for user {user_id}"

    # New subscription — find user by stripe customer ID or first matching user
    user = None
    for u in user_store.iter_users():
        if u.get("stripe_customer_id") == stripe_customer_id:
            user = u
            break

    if not user:
        return "User not found for customer"

    subscription_store.create(
        user_id=user["id"],
        plan=plan,
        stripe_subscription_id=stripe_sub_id,
        stripe_customer_id=stripe_customer_id,
        stripe_price_id=stripe_price_id,
    )
    user_store.update_tier(user["id"], plan)
    return f"Subscription created for user {user['id']}"


_EVENT_HANDLERS: dict[str, Any] = {
    "customer.subscription.created": _handle_subscription_created,
    "customer.subscription.updated": _handle_subscription_updated,
    "customer.subscription.deleted": _handle_subscription_deleted,
    "invoice.paid": _handle_invoice_paid,
    "invoice.payment_failed": _handle_invoice_payment_failed,
}
