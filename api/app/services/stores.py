"""In-memory user and rate-limit stores.

Dict-backed implementations with the same interface future DB/Redis
backends will expose. Module-level singletons are imported by middleware
and routers.
"""

from __future__ import annotations

from datetime import datetime, timezone

TIER_LIMITS: dict[str, dict[str, int]] = {
    "FREE": {
        "convert": 10,
        "generate": 50,
        "extract": 100,
        "sign": 50,
        "other": 500,
    },
    "PRO": {
        "convert": 1_000,
        "generate": 5_000,
        "extract": 10_000,
        "sign": 2_000,
        "other": 50_000,
    },
    "TEAM": {
        "convert": 10_000,
        "generate": 50_000,
        "extract": 100_000,
        "sign": 20_000,
        "other": 500_000,
    },
    "ENTERPRISE": {},  # unlimited — skip check
}

ENDPOINT_FAMILIES: dict[str, str] = {
    "/api/v1/documents/generate": "generate",
    "/api/v1/documents/extract": "extract",
    "/api/v1/documents/validate": "other",
    "/api/v1/documents/render": "other",
    "/api/v1/documents/parse": "other",
    "/api/v1/billing/checkout": "other",
    "/api/v1/billing/portal": "other",
    "/api/v1/billing/subscription": "other",
    "/api/v1/documents/sign": "sign",
    "/api/v1/documents/verify": "other",
    "/api/v1/documents/transition": "other",
    "/api/v1/documents/diff": "other",
    "/api/v1/documents/redact": "other",
    "/api/v1/documents/redactions": "other",
    "/api/v1/documents/verify-redaction": "other",
}

# Paths that bypass auth and rate limiting entirely
PUBLIC_PATHS: set[str] = {"/api/v1/health", "/api/v1/webhooks/stripe"}


def _today_utc() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%d")


class UserStore:
    """Dict-backed user store keyed by user_id."""

    def __init__(self) -> None:
        self._users: dict[str, dict] = {}
        self._key_index: dict[str, str] = {}  # key_prefix -> user_id

    def add_user(
        self,
        user_id: str,
        tier: str = "FREE",
        api_key_hash: str = "",
        api_key_prefix: str = "",
        email: str = "",
    ) -> dict:
        now = datetime.now(timezone.utc).isoformat()
        user = {
            "id": user_id,
            "tier": tier,
            "email": email,
            "api_key_hash": api_key_hash,
            "api_key_prefix": api_key_prefix,
            "api_key_created_at": now,
            "last_used_at": None,
            "created_at": now,
        }
        self._users[user_id] = user
        if api_key_prefix:
            self._key_index[api_key_prefix] = user_id
        return user

    def get_user(self, user_id: str) -> dict | None:
        return self._users.get(user_id)

    def find_user_by_key_prefix(self, prefix: str) -> dict | None:
        uid = self._key_index.get(prefix)
        if uid:
            return self._users.get(uid)
        return None

    def find_by_email(self, email: str) -> dict | None:
        for u in self._users.values():
            if u.get("email") == email:
                return u
        return None

    def update_tier(self, user_id: str, tier: str) -> None:
        user = self._users.get(user_id)
        if user:
            user["tier"] = tier

    def iter_users(self) -> list[dict]:
        return list(self._users.values())

    def update_api_key(
        self, user_id: str, key_hash: str, key_prefix: str
    ) -> None:
        user = self._users.get(user_id)
        if not user:
            return
        old_prefix = user.get("api_key_prefix", "")
        if old_prefix and old_prefix in self._key_index:
            del self._key_index[old_prefix]
        user["api_key_hash"] = key_hash
        user["api_key_prefix"] = key_prefix
        user["api_key_created_at"] = datetime.now(timezone.utc).isoformat()
        self._key_index[key_prefix] = user_id

    def touch_last_used(self, user_id: str) -> None:
        user = self._users.get(user_id)
        if user:
            user["last_used_at"] = datetime.now(timezone.utc).isoformat()

    def reset(self) -> None:
        self._users.clear()
        self._key_index.clear()


class RateLimitStore:
    """Dict-backed rate limiter keyed by '{user_id}:{family}:{date}'."""

    def __init__(self) -> None:
        self._counts: dict[str, int] = {}

    def _key(self, user_id: str, family: str, date_str: str) -> str:
        return f"{user_id}:{family}:{date_str}"

    def increment(self, user_id: str, family: str, date_str: str | None = None) -> int:
        date_str = date_str or _today_utc()
        k = self._key(user_id, family, date_str)
        self._counts[k] = self._counts.get(k, 0) + 1
        return self._counts[k]

    def get_count(self, user_id: str, family: str, date_str: str | None = None) -> int:
        date_str = date_str or _today_utc()
        return self._counts.get(self._key(user_id, family, date_str), 0)

    def get_usage(self, user_id: str, date_str: str | None = None) -> dict[str, int]:
        date_str = date_str or _today_utc()
        prefix = f"{user_id}:"
        suffix = f":{date_str}"
        result: dict[str, int] = {}
        for k, v in self._counts.items():
            if k.startswith(prefix) and k.endswith(suffix):
                family = k[len(prefix):-len(suffix)]
                result[family] = v
        return result

    def reset(self) -> None:
        self._counts.clear()


# Module-level singletons
user_store = UserStore()
rate_limit_store = RateLimitStore()
