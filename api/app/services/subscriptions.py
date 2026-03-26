"""In-memory subscription store.

Tracks user subscriptions with plan, status, and Stripe metadata.
Same interface future PostgreSQL implementation will expose.
"""

from __future__ import annotations

import uuid
from datetime import datetime, timezone, timedelta


VALID_PLANS = {"FREE", "PRO", "TEAM", "ENTERPRISE"}
VALID_STATUSES = {"ACTIVE", "TRIALING", "PAST_DUE", "CANCELLED"}

PLAN_FROM_PRICE: dict[str, str] = {
    "price_pro": "PRO",
    "price_team": "TEAM",
}


class SubscriptionStore:
    """Dict-backed subscription store keyed by user_id."""

    def __init__(self) -> None:
        self._subs: dict[str, dict] = {}

    def create(
        self,
        user_id: str,
        plan: str,
        stripe_subscription_id: str = "",
        stripe_customer_id: str = "",
        stripe_price_id: str = "",
    ) -> dict:
        now = datetime.now(timezone.utc)
        sub = {
            "id": str(uuid.uuid4()),
            "user_id": user_id,
            "plan": plan,
            "status": "ACTIVE",
            "stripe_subscription_id": stripe_subscription_id,
            "stripe_customer_id": stripe_customer_id,
            "stripe_price_id": stripe_price_id,
            "current_period_start": now.isoformat(),
            "current_period_end": (now + timedelta(days=30)).isoformat(),
            "created_at": now.isoformat(),
            "updated_at": now.isoformat(),
        }
        self._subs[user_id] = sub
        return sub

    def get(self, user_id: str) -> dict | None:
        return self._subs.get(user_id)

    def get_by_stripe_sub_id(self, stripe_subscription_id: str) -> tuple[str, dict] | None:
        for uid, sub in self._subs.items():
            if sub["stripe_subscription_id"] == stripe_subscription_id:
                return uid, sub
        return None

    def update_status(self, user_id: str, status: str, plan: str | None = None) -> dict | None:
        sub = self._subs.get(user_id)
        if not sub:
            return None
        sub["status"] = status
        if plan:
            sub["plan"] = plan
        sub["updated_at"] = datetime.now(timezone.utc).isoformat()
        return sub

    def cancel(self, user_id: str) -> dict | None:
        sub = self._subs.get(user_id)
        if not sub:
            return None
        sub["status"] = "CANCELLED"
        sub["updated_at"] = datetime.now(timezone.utc).isoformat()
        return sub

    def reset(self) -> None:
        self._subs.clear()


subscription_store = SubscriptionStore()
