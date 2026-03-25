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
    def version() -> str:
        """Return the underlying Rust engine version."""
        return spdf_native.version()
