"""Thin Python wrapper around the spdf_native Rust extension module.

Provides SpdfEngine as the single entry point for all SPDF operations
used by the FastAPI routers. All heavy lifting happens in Rust via PyO3.
"""

from __future__ import annotations

import json
from typing import Any

import spdf_native


class SpdfEngine:
    """Stateless facade over the Rust SPDF core engine."""

    @staticmethod
    def validate(spdf_bytes: bytes) -> dict[str, Any]:
        """Validate an SPDF container. Returns the validation report dict."""
        raw = spdf_native.validate_spdf(spdf_bytes)
        return json.loads(raw)

    @staticmethod
    def generate(
        semantic: dict[str, Any],
        layout: dict[str, Any],
        styles: dict[str, Any],
        metadata: dict[str, Any],
        audit: dict[str, Any] | None = None,
    ) -> bytes:
        """Build a complete .spdf container from layer dicts. Returns raw bytes."""
        audit = audit or {"entries": []}
        return bytes(
            spdf_native.generate_spdf(
                json.dumps(semantic),
                json.dumps(layout),
                json.dumps(styles),
                json.dumps(metadata),
                json.dumps(audit),
            )
        )

    @staticmethod
    def render_pdf(spdf_bytes: bytes) -> bytes:
        """Render the semantic layer of an SPDF container to PDF bytes."""
        return bytes(spdf_native.render_to_pdf(spdf_bytes))

    @staticmethod
    def parse(semantic_json: str) -> dict[str, Any]:
        """Parse and validate a semantic JSON string. Returns the Document dict."""
        raw = spdf_native.parse_semantic(semantic_json)
        return json.loads(raw)

    @staticmethod
    def extract(spdf_bytes: bytes) -> dict[str, Any]:
        """Extract structured invoice data from an SPDF container."""
        raw = spdf_native.extract_invoice_data(spdf_bytes)
        return json.loads(raw)

    @staticmethod
    def sign(spdf_bytes: bytes, signer_name: str, signer_email: str) -> bytes:
        """Sign an SPDF document. Must be in Review state. Returns signed bytes."""
        return bytes(spdf_native.sign_document(spdf_bytes, signer_name, signer_email))

    @staticmethod
    def verify(spdf_bytes: bytes) -> dict[str, Any]:
        """Verify all signatures in an SPDF document."""
        raw = spdf_native.verify_document(spdf_bytes)
        return json.loads(raw)

    @staticmethod
    def diff(doc_a_bytes: bytes, doc_b_bytes: bytes) -> dict[str, Any]:
        """Compare two SPDF documents. Returns the diff report dict."""
        raw = spdf_native.diff_documents(doc_a_bytes, doc_b_bytes)
        return json.loads(raw)

    @staticmethod
    def redact(spdf_bytes: bytes, target_eid: str, reason: str) -> bytes:
        """Redact an element from an SPDF document. Returns updated bytes."""
        return bytes(spdf_native.redact_element(spdf_bytes, target_eid, reason))

    @staticmethod
    def verify_redaction(spdf_bytes: bytes, redaction_eid: str) -> dict[str, Any]:
        """Verify a redaction exists and return its proof hash."""
        raw = spdf_native.verify_redaction(spdf_bytes, redaction_eid)
        return json.loads(raw)

    @staticmethod
    def list_redactions(spdf_bytes: bytes) -> list[dict[str, Any]]:
        """List all redactions in an SPDF document."""
        raw = spdf_native.list_redactions(spdf_bytes)
        return json.loads(raw)

    @staticmethod
    def transition(spdf_bytes: bytes, target_state: str) -> bytes:
        """Transition a document to a new state. Returns updated bytes."""
        return bytes(spdf_native.transition_document(spdf_bytes, target_state))

    @staticmethod
    def version() -> str:
        """Return the underlying Rust engine version."""
        return spdf_native.version()
