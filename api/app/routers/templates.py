"""Template CRUD endpoints."""

from __future__ import annotations

from fastapi import APIRouter, Query, Request

from app.errors import SpdfApiError
from app.schemas import (
    TemplateCreate,
    TemplateListResponse,
    TemplateResponse,
    TemplateUpdate,
)
from app.services.templates import template_store

router = APIRouter(prefix="/api/v1/templates", tags=["templates"])


def _get_user(request: Request) -> dict:
    user = getattr(request.state, "user", None)
    if not user:
        raise SpdfApiError(401, "UNAUTHORIZED", "Not authenticated.")
    return user


def _to_response(t: dict) -> TemplateResponse:
    return TemplateResponse(
        id=t["id"],
        name=t["name"],
        description=t["description"],
        semantic_template=t["semantic_template"],
        created_at=t["created_at"],
        updated_at=t["updated_at"],
    )


@router.post("", response_model=TemplateResponse, status_code=201)
async def create_template(request: Request, body: TemplateCreate) -> TemplateResponse:
    """Create a new template."""
    _get_user(request)
    if not body.name or not body.name.strip():
        raise SpdfApiError(422, "INVALID_PAYLOAD", "Template name is required.")
    t = template_store.create(
        name=body.name.strip(),
        description=body.description,
        semantic_template=body.semantic_template,
    )
    return _to_response(t)


@router.get("", response_model=TemplateListResponse)
async def list_templates(
    request: Request,
    cursor: str | None = Query(default=None),
    limit: int = Query(default=20, ge=1, le=100),
) -> TemplateListResponse:
    """List templates with cursor-based pagination."""
    _get_user(request)
    items, next_cursor = template_store.list(cursor=cursor, limit=limit)
    return TemplateListResponse(
        items=[_to_response(t) for t in items],
        next_cursor=next_cursor,
        has_more=next_cursor is not None,
    )


@router.get("/{template_id}", response_model=TemplateResponse)
async def get_template(request: Request, template_id: str) -> TemplateResponse:
    """Get a template by ID."""
    _get_user(request)
    t = template_store.get(template_id)
    if not t:
        raise SpdfApiError(404, "NOT_FOUND", "Template not found.")
    return _to_response(t)


@router.patch("/{template_id}", response_model=TemplateResponse)
async def update_template(
    request: Request, template_id: str, body: TemplateUpdate
) -> TemplateResponse:
    """Update a template."""
    _get_user(request)
    t = template_store.update(
        template_id,
        name=body.name,
        description=body.description,
        semantic_template=body.semantic_template,
    )
    if not t:
        raise SpdfApiError(404, "NOT_FOUND", "Template not found.")
    return _to_response(t)


@router.delete("/{template_id}", status_code=204)
async def delete_template(request: Request, template_id: str) -> None:
    """Soft-delete a template."""
    _get_user(request)
    if not template_store.delete(template_id):
        raise SpdfApiError(404, "NOT_FOUND", "Template not found.")
