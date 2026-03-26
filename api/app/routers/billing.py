"""Billing endpoints: subscription status, checkout, and portal."""

from __future__ import annotations

from fastapi import APIRouter, Request

from app.errors import SpdfApiError
from app.schemas import (
    CheckoutRequest,
    CheckoutResponse,
    PortalResponse,
    SubscriptionResponse,
)
from app.services.subscriptions import subscription_store

router = APIRouter(prefix="/api/v1/billing", tags=["billing"])


def _get_user(request: Request) -> dict:
    user = getattr(request.state, "user", None)
    if not user:
        raise SpdfApiError(401, "UNAUTHORIZED", "Not authenticated.")
    return user


@router.get("/subscription", response_model=SubscriptionResponse)
async def get_subscription(request: Request) -> SubscriptionResponse:
    """Get current subscription status."""
    user = _get_user(request)
    sub = subscription_store.get(user["id"])
    if not sub:
        return SubscriptionResponse(
            plan="FREE",
            status="ACTIVE",
        )
    return SubscriptionResponse(
        plan=sub["plan"],
        status=sub["status"],
        current_period_start=sub.get("current_period_start"),
        current_period_end=sub.get("current_period_end"),
        created_at=sub.get("created_at"),
    )


@router.post("/checkout", response_model=CheckoutResponse)
async def create_checkout(request: Request, body: CheckoutRequest) -> CheckoutResponse:
    """Create a checkout session (stub — returns mock URL)."""
    user = _get_user(request)
    if body.plan not in ("PRO", "TEAM"):
        raise SpdfApiError(400, "INVALID_PLAN", "Plan must be PRO or TEAM.")
    return CheckoutResponse(
        checkout_url=f"https://checkout.stripe.com/stub/{user['id']}/{body.plan.lower()}"
    )


@router.post("/portal", response_model=PortalResponse)
async def create_portal(request: Request) -> PortalResponse:
    """Create a billing portal session (stub — returns mock URL)."""
    user = _get_user(request)
    return PortalResponse(
        portal_url=f"https://billing.stripe.com/stub/{user['id']}/portal"
    )
