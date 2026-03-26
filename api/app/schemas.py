"""Pydantic request/response models for all SPDF API endpoints."""

from __future__ import annotations

from typing import Any

from pydantic import BaseModel, Field


# --- Generation ---


class GenerateRequest(BaseModel):
    semantic: dict[str, Any]
    layout: dict[str, Any] = Field(default_factory=lambda: {"layout": "default"})
    styles: dict[str, Any] = Field(default_factory=lambda: {"styles": {}})
    metadata: dict[str, Any] = Field(default_factory=dict)
    audit: dict[str, Any] | None = None


# --- Validation ---


class ValidationReport(BaseModel):
    valid: bool
    manifest_errors: list[dict[str, Any]]
    document_errors: list[dict[str, Any]]
    error_count: int
    fatal_count: int


# --- Parse ---


class ParseRequest(BaseModel):
    semantic_json: str


class ParseResponse(BaseModel):
    document: dict[str, Any]


# --- Extract ---


class InvoicePartyInfo(BaseModel):
    name: str | None = None
    address: str | None = None
    gstin: str | None = None


class InvoiceData(BaseModel):
    invoice_number: str | None = None
    issue_date: str | None = None
    due_date: str | None = None
    vendor: InvoicePartyInfo | None = None
    client: InvoicePartyInfo | None = None
    currency: str | None = None
    line_items: list[Any] = Field(default_factory=list)
    subtotal: str | None = None
    tax_label: str | None = None
    tax_amount: str | None = None
    discount: str | None = None
    total: str | None = None
    payment_method: str | None = None


# --- Health ---


class HealthResponse(BaseModel):
    status: str = "ok"
    engine_version: str
    api_version: str = "0.1.0"


# --- Account ---


class ApiKeyResponse(BaseModel):
    key_prefix: str
    created_at: str
    last_used_at: str | None = None


class ApiKeyRotateResponse(BaseModel):
    api_key: str
    key_prefix: str
    created_at: str
    warning: str = "Store this key securely. It will not be shown again."


class UsageResponse(BaseModel):
    user_id: str
    tier: str
    usage: dict[str, int] = Field(default_factory=dict)
    limits: dict[str, int] = Field(default_factory=dict)
    next_reset_at: str


# --- Billing ---


class SubscriptionResponse(BaseModel):
    plan: str
    status: str
    current_period_start: str | None = None
    current_period_end: str | None = None
    created_at: str | None = None


class CheckoutRequest(BaseModel):
    plan: str


class CheckoutResponse(BaseModel):
    checkout_url: str


class PortalResponse(BaseModel):
    portal_url: str


# --- Templates ---


class TemplateCreate(BaseModel):
    name: str
    description: str = ""
    semantic_template: dict[str, Any] = Field(default_factory=dict)


class TemplateUpdate(BaseModel):
    name: str | None = None
    description: str | None = None
    semantic_template: dict[str, Any] | None = None


class TemplateResponse(BaseModel):
    id: str
    name: str
    description: str
    semantic_template: dict[str, Any]
    created_at: str
    updated_at: str


class TemplateListResponse(BaseModel):
    items: list[TemplateResponse]
    next_cursor: str | None = None
    has_more: bool = False
