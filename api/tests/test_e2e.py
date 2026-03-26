"""End-to-end integration tests exercising multi-step API flows."""

from __future__ import annotations

import json
from typing import Any

import pytest
from fastapi.testclient import TestClient

from app.services.api_keys import TEST_API_KEY, TEST_USER_ID
from app.services.stores import TIER_LIMITS, rate_limit_store, user_store
from app.services.subscriptions import subscription_store


def _engine_check() -> bool:
    try:
        import spdf_native  # noqa: F401

        return True
    except ImportError:
        return False


needs_engine = pytest.mark.skipif(
    not _engine_check(),
    reason="spdf_native not available",
)


# ---------------------------------------------------------------------------
# Full document flow: generate -> validate -> extract
# ---------------------------------------------------------------------------


@needs_engine
def test_full_document_flow(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    gen_resp = client.post(
        "/api/v1/documents/generate", json={"semantic": sample_semantic}
    )
    assert gen_resp.status_code == 200
    spdf_bytes = gen_resp.content
    assert spdf_bytes[:4] == b"PK\x03\x04"

    val_resp = client.post(
        "/api/v1/documents/validate",
        files={"file": ("test.spdf", spdf_bytes, "application/octet-stream")},
    )
    assert val_resp.status_code == 200
    assert val_resp.json()["valid"] is True

    ext_resp = client.post(
        "/api/v1/documents/extract",
        files={"file": ("test.spdf", spdf_bytes, "application/octet-stream")},
    )
    assert ext_resp.status_code == 200
    assert ext_resp.json()["invoice_number"] == "INV-2026-200"
    assert ext_resp.json()["total"] == "132750.00"


# ---------------------------------------------------------------------------
# Generate -> render to PDF
# ---------------------------------------------------------------------------


@needs_engine
def test_generate_then_render(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    gen_resp = client.post(
        "/api/v1/documents/generate", json={"semantic": sample_semantic}
    )
    assert gen_resp.status_code == 200

    render_resp = client.post(
        "/api/v1/documents/render",
        files={"file": ("test.spdf", gen_resp.content, "application/octet-stream")},
    )
    assert render_resp.status_code == 200
    assert render_resp.content[:5] == b"%PDF-"


# ---------------------------------------------------------------------------
# Rate limiting across families
# ---------------------------------------------------------------------------


def test_rate_limits_across_families(client: TestClient) -> None:
    gen_limit = TIER_LIMITS["FREE"]["generate"]
    for _ in range(gen_limit):
        rate_limit_store.increment(TEST_USER_ID, "generate")

    gen_resp = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert gen_resp.status_code == 429

    other_resp = client.post("/api/v1/documents/parse", json={"semantic_json": "{}"})
    assert other_resp.status_code != 429


# ---------------------------------------------------------------------------
# Tier upgrade -> higher limits
# ---------------------------------------------------------------------------


def test_tier_upgrade_unlocks_requests(client: TestClient) -> None:
    free_limit = TIER_LIMITS["FREE"]["generate"]
    for _ in range(free_limit):
        rate_limit_store.increment(TEST_USER_ID, "generate")

    resp_429 = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp_429.status_code == 429

    user_store.update_tier(TEST_USER_ID, "PRO")
    resp_ok = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp_ok.status_code != 429


# ---------------------------------------------------------------------------
# API key rotation flow
# ---------------------------------------------------------------------------


def test_key_rotation_full_flow(client: TestClient, raw_client: TestClient) -> None:
    rotate_resp = client.post("/api/v1/account/api-key/rotate")
    assert rotate_resp.status_code == 200
    new_key = rotate_resp.json()["api_key"]

    old_resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {TEST_API_KEY}"},
    )
    assert old_resp.status_code == 401

    new_resp = raw_client.get(
        "/api/v1/account/api-key",
        headers={"Authorization": f"Bearer {new_key}"},
    )
    assert new_resp.status_code == 200


# ---------------------------------------------------------------------------
# Usage tracking accuracy
# ---------------------------------------------------------------------------


@needs_engine
def test_usage_tracking_after_operations(
    client: TestClient, sample_semantic: dict[str, Any]
) -> None:
    client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})

    usage_resp = client.get("/api/v1/account/usage")
    usage = usage_resp.json()["usage"]
    assert usage.get("generate", 0) >= 2


# ---------------------------------------------------------------------------
# Webhook tier change -> immediate rate limit effect
# ---------------------------------------------------------------------------


def test_webhook_tier_change_affects_rate_limits(
    client: TestClient, raw_client: TestClient
) -> None:
    free_limit = TIER_LIMITS["FREE"]["generate"]
    for _ in range(free_limit):
        rate_limit_store.increment(TEST_USER_ID, "generate")

    assert client.post("/api/v1/documents/generate", json={"semantic": {}}).status_code == 429

    subscription_store.create(
        TEST_USER_ID, "PRO", stripe_subscription_id="sub_e2e_123"
    )
    user_store.update_tier(TEST_USER_ID, "PRO")

    resp = client.post("/api/v1/documents/generate", json={"semantic": {}})
    assert resp.status_code != 429


# ---------------------------------------------------------------------------
# JWT + template combined flow
# ---------------------------------------------------------------------------


def test_jwt_auth_template_crud(jwt_client: TestClient) -> None:
    create_resp = jwt_client.post(
        "/api/v1/templates", json={"name": "JWT Template"}
    )
    assert create_resp.status_code == 201
    tid = create_resp.json()["id"]

    get_resp = jwt_client.get(f"/api/v1/templates/{tid}")
    assert get_resp.status_code == 200
    assert get_resp.json()["name"] == "JWT Template"

    del_resp = jwt_client.delete(f"/api/v1/templates/{tid}")
    assert del_resp.status_code == 204
