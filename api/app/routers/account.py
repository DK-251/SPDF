"""Account management endpoints: API key info, rotation, and usage."""

from __future__ import annotations

from datetime import datetime, timezone, timedelta

from fastapi import APIRouter, Request

from app.errors import SpdfApiError
from app.schemas import ApiKeyResponse, ApiKeyRotateResponse, UsageResponse
from app.services.api_keys import generate_api_key
from app.services.stores import TIER_LIMITS, rate_limit_store, user_store

router = APIRouter(prefix="/api/v1/account", tags=["account"])


def _get_user(request: Request) -> dict:
    """Extract authenticated user from request state (set by middleware)."""
    user = getattr(request.state, "user", None)
    if not user:
        raise SpdfApiError(401, "UNAUTHORIZED", "Not authenticated.")
    return user


@router.get("/api-key", response_model=ApiKeyResponse)
async def get_api_key(request: Request) -> ApiKeyResponse:
    """View current API key info (prefix only, never the full key)."""
    user = _get_user(request)
    return ApiKeyResponse(
        key_prefix=user.get("api_key_prefix", ""),
        created_at=user.get("api_key_created_at", ""),
        last_used_at=user.get("last_used_at"),
    )


@router.post("/api-key/rotate", response_model=ApiKeyRotateResponse)
async def rotate_api_key(request: Request) -> ApiKeyRotateResponse:
    """Generate a new API key. Returns the full key exactly once."""
    user = _get_user(request)
    full_key, key_hash, key_prefix = generate_api_key(key_type="live")
    user_store.update_api_key(user["id"], key_hash, key_prefix)
    now = datetime.now(timezone.utc).isoformat()
    return ApiKeyRotateResponse(
        api_key=full_key,
        key_prefix=key_prefix,
        created_at=now,
    )


@router.get("/usage", response_model=UsageResponse)
async def get_usage(request: Request) -> UsageResponse:
    """View today's usage per endpoint family with tier limits."""
    user = _get_user(request)
    tier = user.get("tier", "FREE")
    today = datetime.now(timezone.utc).strftime("%Y-%m-%d")

    usage = rate_limit_store.get_usage(user["id"], today)
    limits = TIER_LIMITS.get(tier, TIER_LIMITS["FREE"])

    now = datetime.now(timezone.utc)
    midnight = now.replace(hour=0, minute=0, second=0, microsecond=0) + timedelta(days=1)

    return UsageResponse(
        user_id=user["id"],
        tier=tier,
        usage=usage,
        limits=limits,
        next_reset_at=midnight.isoformat(),
    )
