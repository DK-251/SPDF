"""JWT token decoding and verification.

Supports configurable secret and issuer for Clerk integration.
Used alongside API key auth in the rate-limit middleware.
"""

from __future__ import annotations

import os
from typing import Any

import jwt

# Configurable via environment; defaults for local development/testing.
JWT_SECRET = os.environ.get("JWT_SECRET", "spdf-dev-jwt-secret-do-not-use-in-production")
JWT_ALGORITHM = os.environ.get("JWT_ALGORITHM", "HS256")
JWT_ISSUER = os.environ.get("JWT_ISSUER", "spdf-dev")

# Test constants
TEST_JWT_SECRET = "spdf-test-jwt-secret"
TEST_JWT_ISSUER = "spdf-test"


def decode_jwt(
    token: str,
    secret: str | None = None,
    issuer: str | None = None,
    algorithms: list[str] | None = None,
) -> dict[str, Any]:
    """Decode and verify a JWT token.

    Returns the decoded claims dict.
    Raises jwt.InvalidTokenError (or subclass) on any failure.
    """
    secret = secret or JWT_SECRET
    issuer = issuer or JWT_ISSUER
    algorithms = algorithms or [JWT_ALGORITHM]

    return jwt.decode(
        token,
        secret,
        algorithms=algorithms,
        issuer=issuer,
        options={"require": ["sub", "iss", "exp", "iat"]},
    )


def create_test_token(
    sub: str,
    secret: str | None = None,
    issuer: str | None = None,
    extra_claims: dict[str, Any] | None = None,
    exp_delta_seconds: int = 3600,
) -> str:
    """Create a signed JWT for testing purposes only."""
    import time

    now = int(time.time())
    payload: dict[str, Any] = {
        "sub": sub,
        "iss": issuer or TEST_JWT_ISSUER,
        "iat": now,
        "exp": now + exp_delta_seconds,
    }
    if extra_claims:
        payload.update(extra_claims)

    return jwt.encode(payload, secret or TEST_JWT_SECRET, algorithm="HS256")
