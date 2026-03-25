"""Shared fixtures for SPDF API tests."""

from __future__ import annotations

import json
import time
import uuid
from datetime import datetime, timezone
from typing import Any

import pytest
from fastapi.testclient import TestClient

from app.main import app


def _eid() -> str:
    ms = int(time.time() * 1000)
    rand = uuid.uuid4().hex[:4]
    return f"el-{ms}-{rand}"


def _now_iso() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%fZ")


@pytest.fixture(scope="session")
def engine_available() -> bool:
    try:
        import spdf_native  # noqa: F401

        return True
    except ImportError:
        return False


@pytest.fixture()
def client() -> TestClient:
    return TestClient(app)


@pytest.fixture()
def sample_semantic() -> dict[str, Any]:
    """A valid invoice semantic document matching the Rust DOM serde format."""
    now = _now_iso()
    return {
        "spdf:version": {"major": 1, "minor": 0},
        "document_id": f"spdf-{uuid.uuid4()}",
        "title": "Invoice INV-2026-200",
        "locale": "en-IN",
        "direction": "LTR",
        "document_state": "DRAFT",
        "pages": [
            {
                "eid": _eid(),
                "page_number": 1,
                "elements": [
                    {
                        "element_type": "Heading",
                        "eid": _eid(),
                        "level": 1,
                        "text": "Tax Invoice",
                        "created_at": now,
                        "modified_at": now,
                    },
                    {
                        "element_type": "InvoiceHeader",
                        "eid": _eid(),
                        "invoice_number": "INV-2026-200",
                        "issue_date": "2026-03-25",
                        "due_date": "2026-04-25",
                        "vendor": {
                            "name": "SPDF Corp",
                            "address": "HSR Layout, Bangalore 560102",
                            "gstin": "29AABCU9603R1ZM",
                        },
                        "client": {
                            "name": "Acme Industries",
                            "address": "MG Road, Mumbai 400001",
                            "gstin": "27AADCA1234B1ZK",
                        },
                        "currency": "INR",
                        "created_at": now,
                        "modified_at": now,
                    },
                    {
                        "element_type": "LineItemTable",
                        "eid": _eid(),
                        "headers": ["Description", "Qty", "Rate", "Amount"],
                        "rows": [
                            [
                                {"value": "API Integration", "spdf:type": "text"},
                                {"value": "1", "spdf:type": "integer"},
                                {"value": "75000.00", "spdf:type": "currency"},
                                {"value": "75000.00", "spdf:type": "currency"},
                            ],
                            [
                                {"value": "PDF Templates (5)", "spdf:type": "text"},
                                {"value": "5", "spdf:type": "integer"},
                                {"value": "10000.00", "spdf:type": "currency"},
                                {"value": "50000.00", "spdf:type": "currency"},
                            ],
                        ],
                        "created_at": now,
                        "modified_at": now,
                    },
                    {
                        "element_type": "PaymentTerms",
                        "eid": _eid(),
                        "subtotal": "125000.00",
                        "discount": "12500.00",
                        "tax_label": "IGST 18%",
                        "tax_amount": "20250.00",
                        "total": "132750.00",
                        "payment_method": "NEFT",
                        "created_at": now,
                        "modified_at": now,
                    },
                ],
            }
        ],
    }


@pytest.fixture()
def sample_non_invoice_semantic() -> dict[str, Any]:
    """A valid document with no invoice elements (just a heading + paragraph)."""
    now = _now_iso()
    return {
        "spdf:version": {"major": 1, "minor": 0},
        "document_id": f"spdf-{uuid.uuid4()}",
        "title": "Plain Document",
        "locale": "en-US",
        "direction": "LTR",
        "document_state": "DRAFT",
        "pages": [
            {
                "eid": _eid(),
                "page_number": 1,
                "elements": [
                    {
                        "element_type": "Heading",
                        "eid": _eid(),
                        "level": 1,
                        "text": "Chapter One",
                        "created_at": now,
                        "modified_at": now,
                    },
                    {
                        "element_type": "Paragraph",
                        "eid": _eid(),
                        "text": "This document has no invoice data.",
                        "created_at": now,
                        "modified_at": now,
                    },
                ],
            }
        ],
    }


@pytest.fixture()
def sample_spdf_bytes(
    client: TestClient,
    sample_semantic: dict[str, Any],
    engine_available: bool,
) -> bytes:
    """Generate real .spdf bytes via the generate endpoint."""
    if not engine_available:
        pytest.skip("spdf_native not available")
    resp = client.post("/api/v1/documents/generate", json={"semantic": sample_semantic})
    assert resp.status_code == 200, f"Generate failed: {resp.text}"
    return resp.content
