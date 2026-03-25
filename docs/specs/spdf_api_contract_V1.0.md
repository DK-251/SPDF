# SPDF API Contract Specification
## SPDF-API-2025-001 · Version 1.0 · Status: APPROVED — Ready for Implementation

**Predecessor:** SPDF-SAD-2025-001 (System Architecture Design)  
**Successor:** SPDF-DB-001 (Database Schema Design), SPDF-SPRINT-001 (Development Sprint Plan)  
**Classification:** Internal Engineering — Confidential

---

## Decisions Recorded

| Question | Decision |
|---|---|
| OQ-07: Authentication | Both API Keys (SDK/scripts) + JWT Bearer (Studio/OAuth) |
| OQ-06: Claude model | Auto-detect: Haiku for simple docs, Sonnet for complex |
| OQ-04: Render layer | Optional — developer specifies `include_render_layer` at generation time |

---

## Section 01 — API Design Principles

### 1.1 Core Conventions

| Principle | Implementation |
|---|---|
| Base URL | `https://api.spdf.dev/v1` |
| Protocol | HTTPS only. HTTP connections receive `301 Moved Permanently` |
| Versioning | Major version in URL path (`/v1/`). Breaking changes → `/v2/`. Never break existing clients |
| Data format | `application/json` for all request and response bodies unless file upload/download |
| Encoding | UTF-8 on all text content |
| Pagination | Cursor-based on all list endpoints. Never offset-based |
| Async operations | Any operation expected to exceed 5s returns `202 Accepted` + `job_id`. Client polls `/v1/jobs/{id}` |
| Idempotency | All `POST` mutation endpoints accept `X-Idempotency-Key` header (UUID4). 24-hour dedup window |
| Null vs absent | `null` = explicit no-value. Key absent = use default. Treated as semantically distinct |

### 1.2 Authentication Model

Two authentication paths are supported simultaneously on all protected endpoints.

**Path 1 — API Key (developers, SDK, scripts)**

```
Authorization: Bearer sk_live_EXAMPLE_KEY_PLACEHOLDER_00
```

- Key format: `sk_live_{26 base62 chars}` (production) or `sk_test_{26 base62 chars}` (test mode)
- Stored as bcrypt hash in `users.api_key_hash`. Never stored plaintext
- Test keys never trigger billing. All other behaviour is identical
- Key prefix (first 8 chars) stored in `users.api_key_prefix` for display only

**Path 2 — JWT Bearer (Studio, OAuth apps)**

```
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
```

- Issued and validated by Clerk
- Contains: `sub` (user_id), `org_id`, `email`, `tier`, `session_id`
- Verified on every request via `clerk.verify_token(jwt)`

### 1.3 Standard Request Headers

| Header | Required | Description |
|---|---|---|
| `Authorization` | REQUIRED | `Bearer {api_key_or_jwt}` |
| `Content-Type` | REQUIRED on POST/PUT | `application/json` or `multipart/form-data` |
| `X-Idempotency-Key` | RECOMMENDED on POST | UUID4. Prevents double-execution on retry |
| `X-SPDF-Version` | OPTIONAL | Preferred SPDF format version. Default: `1.0` |

### 1.4 Standard Response Headers

| Header | Description |
|---|---|
| `X-Request-Id` | UUID4 unique to this request |
| `X-RateLimit-Limit` | Requests allowed in current window |
| `X-RateLimit-Remaining` | Requests remaining in current window |
| `X-RateLimit-Reset` | Unix epoch when rate limit window resets |
| `X-SPDF-Version` | SPDF format version used to process this request |
| `X-Processing-Time-Ms` | Milliseconds spent processing the request |

### 1.5 Rate Limiting

| Tier | `/convert` | `/generate` | `/extract` | `/sign` + `/verify` | All other |
|---|---|---|---|---|---|
| FREE | 10/day | 50/day | 100/day | 50/day | 500/day |
| PRO | 1,000/day | 5,000/day | 10,000/day | 2,000/day | 50,000/day |
| TEAM | 10,000/day | 50,000/day | 100,000/day | 20,000/day | 500,000/day |
| ENTERPRISE | Custom | Custom | Custom | Custom | Custom |

### 1.6 Pagination Model

All list endpoints use cursor-based pagination.

**Request parameters:** `limit` (1–100, default 20), `cursor` (opaque string from previous `next_cursor`)

**Response shape:**

```json
{
  "data": [],
  "pagination": {
    "limit": 20,
    "has_more": true,
    "next_cursor": "eyJjcmVhdGVkX2F0IjoiMjAyNS0wMy0xNVQwOTozMDowMC4wMDBaIn0=",
    "total_count": null
  }
}
```

`total_count` is always `null`. `has_more: false` + `next_cursor: null` signals the final page.

---

## Section 02 — Error Model

### 2.1 Standard Error Response

```json
{
  "error": {
    "code": "DOCUMENT_NOT_FOUND",
    "message": "Document not found or you do not have access to it.",
    "details": {
      "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a"
    },
    "request_id": "req_a1b2c3d4e5f6",
    "docs_url": "https://docs.spdf.dev/errors#DOCUMENT_NOT_FOUND"
  }
}
```

### 2.2 HTTP Status to Error Class Mapping

| HTTP Status | Error Class | When Used |
|---|---|---|
| `400 Bad Request` | Input errors | Malformed JSON, missing required fields, invalid values |
| `401 Unauthorized` | Auth errors | Missing token, invalid token, expired token |
| `403 Forbidden` | Permission errors | Valid auth but insufficient permission |
| `404 Not Found` | Resource errors | Document, job, template does not exist |
| `409 Conflict` | State errors | Operation not valid given current resource state |
| `413 Payload Too Large` | Size errors | Uploaded file exceeds size limit |
| `422 Unprocessable` | Validation errors | SPDF schema failures |
| `429 Too Many Requests` | Rate limit errors | Daily quota exceeded |
| `500 Internal Server Error` | Server errors | Unexpected error. Never exposes stack traces |
| `502 Bad Gateway` | Upstream errors | Claude API unavailable, R2 unreachable |
| `503 Service Unavailable` | Availability errors | Planned maintenance or degraded state |

### 2.3 Error Code Registry

**Authentication errors (401)**

| Code | Description |
|---|---|
| `AUTH_REQUIRED` | No Authorization header present |
| `INVALID_TOKEN` | Token format invalid or signature check failed |
| `EXPIRED_TOKEN` | JWT has expired |
| `INVALID_API_KEY` | API key not found or revoked |

**Permission errors (403)**

| Code | Description |
|---|---|
| `PERMISSION_DENIED` | User does not own this resource |
| `TIER_REQUIRED` | Operation requires a higher subscription tier |
| `QUOTA_EXCEEDED` | Monthly conversion quota exhausted |

**Resource errors (404)**

| Code | Description |
|---|---|
| `DOCUMENT_NOT_FOUND` | Document ID does not exist or is not accessible |
| `JOB_NOT_FOUND` | Job ID does not exist or is not accessible |
| `TEMPLATE_NOT_FOUND` | Template ID does not exist or is not accessible |

**Input errors (400)**

| Code | Description |
|---|---|
| `MISSING_REQUIRED_FIELD` | A required request field is absent |
| `INVALID_FIELD_VALUE` | A field value fails type or range validation |
| `MALFORMED_JSON` | Request body is not valid JSON |
| `UNSUPPORTED_FILE_TYPE` | Uploaded file is not a supported type |
| `FILE_CORRUPTED` | Uploaded file is corrupt or unreadable |

**State errors (409)**

| Code | Description |
|---|---|
| `DOCUMENT_ALREADY_SIGNED` | Attempted to modify a SIGNED or CERTIFIED document |
| `DOCUMENT_NOT_SIGNABLE` | Document is not in a signable state |
| `STATE_TRANSITION_INVALID` | Requested state transition is not permitted |
| `JOB_NOT_CANCELLABLE` | Job is PROCESSING and can no longer be cancelled |

**Validation errors (422)**

| Code | Description |
|---|---|
| `SPDF_VALIDATION_FAILED` | SPDF fails schema validation. `details.errors[]` contains error codes |
| `TEMPLATE_VARIABLE_MISSING` | Required template variable not provided |
| `TEMPLATE_VARIABLE_INVALID` | Template variable fails type/format validation |

**Rate limit error (429)**

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Daily conversion limit of 10 reached for FREE tier.",
    "details": {
      "limit": 10,
      "used": 10,
      "resets_at": "2025-03-16T00:00:00.000Z",
      "upgrade_url": "https://spdf.dev/pricing"
    },
    "request_id": "req_a1b2c3d4e5f6",
    "docs_url": "https://docs.spdf.dev/errors#RATE_LIMIT_EXCEEDED"
  }
}
```

---

## Section 03 — Shared Schema Definitions

### 3.1 Enums

```
DocumentState:  DRAFT | REVIEW | SIGNED | CERTIFIED
JobStatus:      QUEUED | PROCESSING | COMPLETED | FAILED | CANCELLED
Tier:           FREE | PRO | TEAM | ENTERPRISE
```

### 3.2 DocumentSummary Object

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "title": "Invoice #INV-2025-001",
  "document_type": "Invoice",
  "state": "DRAFT",
  "page_count": 2,
  "file_size_bytes": 48291,
  "spdf_version": "1.0",
  "source_format": "NATIVE",
  "confidence_score": null,
  "locale": "en-IN",
  "created_at": "2025-03-15T09:30:00.000Z",
  "updated_at": "2025-03-15T09:30:00.000Z",
  "owner_user_id": "user_88291abc",
  "owner_org_id": null
}
```

### 3.3 JobResponse Object

```json
{
  "job_id": "job_f3e2d1c0-b9a8-7654-3210-fedcba987654",
  "job_type": "PDF_TO_SPDF",
  "status": "PROCESSING",
  "progress": 42,
  "current_step": "claude_extract",
  "input_file_name": "vendor_invoice_march.pdf",
  "output_document_id": null,
  "result": null,
  "error": null,
  "created_at": "2025-03-15T09:30:00.000Z",
  "started_at": "2025-03-15T09:30:02.000Z",
  "completed_at": null,
  "expires_at": "2025-03-22T09:30:00.000Z",
  "_links": {
    "self": "/v1/jobs/job_f3e2d1c0-b9a8-7654-3210-fedcba987654",
    "cancel": "/v1/jobs/job_f3e2d1c0-b9a8-7654-3210-fedcba987654"
  }
}
```

### 3.4 ConfidenceReport Object

```json
{
  "overall_score": 0.94,
  "element_count": 47,
  "high_confidence_count": 44,
  "low_confidence_elements": [
    {
      "eid": "el-1709251200000-00031-d4e5",
      "element_type": "LineItem",
      "score": 0.71,
      "reason": "Ambiguous column alignment in source PDF"
    }
  ],
  "model_used": "claude-haiku-4-5",
  "conversion_method": "AI_ASSISTED"
}
```

---

## Section 04 — Document Endpoints

### 4.1 Upload PDF for Conversion

```
POST /v1/documents/upload
Content-Type: multipart/form-data
```

**Request fields:**

| Field | Type | Required | Description |
|---|---|---|---|
| `file` | binary | REQUIRED | PDF file. Max 50 MB |
| `title` | string | OPTIONAL | Document title. Defaults to filename |
| `locale` | string | OPTIONAL | BCP 47 locale hint (e.g. `en-IN`) |
| `document_type` | string | OPTIONAL | Type hint (e.g. `Invoice`) |
| `webhook_url` | string | OPTIONAL | HTTPS URL to POST completion notification |
| `webhook_secret` | string | OPTIONAL | HMAC secret for webhook auth |

**Response `202 Accepted`:**

```json
{
  "job_id": "job_f3e2d1c0-b9a8-7654-3210-fedcba987654",
  "status": "QUEUED",
  "estimated_seconds": 15,
  "_links": {
    "status": "/v1/jobs/job_f3e2d1c0-b9a8-7654-3210-fedcba987654"
  }
}
```

---

### 4.2 Generate SPDF from Template

```
POST /v1/documents/generate
Content-Type: application/json
X-Idempotency-Key: {uuid4}
```

**Request body:**

```json
{
  "template_id": "tmpl_invoice_standard_v2",
  "title": "Invoice #INV-2025-001",
  "locale": "en-IN",
  "data": {
    "INVOICE_NUMBER": "INV-2025-001",
    "ISSUE_DATE": "2025-03-15",
    "DUE_DATE": "2025-04-14",
    "VENDOR_NAME": "ACME Software Solutions Pvt. Ltd.",
    "CLIENT_NAME": "GlobalTech India Pvt. Ltd.",
    "LINE_ITEMS": [
      {
        "description": "Backend API Development",
        "qty": 80,
        "unit": "hours",
        "unit_price": "2500.00",
        "hsn_code": "998314"
      }
    ],
    "CURRENCY": "INR",
    "TAX_SCHEME": "GST_18"
  },
  "options": {
    "include_render_layer": true,
    "output_format": "spdf",
    "sign_after_generate": false
  }
}
```

**Response `201 Created`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "title": "Invoice #INV-2025-001",
  "state": "DRAFT",
  "page_count": 2,
  "file_size_bytes": 48291,
  "include_render_layer": true,
  "download_url": "https://files.spdf.dev/...?X-Signature=abc123&Expires=1709254800",
  "download_url_expires_at": "2025-03-15T10:30:00.000Z",
  "pdf_download_url": null,
  "created_at": "2025-03-15T09:30:00.000Z",
  "_links": {
    "self": "/v1/documents/spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
    "extract": "/v1/documents/spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a/extract",
    "sign": "/v1/documents/spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a/sign",
    "validate": "/v1/documents/spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a/validate"
  }
}
```

---

### 4.3 List Documents

```
GET /v1/documents
```

**Query parameters:** `limit`, `cursor`, `state`, `document_type`, `source_format`, `q` (title search)

---

### 4.4 Get Document

```
GET /v1/documents/{document_id}
```

Returns document metadata and a fresh signed download URL (1 hour expiry).

---

### 4.5 Delete Document

```
DELETE /v1/documents/{document_id}
```

Soft-delete only. Returns `204 No Content`. Returns `409 DOCUMENT_ALREADY_SIGNED` if state is SIGNED or CERTIFIED.

---

### 4.6 Extract Structured Data

```
POST /v1/documents/{document_id}/extract
Content-Type: application/json
```

**Request body:**

```json
{
  "element_types": ["InvoiceHeader", "LineItemTable", "PaymentTerms"],
  "enhance": false,
  "include_element_tree": false,
  "include_confidence_scores": true
}
```

**Response `200 OK`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "document_type": "Invoice",
  "document_state": "DRAFT",
  "confidence_report": { "overall_score": 0.94, "element_count": 47 },
  "structured_data": {
    "InvoiceHeader": {
      "invoice_number": "INV-2025-001",
      "issue_date": "2025-03-15T00:00:00.000Z",
      "due_date": "2025-04-14T23:59:59.999Z",
      "vendor": { "name": "ACME Software Solutions Pvt. Ltd.", "gstin": "27AABCA1234F1Z5" },
      "client": { "name": "GlobalTech India Pvt. Ltd.", "gstin": "29AACG5678H1ZQ" },
      "_confidence": 0.98
    },
    "LineItems": [
      {
        "description": "Backend API Development",
        "qty": "80",
        "unit": "hours",
        "unit_price": "2500.00",
        "total": "200000.00",
        "currency": "INR",
        "_confidence": 0.97
      }
    ],
    "FinancialSummary": {
      "subtotal": "280000.00",
      "tax_rate": "18.00",
      "tax_amount": "50400.00",
      "total": "330400.00",
      "currency": "INR"
    }
  },
  "element_tree": null,
  "processing_time_ms": 87
}
```

> All financial values are strings in decimal notation. Dates are ISO 8601 UTC. Never floats.

---

### 4.7 Sign Document

```
POST /v1/documents/{document_id}/sign
Content-Type: multipart/form-data
```

**Request fields:**

| Field | Type | Required | Description |
|---|---|---|---|
| `certificate` | binary | REQUIRED | PKCS#12 (.p12) signing certificate |
| `certificate_password` | string | REQUIRED | Password to decrypt the .p12 file |
| `signer_name` | string | REQUIRED | Full name of the signer |
| `signer_title` | string | OPTIONAL | Job title of the signer |
| `signer_email` | string | OPTIONAL | Email of the signer |
| `signature_image` | binary | OPTIONAL | PNG/JPEG handwritten signature image |
| `signature_block_eid` | string | OPTIONAL | EID of the SignatureBlock element to sign |
| `timestamp` | boolean | OPTIONAL | If `true`, adds RFC 3161 timestamp (→ CERTIFIED) |

**Response `200 OK`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "state": "SIGNED",
  "signature_id": "sig-a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "signed_at": "2025-03-15T14:22:10.441Z",
  "signer": {
    "name": "Arjun Sharma",
    "title": "Authorized Signatory",
    "email": "arjun.sharma@acme.com",
    "certificate_fingerprint": "sha256:a1b2c3d4..."
  },
  "document_hash": "sha256:e3b0c44298fc1c149afb...",
  "certified": false,
  "download_url": "https://files.spdf.dev/...?X-Signature=abc123&Expires=1709254800"
}
```

---

### 4.8 Verify Document Signature

```
POST /v1/documents/{document_id}/verify
```

No request body required.

**Response `200 OK`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "is_valid": true,
  "state": "SIGNED",
  "signatures": [
    {
      "signature_id": "sig-a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "signer_name": "Arjun Sharma",
      "signed_at": "2025-03-15T14:22:10.441Z",
      "algorithm": "RSA-PSS-SHA256",
      "document_hash_matches": true,
      "certificate_valid": true
    }
  ],
  "tamper_detected": false,
  "audit_chain_valid": true,
  "verification_time_ms": 48
}
```

---

### 4.9 Validate Document

```
POST /v1/documents/{document_id}/validate
```

**Response `200 OK`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "is_valid": true,
  "error_count": 0,
  "warning_count": 1,
  "errors": [],
  "warnings": [
    {
      "code": "W_LOW_CONTRAST",
      "severity": "Warning",
      "message": "Element fails WCAG 4.5:1 colour contrast ratio",
      "element_eid": "el-1709251200000-00041-b3c2",
      "spec_reference": "SPDF Spec v1.0 Section 8.6"
    }
  ],
  "validated_at": "2025-03-15T09:30:00.000Z"
}
```

---

### 4.10 Semantic Diff

```
GET /v1/documents/{document_id}/diff/{document_id_2}
```

**Query parameters:** `include_unchanged` (boolean, default false)

**Response `200 OK`:**

```json
{
  "document_a_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "document_b_id": "spdf-f1e2d3c4-b5a6-7890-abcd-ef1234567890",
  "diff_summary": {
    "elements_added": 2,
    "elements_removed": 0,
    "elements_modified": 3,
    "elements_unchanged": 42
  },
  "changes": [
    {
      "change_type": "MODIFIED",
      "eid": "el-1709251200000-00031-d4e5",
      "element_type": "TableCell",
      "field": "value",
      "before": "80000.00",
      "after": "96000.00",
      "semantic_impact": "FINANCIAL_VALUE_CHANGED"
    }
  ],
  "processing_time_ms": 312
}
```

---

### 4.11 Transition Document State

```
POST /v1/documents/{document_id}/transition
Content-Type: application/json
```

**Request body:**

```json
{
  "to_state": "REVIEW",
  "comment": "Ready for client review."
}
```

Permitted: `DRAFT → REVIEW` and `REVIEW → DRAFT` only. `REVIEW → SIGNED` is handled by `/sign`.

---

### 4.12 Redact Elements

```
POST /v1/documents/{document_id}/redact
Content-Type: application/json
```

**Request body:**

```json
{
  "eids": ["el-1709251200000-00041-b3c2"],
  "reason": "GDPR Article 17 — Right to Erasure",
  "replacement_text": "[PII ERASED]",
  "regenerate_render_layer": true
}
```

**Response `200 OK`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "erased_count": 1,
  "erased_eids": ["el-1709251200000-00041-b3c2"],
  "proof_hashes": {
    "el-1709251200000-00041-b3c2": "sha256:a1b2c3d4..."
  },
  "render_layer_updated": true,
  "audit_entry_seq": 5
}
```

---

## Section 05 — Job Endpoints

### 5.1 Get Job Status

```
GET /v1/jobs/{job_id}
```

**Response `200 OK` (completed):**

```json
{
  "job_id": "job_f3e2d1c0-b9a8-7654-3210-fedcba987654",
  "job_type": "PDF_TO_SPDF",
  "status": "COMPLETED",
  "progress": 100,
  "current_step": "done",
  "output_document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "result": {
    "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
    "download_url": "https://files.spdf.dev/...?Expires=1709254800",
    "confidence_report": {
      "overall_score": 0.91,
      "model_used": "claude-haiku-4-5"
    }
  },
  "error": null,
  "completed_at": "2025-03-15T09:30:18.000Z"
}
```

**Response `200 OK` (failed):**

```json
{
  "job_id": "job_f3e2d1c0-b9a8-7654-3210-fedcba987654",
  "status": "FAILED",
  "error": {
    "code": "CONVERSION_FAILED",
    "message": "Could not extract meaningful content from this PDF.",
    "is_retryable": false
  }
}
```

---

### 5.2 Cancel Job

```
DELETE /v1/jobs/{job_id}
```

Cancels a QUEUED job only. Returns `409 JOB_NOT_CANCELLABLE` if PROCESSING.

---

### 5.3 List Jobs

```
GET /v1/jobs
```

**Query parameters:** `limit`, `cursor`, `status`, `job_type`

---

## Section 06 — Template Endpoints

### 6.1 List Templates

```
GET /v1/templates
```

**Query parameters:** `limit`, `cursor`, `category`, `visibility` (`own` | `public`)

### 6.2 Get Template

```
GET /v1/templates/{template_id}
```

Returns full template details including `variable_schema` with required and optional variables.

**Response `200 OK`:**

```json
{
  "template_id": "tmpl_invoice_gst_india",
  "name": "GST Invoice (India)",
  "category": "Invoice",
  "is_public": true,
  "variable_schema": {
    "required": [
      { "name": "INVOICE_NUMBER", "type": "string", "example": "INV-2025-001" },
      { "name": "LINE_ITEMS", "type": "array",
        "item_schema": { "description": "string", "qty": "decimal string", "unit_price": "decimal string" }
      }
    ],
    "optional": [
      { "name": "PAYMENT_TERMS", "type": "string", "default": "Payment due within 30 days" }
    ]
  }
}
```

---

## Section 07 — AI Query Endpoint

### 7.1 Natural Language Document Query

```
POST /v1/documents/{document_id}/ask
Content-Type: application/json
```

**Request body:**

```json
{
  "question": "What is the total amount due and when is the payment deadline?",
  "return_structured": true
}
```

**Response `200 OK`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "question": "What is the total amount due and when is the payment deadline?",
  "answer": "The total amount due is ₹3,30,400.00. The payment deadline is 14 April 2025.",
  "structured_answer": {
    "total_amount": "330400.00",
    "currency": "INR",
    "due_date": "2025-04-14T23:59:59.999Z",
    "referenced_eids": ["el-1709251200000-00078-f2e1"]
  },
  "confidence": 0.97,
  "model_used": "claude-haiku-4-5",
  "processing_time_ms": 1240
}
```

> Deferred feature — re-enabled when 10+ active users.

---

## Section 08 — Utility Endpoints

### 8.1 Validate File (Without Uploading)

```
POST /v1/validate
Content-Type: multipart/form-data
```

**Request fields:** `file` (binary, .spdf), `mode` (`strict` | `lenient`, default `strict`)

Returns full `ValidationReport`. No authentication required for first 10 requests per IP per day.

---

### 8.2 Render SPDF to PDF

```
POST /v1/documents/{document_id}/render
Content-Type: application/json
```

**Request body:**

```json
{ "format": "pdf", "regenerate": false }
```

**Response `200 OK`:**

```json
{
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "format": "pdf",
  "regenerated": false,
  "download_url": "https://files.spdf.dev/...render.pdf?Expires=1709254800",
  "file_size_bytes": 152480
}
```

---

### 8.3 Health Check — Liveness

```
GET /v1/health
```

No authentication required. Response time target: < 50ms.

**Response `200 OK`:**

```json
{
  "status": "ok",
  "version": "1.0.0",
  "environment": "production",
  "timestamp": "2025-03-15T09:30:00.000Z"
}
```

---

### 8.4 Health Check — Readiness

```
GET /v1/health/ready
```

**Response `200 OK`:**

```json
{
  "status": "ready",
  "checks": {
    "database": { "status": "ok", "latency_ms": 12 },
    "redis": { "status": "ok", "latency_ms": 4 },
    "r2_storage": { "status": "ok", "latency_ms": 38 },
    "core_engine": { "status": "ok", "version": "1.0.0" },
    "claude_api": { "status": "ok", "latency_ms": 210 }
  },
  "queue_depth": 14,
  "workers_active": 2
}
```

Returns `503` if database, Redis, or R2 are unhealthy. `claude_api` failure returns `200` with `"status": "degraded"`.

---

## Section 09 — Webhook Specification

### 9.1 Webhook Delivery

```
POST {your_webhook_url}
Content-Type: application/json
X-SPDF-Signature: sha256={hmac_hex}
X-SPDF-Event: job.completed
X-Request-Id: req_a1b2c3d4e5f6
```

**Webhook body (job.completed):**

```json
{
  "event": "job.completed",
  "job_id": "job_f3e2d1c0-b9a8-7654-3210-fedcba987654",
  "job_type": "PDF_TO_SPDF",
  "status": "COMPLETED",
  "output_document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "confidence_report": { "overall_score": 0.91 },
  "completed_at": "2025-03-15T09:30:18.000Z"
}
```

### 9.2 Webhook Signature Verification

```python
import hmac, hashlib

def verify_webhook(payload_bytes: bytes, secret: str, signature_header: str) -> bool:
    expected = hmac.new(
        secret.encode("utf-8"),
        payload_bytes,
        hashlib.sha256
    ).hexdigest()
    received = signature_header.removeprefix("sha256=")
    return hmac.compare_digest(expected, received)
```

### 9.3 Webhook Retry Policy

| Attempt | Delay |
|---|---|
| 1st retry | 30 seconds |
| 2nd retry | 5 minutes |
| 3rd retry | 30 minutes |
| 4th retry | 2 hours |
| 5th retry (final) | 12 hours |

> Deferred feature — re-enabled when first developer requests via GitHub issue.

---

## Section 10 — Account Endpoints

### 10.1 Get Current Usage

```
GET /v1/account/usage
```

**Response `200 OK`:**

```json
{
  "user_id": "user_88291abc",
  "tier": "PRO",
  "usage": {
    "conversions": { "used": 47, "limit": 1000, "unit": "per_day_reset_utc" },
    "generations": { "used": 312, "limit": 5000, "unit": "per_day_reset_utc" },
    "extractions": { "used": 89, "limit": 10000, "unit": "per_day_reset_utc" }
  },
  "next_reset_at": "2025-03-16T00:00:00.000Z"
}
```

### 10.2 Get API Key

```
GET /v1/account/api-key
```

**Response `200 OK`:**

```json
{
  "key_prefix": "sk_live_EXAMPLE_",
  "created_at": "2025-03-01T00:00:00.000Z",
  "last_used_at": "2025-03-15T09:28:00.000Z"
}
```

### 10.3 Rotate API Key

```
POST /v1/account/api-key/rotate
```

**Response `200 OK`:**

```json
{
  "api_key": "sk_live_EXAMPLE_KEY_PLACEHOLDER_00",
  "key_prefix": "sk_live_EXAMPLE_",
  "created_at": "2025-03-15T09:30:00.000Z",
  "warning": "Store this key securely. It will not be shown again."
}
```

---

## Section 11 — Claude Model Auto-Detection

### 11.1 Complexity Signals

| Signal | Haiku | Sonnet |
|---|---|---|
| Page count | ≤ 5 pages | > 5 pages |
| Detected tables | ≤ 3 tables | > 3 tables |
| Text density (chars/page) | ≤ 2,000 | > 2,000 |
| Multi-currency detected | No | Yes |
| Mixed language detected | No | Yes |

A document qualifies for Haiku only if **all** signals fall within the Haiku threshold.

### 11.2 Model Override

Include `"model": "sonnet"` in the upload request options to override auto-detection.

---

## Section 12 — Versioning and Deprecation

### 12.1 Non-Breaking Changes (allowed at any time)

- Adding new optional request fields
- Adding new response fields
- Adding new endpoints under `/v1/`
- Adding new error codes or enum values

### 12.2 Breaking Changes (require `/v2/`)

- Removing or renaming a request field
- Removing or renaming a response field
- Changing an HTTP method
- Removing an endpoint

### 12.3 Deprecation Lifecycle

1. Deprecated endpoint receives `X-Deprecation-Warning` response header
2. Six-month notice period
3. Endpoint returns `410 Gone` with `migration_guide_url`

---

## Appendix A — Endpoint Summary

| Method | Path | Auth | Sync | Phase |
|---|---|---|---|---|
| `POST` | `/v1/documents/upload` | Required | No (202) | 1 |
| `POST` | `/v1/documents/generate` | Required | Yes (201) | 1 |
| `GET` | `/v1/documents` | Required | Yes | 1 |
| `GET` | `/v1/documents/{id}` | Required | Yes | 1 |
| `DELETE` | `/v1/documents/{id}` | Required | Yes | 1 |
| `POST` | `/v1/documents/{id}/extract` | Required | Yes | 1 |
| `POST` | `/v1/documents/{id}/sign` | Required | Yes | 2 |
| `POST` | `/v1/documents/{id}/verify` | Required | Yes | 2 |
| `POST` | `/v1/documents/{id}/validate` | Required | Yes | 1 |
| `GET` | `/v1/documents/{id}/diff/{id2}` | Required | Yes | 2 |
| `POST` | `/v1/documents/{id}/transition` | Required | Yes | 2 |
| `POST` | `/v1/documents/{id}/redact` | Required | Yes | 3 |
| `POST` | `/v1/documents/{id}/render` | Required | Yes | 3 |
| `POST` | `/v1/documents/{id}/ask` | Required | Yes | 3 |
| `GET` | `/v1/jobs/{id}` | Required | Yes | 1 |
| `DELETE` | `/v1/jobs/{id}` | Required | Yes | 3 |
| `GET` | `/v1/jobs` | Required | Yes | 1 |
| `GET` | `/v1/templates` | Required | Yes | 2 |
| `GET` | `/v1/templates/{id}` | Required | Yes | 2 |
| `POST` | `/v1/validate` | Optional | Yes | 1 |
| `GET` | `/v1/account/usage` | Required | Yes | 2 |
| `GET` | `/v1/account/api-key` | Required | Yes | 1 |
| `POST` | `/v1/account/api-key/rotate` | Required | Yes | 1 |
| `GET` | `/v1/health` | None | Yes | 1 |
| `GET` | `/v1/health/ready` | None | Yes | 1 |

---

## Appendix B — Change Log

| Version | Date | Author | Changes |
|---|---|---|---|
| 1.0 | March 2025 | Founder + Claude AI | Initial release — all sections, 25 endpoints, full error model |

---

*— End of SPDF API Contract Specification v1.0 —*  
*SPDF Platform · Internal Engineering Document · Confidential*
