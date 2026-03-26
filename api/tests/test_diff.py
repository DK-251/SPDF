"""Tests for the semantic diff endpoint."""

from __future__ import annotations

import copy
from typing import Any

import pytest
from fastapi.testclient import TestClient


def _gen(client: TestClient, semantic: dict[str, Any]) -> bytes:
    resp = client.post("/api/v1/documents/generate", json={"semantic": semantic})
    assert resp.status_code == 200
    return resp.content


# ---------- Identical documents ----------


def test_diff_identical_documents(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    spdf = _gen(client, sample_semantic)
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf, "application/octet-stream"),
            "file_b": ("b.spdf", spdf, "application/octet-stream"),
        },
    )
    assert resp.status_code == 200
    report = resp.json()
    assert report["summary"]["total_changes"] == 0


# ---------- Metadata changes ----------


def test_diff_title_change(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    sem_a = copy.deepcopy(sample_semantic)
    sem_b = copy.deepcopy(sample_semantic)
    sem_b["title"] = "Modified Invoice"

    spdf_a = _gen(client, sem_a)
    spdf_b = _gen(client, sem_b)
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf_a, "application/octet-stream"),
            "file_b": ("b.spdf", spdf_b, "application/octet-stream"),
        },
    )
    assert resp.status_code == 200
    report = resp.json()
    assert report["summary"]["total_changes"] > 0
    title_changes = [
        c for c in report["metadata_changes"] if c.get("field") == "title"
    ]
    assert len(title_changes) == 1


# ---------- Element changes ----------


def test_diff_element_added(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    import time
    import uuid

    sem_a = copy.deepcopy(sample_semantic)
    sem_b = copy.deepcopy(sample_semantic)
    now = sem_b["pages"][0]["elements"][0].get("created_at", "2026-01-01T00:00:00Z")
    new_eid = f"el-{int(time.time()*1000)}-{uuid.uuid4().hex[:4]}"
    sem_b["pages"][0]["elements"].append(
        {
            "element_type": "Paragraph",
            "eid": new_eid,
            "text": "New paragraph",
            "created_at": now,
            "modified_at": now,
        }
    )

    spdf_a = _gen(client, sem_a)
    spdf_b = _gen(client, sem_b)
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf_a, "application/octet-stream"),
            "file_b": ("b.spdf", spdf_b, "application/octet-stream"),
        },
    )
    assert resp.status_code == 200
    report = resp.json()
    assert report["summary"]["added"] >= 1


def test_diff_element_removed(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    sem_a = copy.deepcopy(sample_semantic)
    sem_b = copy.deepcopy(sample_semantic)
    sem_b["pages"][0]["elements"].pop()  # Remove last element

    spdf_a = _gen(client, sem_a)
    spdf_b = _gen(client, sem_b)
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf_a, "application/octet-stream"),
            "file_b": ("b.spdf", spdf_b, "application/octet-stream"),
        },
    )
    assert resp.status_code == 200
    report = resp.json()
    assert report["summary"]["removed"] >= 1


def test_diff_element_modified(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    sem_a = copy.deepcopy(sample_semantic)
    sem_b = copy.deepcopy(sample_semantic)
    # Modify heading text
    sem_b["pages"][0]["elements"][0]["text"] = "Modified Heading"

    spdf_a = _gen(client, sem_a)
    spdf_b = _gen(client, sem_b)
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf_a, "application/octet-stream"),
            "file_b": ("b.spdf", spdf_b, "application/octet-stream"),
        },
    )
    assert resp.status_code == 200
    report = resp.json()
    assert report["summary"]["modified"] >= 1


# ---------- Impact classification ----------


def test_diff_financial_change_is_major(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    sem_a = copy.deepcopy(sample_semantic)
    sem_b = copy.deepcopy(sample_semantic)
    # Find PaymentTerms and change total
    for el in sem_b["pages"][0]["elements"]:
        if el.get("element_type") == "PaymentTerms":
            el["total"] = "999999.00"
            break

    spdf_a = _gen(client, sem_a)
    spdf_b = _gen(client, sem_b)
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf_a, "application/octet-stream"),
            "file_b": ("b.spdf", spdf_b, "application/octet-stream"),
        },
    )
    assert resp.status_code == 200
    report = resp.json()
    total_changes = [
        c for c in report["element_changes"] if c.get("field") == "total"
    ]
    assert len(total_changes) > 0
    assert total_changes[0]["impact"] == "MAJOR"


# ---------- Summary structure ----------


def test_diff_summary_structure(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    spdf = _gen(client, sample_semantic)
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf, "application/octet-stream"),
            "file_b": ("b.spdf", spdf, "application/octet-stream"),
        },
    )
    report = resp.json()
    summary = report["summary"]
    assert "added" in summary
    assert "removed" in summary
    assert "modified" in summary
    assert "total_changes" in summary
    assert "highest_impact" in summary


def test_diff_missing_file_b(
    client: TestClient, sample_spdf_bytes: bytes, engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", sample_spdf_bytes, "application/octet-stream"),
        },
    )
    assert resp.status_code == 422


def test_diff_state_change_is_critical(
    client: TestClient, sample_semantic: dict[str, Any], engine_available: bool
) -> None:
    if not engine_available:
        pytest.skip("spdf_native not available")
    sem_a = copy.deepcopy(sample_semantic)
    spdf_a = _gen(client, sem_a)

    # Transition doc_b to REVIEW
    resp = client.post(
        "/api/v1/documents/transition",
        files={"file": ("doc.spdf", spdf_a, "application/octet-stream")},
        data={"target_state": "REVIEW"},
    )
    assert resp.status_code == 200
    spdf_b = resp.content

    resp = client.post(
        "/api/v1/documents/diff",
        files={
            "file_a": ("a.spdf", spdf_a, "application/octet-stream"),
            "file_b": ("b.spdf", spdf_b, "application/octet-stream"),
        },
    )
    assert resp.status_code == 200
    report = resp.json()
    assert report["summary"]["highest_impact"] == "CRITICAL"
