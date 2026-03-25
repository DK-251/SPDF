"""API key generation, hashing, and verification."""

from __future__ import annotations

import secrets

from passlib.hash import bcrypt

# Key format: sk_{type}_{26 chars}
_KEY_LENGTH = 26


def generate_api_key(key_type: str = "live") -> tuple[str, str, str]:
    """Generate a new API key.

    Returns (full_key, key_hash, key_prefix).
    """
    token = secrets.token_urlsafe(_KEY_LENGTH)[:_KEY_LENGTH]
    full_key = f"sk_{key_type}_{token}"
    key_hash = bcrypt.using(rounds=12).hash(full_key)
    key_prefix = full_key[:16]
    return full_key, key_hash, key_prefix


def hash_api_key(plain_key: str) -> str:
    """Hash an API key with bcrypt (rounds=12)."""
    return bcrypt.using(rounds=12).hash(plain_key)


def verify_api_key(plain_key: str, key_hash: str) -> bool:
    """Verify a plain API key against its bcrypt hash."""
    try:
        return bcrypt.verify(plain_key, key_hash)
    except Exception:
        return False


# Well-known test key for development and testing.
# Full key is only used in test fixtures — never in production.
TEST_API_KEY = "sk_test_dev_000000000000000000"
TEST_API_KEY_PREFIX = TEST_API_KEY[:16]
TEST_USER_ID = "test-user-00000000-0000-0000-0000-000000000000"


def seed_test_user() -> None:
    """Seed the in-memory store with a test user and known API key."""
    from app.services.stores import user_store

    if user_store.get_user(TEST_USER_ID):
        return
    key_hash = bcrypt.using(rounds=4).hash(TEST_API_KEY)
    user_store.add_user(
        user_id=TEST_USER_ID,
        tier="FREE",
        api_key_hash=key_hash,
        api_key_prefix=TEST_API_KEY_PREFIX,
    )
