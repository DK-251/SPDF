"""Tests for document signing, verification, and state transition endpoints."""

from __future__ import annotations

import json
from typing import Any

import pytest
from fastapi.testclient import TestClient


def _make_review_spdf(client: TestClient, sample_semantic: dict[str, Any]) -> bytes:
    """Generate an SPDF in REVIEW state via generate + transition."""
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200
    draft_bytes = resp.content

    resp = client.post(
        "/api/v1/documents/transition",
        files={"file": ("doc.spdf", draft_bytes, "application/octet-stream")},
        data={"target_state": "REVIEW"},
    )
    assert resp.status_code == 200
    return resp.content


# ---------- Sign endpoint ----------


def test_sign_review_document(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    review_bytes = _make_review_spdf(client, sample_semantic)
    resp = client.post(
        "/api/v1/documents/sign",
        files={"file": ("doc.spdf", review_bytes, "application/octet-stream")},
        data={"signer_name": "Alice", "signer_email": "alice@spdf.dev"},
    )
    assert resp.status_code == 200
    assert len(resp.content) > 0


def test_sign_draft_document_fails(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/sign",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
        data={"signer_name": "Alice", "signer_email": "alice@spdf.dev"},
    )
    assert resp.status_code in (400, 422, 500)


def test_sign_missing_fields(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/sign",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    assert resp.status_code == 422


# ---------- Verify endpoint ----------


def test_verify_signed_document(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    review_bytes = _make_review_spdf(client, sample_semantic)
    sign_resp = client.post(
        "/api/v1/documents/sign",
        files={"file": ("doc.spdf", review_bytes, "application/octet-stream")},
        data={"signer_name": "Alice", "signer_email": "alice@spdf.dev"},
    )
    assert sign_resp.status_code == 200

    verify_resp = client.post(
        "/api/v1/documents/verify",
        files={"file": ("signed.spdf", sign_resp.content, "application/octet-stream")},
    )
    assert verify_resp.status_code == 200
    report = verify_resp.json()
    assert report["valid"] is True
    assert report["tamper_detected"] is False
    assert report["signature_count"] == 1


def test_verify_unsigned_document(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/verify",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    assert resp.status_code == 200
    report = resp.json()
    assert report["valid"] is False
    assert report["signature_count"] == 0


def test_verify_returns_signer_details(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    review_bytes = _make_review_spdf(client, sample_semantic)
    sign_resp = client.post(
        "/api/v1/documents/sign",
        files={"file": ("doc.spdf", review_bytes, "application/octet-stream")},
        data={"signer_name": "Bob Smith", "signer_email": "bob@example.com"},
    )
    verify_resp = client.post(
        "/api/v1/documents/verify",
        files={"file": ("signed.spdf", sign_resp.content, "application/octet-stream")},
    )
    report = verify_resp.json()
    sig = report["signatures"][0]
    assert sig["signer_name"] == "Bob Smith"
    assert sig["signer_email"] == "bob@example.com"
    assert sig["valid"] is True


# ---------- Transition endpoint ----------


def test_transition_draft_to_review(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/transition",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
        data={"target_state": "REVIEW"},
    )
    assert resp.status_code == 200
    assert len(resp.content) > 0


def test_transition_invalid_state_fails(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/transition",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
        data={"target_state": "SIGNED"},
    )
    assert resp.status_code in (400, 422, 500)


def test_transition_review_back_to_draft(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    review_bytes = _make_review_spdf(client, sample_semantic)
    resp = client.post(
        "/api/v1/documents/transition",
        files={"file": ("doc.spdf", review_bytes, "application/octet-stream")},
        data={"target_state": "DRAFT"},
    )
    assert resp.status_code == 200


def test_transition_bad_state_string(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/transition",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
        data={"target_state": "NONEXISTENT"},
    )
    assert resp.status_code in (400, 422, 500)


def test_sign_then_verify_round_trip(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    """Full round-trip: generate -> transition to review -> sign -> verify."""
    if not engine_available:
        pytest.skip("spdf_native not available")
    review_bytes = _make_review_spdf(client, sample_semantic)

    sign_resp = client.post(
        "/api/v1/documents/sign",
        files={"file": ("doc.spdf", review_bytes, "application/octet-stream")},
        data={"signer_name": "Deepak", "signer_email": "deepak@spdf.dev"},
    )
    assert sign_resp.status_code == 200

    verify_resp = client.post(
        "/api/v1/documents/verify",
        files={"file": ("signed.spdf", sign_resp.content, "application/octet-stream")},
    )
    assert verify_resp.status_code == 200
    report = verify_resp.json()
    assert report["valid"] is True
    assert report["signature_count"] == 1
    assert report["signatures"][0]["signer_name"] == "Deepak"
