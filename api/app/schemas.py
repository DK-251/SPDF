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
