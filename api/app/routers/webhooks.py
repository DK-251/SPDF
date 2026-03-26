"""Webhook endpoints for external service integrations."""

from __future__ import annotations

import json

from fastapi import APIRouter, Request

from app.errors import SpdfApiError
from app.services.stripe_service import process_webhook_event, verify_stripe_signature

router = APIRouter(prefix="/api/v1/webhooks", tags=["webhooks"])


@router.post("/stripe")
async def stripe_webhook(request: Request) -> dict:
    """Handle Stripe webhook events.

    This endpoint is public (no Bearer auth) — Stripe authenticates
    via the Stripe-Signature header.
    """
    body = await request.body()
    sig_header = request.headers.get("stripe-signature", "")

    if not verify_stripe_signature(body, sig_header):
        raise SpdfApiError(400, "INVALID_SIGNATURE", "Stripe signature verification failed.")

    try:
        event = json.loads(body)
    except (json.JSONDecodeError, ValueError):
        raise SpdfApiError(400, "INVALID_PAYLOAD", "Invalid JSON in webhook body.")

    if not isinstance(event, dict) or "type" not in event:
        raise SpdfApiError(400, "INVALID_PAYLOAD", "Missing event type in webhook body.")

    process_webhook_event(event)
    return {"status": "ok"}
