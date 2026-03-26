"""In-memory template store.

Templates are named semantic JSON presets with variable placeholders.
Supports CRUD with cursor-based pagination (ADR-008).
"""

from __future__ import annotations

import uuid
from datetime import datetime, timezone
from typing import Any


class TemplateStore:
    """Dict-backed template store keyed by template_id."""

    def __init__(self) -> None:
        self._templates: dict[str, dict] = {}

    def create(
        self,
        name: str,
        description: str = "",
        semantic_template: dict[str, Any] | None = None,
    ) -> dict:
        now = datetime.now(timezone.utc).isoformat()
        tid = str(uuid.uuid4())
        template = {
            "id": tid,
            "name": name,
            "description": description,
            "semantic_template": semantic_template or {},
            "created_at": now,
            "updated_at": now,
            "deleted": False,
        }
        self._templates[tid] = template
        return template

    def get(self, template_id: str) -> dict | None:
        t = self._templates.get(template_id)
        if t and not t["deleted"]:
            return t
        return None

    def list(self, cursor: str | None = None, limit: int = 20) -> tuple[list[dict], str | None]:
        """Return (items, next_cursor). Cursor-based pagination."""
        active = sorted(
            (t for t in self._templates.values() if not t["deleted"]),
            key=lambda t: t["created_at"],
        )

        start_idx = 0
        if cursor:
            for i, t in enumerate(active):
                if t["id"] == cursor:
                    start_idx = i + 1
                    break

        page = active[start_idx : start_idx + limit]
        next_cursor = page[-1]["id"] if len(page) == limit else None
        return page, next_cursor

    def update(
        self,
        template_id: str,
        name: str | None = None,
        description: str | None = None,
        semantic_template: dict[str, Any] | None = None,
    ) -> dict | None:
        t = self.get(template_id)
        if not t:
            return None
        if name is not None:
            t["name"] = name
        if description is not None:
            t["description"] = description
        if semantic_template is not None:
            t["semantic_template"] = semantic_template
        t["updated_at"] = datetime.now(timezone.utc).isoformat()
        return t

    def delete(self, template_id: str) -> bool:
        t = self._templates.get(template_id)
        if t and not t["deleted"]:
            t["deleted"] = True
            return True
        return False

    def reset(self) -> None:
        self._templates.clear()


template_store = TemplateStore()
