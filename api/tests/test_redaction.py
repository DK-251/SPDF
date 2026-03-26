"""Tests for document redaction endpoints."""

from __future__ import annotations

import json
from typing import Any

import pytest
from fastapi.testclient import TestClient


def _get_element_eid(
    client: TestClient, sample_semantic: dict[str, Any], element_type: str
) -> tuple[bytes, str]:
    """Generate SPDF and find the EID of the first element of the given type."""
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200
    spdf_bytes = resp.content

    # Find the EID from the semantic fixture
    for page in sample_semantic["pages"]:
        for el in page["elements"]:
            if el.get("element_type") == element_type:
                return spdf_bytes, el["eid"]
    raise ValueError(f"No {element_type} found in fixture")


# ---------- Redact endpoint ----------


def test_redact_element(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    spdf_bytes, eid = _get_element_eid(client, sample_semantic, "Paragraph")
    # Use Heading instead since we have one
    spdf_bytes, eid = _get_element_eid(client, sample_semantic, "Heading")
    resp = client.post(
        "/api/v1/documents/redact",
        files={"file": ("doc.spdf", spdf_bytes, "application/octet-stream")},
        data={"target_eid": eid, "reason": "Contains PII"},
    )
    assert resp.status_code == 200
    assert len(resp.content) > 0


def test_redact_nonexistent_eid(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/redact",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
        data={"target_eid": "el-nonexistent-0000", "reason": "test"},
    )
    assert resp.status_code in (400, 422, 500)


def test_redact_missing_fields(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/redact",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    assert resp.status_code == 422


# ---------- List redactions ----------


def test_list_redactions_empty(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/redactions",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
    )
    assert resp.status_code == 200
    data = resp.json()
    assert data["redactions"] == []


def test_list_redactions_after_redact(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    spdf_bytes, eid = _get_element_eid(client, sample_semantic, "Heading")
    redact_resp = client.post(
        "/api/v1/documents/redact",
        files={"file": ("doc.spdf", spdf_bytes, "application/octet-stream")},
        data={"target_eid": eid, "reason": "Classified"},
    )
    assert redact_resp.status_code == 200

    list_resp = client.post(
        "/api/v1/documents/redactions",
        files={"file": ("r.spdf", redact_resp.content, "application/octet-stream")},
    )
    assert list_resp.status_code == 200
    data = list_resp.json()
    assert len(data["redactions"]) == 1
    assert data["redactions"][0]["redacted_eid"] == eid
    assert data["redactions"][0]["reason"] == "Classified"


# ---------- Verify redaction ----------


def test_verify_redaction_found(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    spdf_bytes, eid = _get_element_eid(client, sample_semantic, "Heading")
    redact_resp = client.post(
        "/api/v1/documents/redact",
        files={"file": ("doc.spdf", spdf_bytes, "application/octet-stream")},
        data={"target_eid": eid, "reason": "PII"},
    )
    assert redact_resp.status_code == 200

    verify_resp = client.post(
        "/api/v1/documents/verify-redaction",
        files={"file": ("r.spdf", redact_resp.content, "application/octet-stream")},
        data={"redaction_eid": eid},
    )
    assert verify_resp.status_code == 200
    data = verify_resp.json()
    assert data["found"] is True
    assert data["proof_hash"] != ""


def test_verify_redaction_not_found(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/verify-redaction",
        files={"file": ("doc.spdf", sample_spdf_bytes, "application/octet-stream")},
        data={"redaction_eid": "el-nonexistent-0000"},
    )
    assert resp.status_code == 200
    data = resp.json()
    assert data["found"] is False


# ---------- Round trip ----------


def test_redact_then_list_then_verify(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    """Full round-trip: redact -> list -> verify."""
    if not engine_available:
        pytest.skip("spdf_native not available")
    spdf_bytes, eid = _get_element_eid(client, sample_semantic, "Heading")

    # Redact
    redact_resp = client.post(
        "/api/v1/documents/redact",
        files={"file": ("doc.spdf", spdf_bytes, "application/octet-stream")},
        data={"target_eid": eid, "reason": "Legal hold"},
    )
    assert redact_resp.status_code == 200
    redacted = redact_resp.content

    # List
    list_resp = client.post(
        "/api/v1/documents/redactions",
        files={"file": ("r.spdf", redacted, "application/octet-stream")},
    )
    assert list_resp.status_code == 200
    redactions = list_resp.json()["redactions"]
    assert len(redactions) == 1

    # Verify using the redacted_eid
    verify_resp = client.post(
        "/api/v1/documents/verify-redaction",
        files={"file": ("r.spdf", redacted, "application/octet-stream")},
        data={"redaction_eid": eid},
    )
    assert verify_resp.status_code == 200
    assert verify_resp.json()["found"] is True
