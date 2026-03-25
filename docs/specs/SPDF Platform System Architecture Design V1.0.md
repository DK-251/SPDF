# SPDF Platform — System Architecture Design
**SAD-2025-001 v1.0 | Status: APPROVED — READY FOR IMPLEMENTATION**

*The complete engineering blueprint for how to build the SPDF platform — every component, every data flow, every infrastructure decision, and every architectural trade-off documented for implementation.*

| Field | Value |
|---|---|
| Document ID | SPDF-SAD-2025-001 |
| Version | 1.0 — Initial Release |
| Status | APPROVED — Ready for Implementation |
| Author | Principal Systems Architect |
| Date | March 2025 |
| Predecessor | SPDF-PRD-2025-001 (PRD) │ SPDF-SPEC-2025-001 (Technical Spec) |
| Successor | SPDF-DB-001 (Schema Design) │ SPDF-API-001 (OpenAPI Contract) |
| Review Cycle | On any architectural change; mandatory before Phase 2 build start |
| Classification | Internal Engineering — Confidential |

> **Executive Summary:** This System Architecture Design defines the complete technical blueprint for the SPDF platform — a B2B document infrastructure product that makes PDF documents machine-readable without breaking backward compatibility. The architecture is a modular monolith (Phase 1) built on a Rust core engine exposed via PyO3 to a Python FastAPI backend, with a React/WASM Studio frontend. The design prioritizes simplicity, security, and correctness over premature distribution. All 10 architecture decisions are documented as ADRs with full context and trade-offs.

| Sections | ADRs | Components | Build Time |
|---|---|---|---|
| 14 + Appendix | 10 Decisions | 6 Major | 13–15 Weeks |

---

## Table of Contents

1. [Document Overview & Purpose](#1-document-overview--purpose)
2. [Architecture Goals & Constraints](#2-architecture-goals--constraints)
3. [System Context](#3-system-context)
4. [High-Level Architecture](#4-high-level-architecture)
5. [Component Architecture](#5-component-architecture)
6. [Data Architecture](#6-data-architecture)
7. [API Architecture](#7-api-architecture)
8. [Infrastructure Architecture](#8-infrastructure-architecture)
9. [Security Architecture](#9-security-architecture)
10. [Observability Architecture](#10-observability-architecture)
11. [Deployment Architecture](#11-deployment-architecture)
12. [Architecture Decision Records (ADRs)](#12-architecture-decision-records-adrs)
13. [Cross-Cutting Concerns](#13-cross-cutting-concerns)
14. [Technical Risk Register](#14-technical-risk-register)
- [Appendix A — Technology Version Reference](#appendix-a--technology-version-reference)
- [Appendix B — External Service Account Reference](#appendix-b--external-service-account-reference)
- [Appendix C — Glossary](#appendix-c--glossary)

---

## 1. Document Overview & Purpose

### 1.1 Purpose

This System Architecture Design (SAD) is the authoritative engineering blueprint for the SPDF platform. It translates the "what" established in the PRD into the "how" — specifying every system boundary, component responsibility, data flow, API contract, infrastructure resource, and operational process required to build and operate SPDF.

This document is intended for: the development engineer implementing the system (Claude Code), the founding architect making design decisions, and future engineers who will maintain and extend the platform.

### 1.2 Document Scope

| In Scope | Out of Scope |
|---|---|
| All backend services and their internal design | UI/UX visual design (separate design system document) |
| Database schemas and data flows | Detailed sprint plans and task breakdown |
| API contracts and service boundaries | Business logic rules (covered in PRD) |
| Infrastructure topology and IaC | Marketing or go-to-market details |
| Security architecture and threat model | Third-party vendor internal architecture |
| Observability and operations strategy | Detailed test case specifications |
| Architecture Decision Records (ADRs) | — |

### 1.3 Document Series Position

| # | Document | Status | Dependency |
|---|---|---|---|
| 01 | Vision & Architecture Document | ✅ Complete | — |
| 02 | Product Requirements Document (PRD) | ✅ Complete | Vision |
| 03 | Technical Specification (SPDF-SPEC) | ✅ Complete | PRD |
| 04 | System Architecture Design (SAD) — this document | 🔵 Active | All above |
| 05 | Database Schema Design | ⏳ Next | SAD §7 |
| 06 | API Contract Specification (OpenAPI) | ⏳ Next | SAD §8 |
| 07 | Development Sprint Plan | ⏳ Pending | SAD approved |

### 1.4 How to Read This Document

- Read §2 (Architecture Goals) first — every decision references these goals
- Read §3 (System Context) to understand external boundaries
- Read §4 (High-Level Architecture) for the 30,000-ft view
- Read §5–§12 for component-level detail on each system layer
- Consult §13 (ADRs) when a decision seems unusual — it was deliberate
- Refer to §14 (Threat Model) for all security reasoning

---

## 2. Architecture Goals & Constraints

### 2.1 Quality Attribute Goals

| ID | Quality Attribute | Requirement | Rationale |
|---|---|---|---|
| QA-01 | Performance | API p95 latency: /generate < 2s, /extract < 1s, /convert < 10s | Developer experience — slow APIs kill adoption immediately |
| QA-02 | Reliability | 99.5% uptime (SLA), 99.9% for enterprise tier | Document infrastructure must be more reliable than the documents it produces |
| QA-03 | Security | Zero-trust internals; all secrets in vault; no hardcoded credentials; SOC2-ready | Documents contain financial and PII data — breach is existential |
| QA-04 | Scalability | Horizontal scale to 100K conversions/hour without architecture change | Enterprise batch migration is the key revenue event |
| QA-05 | Maintainability | Solo developer can understand, modify, and deploy any component in < 1 day | Single-person team — complexity is the primary risk |
| QA-06 | Observability | Every request traceable end-to-end; all errors surface within 60 seconds | Cannot fix what you cannot see |
| QA-07 | Correctness | Format spec compliance validated on every write; round-trip lossless | A document format that produces invalid documents has zero value |
| QA-08 | Cost Efficiency | Infra cost < $500/month until $5K MRR; Claude API cost < 20% of revenue | Solo bootstrap — every dollar of infra is a dollar less runway |

### 2.2 Hard Constraints

> **CONSTRAINT 1 — Solo developer team:** Architecture complexity is capped by what one person + Claude Code can build and maintain. Every component must be understandable in isolation. No clever abstractions.
>
> **CONSTRAINT 2 — Bootstrap budget ($25K Year 1):** No expensive managed services until revenue justifies them. Every infrastructure choice must have a free tier or sub-$50/month starting cost.
>
> **CONSTRAINT 3 — Claude API dependency:** The intelligence layer requires Claude API. Architecture must handle Claude unavailability gracefully — falling back to heuristic extraction, never returning a 500 to the user.
>
> **CONSTRAINT 4 — SPDF spec compliance:** Every document written by the system MUST pass the SPDF validator. This is non-negotiable. Invalid documents shipped = format credibility destroyed.
>
> **CONSTRAINT 5 — No vendor lock-in for core data:** All SPDF files must be portable to any storage backend. Database schemas use standard PostgreSQL features only.

### 2.3 Architecture Principles

| Principle | Statement | Implication |
|---|---|---|
| Simple First | Choose the simplest solution that meets the requirement today | Monolith before microservices; one DB before many; managed before self-hosted |
| Async by Default | All long-running operations (conversion, signing) are async with job queues | Client never waits > 2s; all heavyweight work runs in background workers |
| Fail Loud | Errors surface immediately with full context; no silent failures | Structured logging on every operation; Sentry on every uncaught exception |
| Immutable Artifacts | SPDF files once written are never mutated in place; new versions get new IDs | Enables safe caching, audit trails, and CDN distribution without invalidation |
| Defense in Depth | Security at every layer: network, app, data, format | Assume each layer will be breached; design so breach does not propagate |
| Infrastructure as Code | Every infrastructure resource is defined in version-controlled code | Reproducible environments; no undocumented manual changes; disaster recovery |
| Twelve-Factor Compliant | Services follow 12-factor app methodology | Config in env; stateless processes; disposable containers; dependency isolation |

---

## 3. System Context

### 3.1 Context Diagram (C4 Level 1)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           EXTERNAL ACTORS                               │
├─────────────┬──────────────┬──────────────┬───────────────┬─────────────┤
│  Developer  │  AP Team /   │  Enterprise  │   Freelancer  │   System /  │
│  (SDK/API)  │  Finance Mgr │     CFO      │    / SMB      │   ERP Bot   │
└──────┬──────┴───────┬──────┴──────┬───────┴───────┬───────┴──────┬──────┘
       │              │             │               │              │
       ▼              ▼             ▼               ▼              ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    ╔══════════════════════╗                            │
│                    ║    SPDF PLATFORM     ║                            │
│                    ║  (This System)       ║                            │
│                    ╚══════════════════════╝                            │
└──────────────────────────┬─────────────────────────────────────────────┘
                           │
            ┌──────────────┼──────────────────────┐
            ▼              ▼                      ▼
┌────────────┐   ┌──────────────────┐   ┌──────────────────┐
│ Anthropic  │   │   Cloudflare R2  │   │    Stripe API    │
│ Claude API │   │ (Object Storage) │   │  (Payments)      │
└────────────┘   └──────────────────┘   └──────────────────┘
    ▼                      ▼                      ▼
┌────────────┐   ┌──────────────────┐   ┌──────────────────┐
│   Clerk    │   │   Resend API     │   │  Sentry / Axiom  │
│  (Auth)    │   │   (Email)        │   │  (Observability) │
└────────────┘   └──────────────────┘   └──────────────────┘
```

### 3.2 External Dependencies

| System | Role | Protocol | Failure Mode | Fallback |
|---|---|---|---|---|
| Anthropic Claude API | AI intelligence for PDF→SPDF conversion | HTTPS REST | Queue job; retry 3x; fall back to heuristic extraction | Heuristic parser (60-70% accuracy) |
| Cloudflare R2 | Primary file storage | S3-compatible HTTPS | Retry with exponential backoff; alert on persistent failure | Local disk buffer (temporary) |
| Supabase (PostgreSQL) | Relational database for all metadata, users, jobs, billing | TCP PostgreSQL wire | Connection pool retry; circuit breaker | Read-only cached responses where possible |
| Upstash (Redis) | Job queue, distributed cache, rate limit counters | HTTPS Redis over TLS | Queue falls back to synchronous; cache miss is acceptable | Skip cache; process inline |
| Clerk | User authentication, session management | HTTPS REST + webhook | Return 503 with retry-after | No fallback — auth is non-bypassable |
| Stripe | Payment processing, subscription management | HTTPS REST + webhook | Queue webhook retries | Allow grace period for existing subscribers |
| Resend | Transactional email | HTTPS REST | Queue email retries; non-blocking | Log email content; retry on next worker cycle |
| Sentry | Error tracking and alerting | HTTPS REST | Log to stdout; buffer locally | Stdout logging (always on) |

---

## 4. High-Level Architecture

### 4.1 Architecture Style: Modular Monolith → Selective Services

> **Phase 1 (MVP — Month 1–6): Modular Monolith** — Single deployable Python FastAPI application with clear internal module boundaries. All modules share one database. One Railway deployment. Zero inter-service network calls. Why: Simplicity wins. A solo developer cannot debug distributed systems under pressure.
>
> **Phase 2 (Scale — Month 7–18): Extract Conversion Service** — The Claude-powered conversion pipeline is extracted into a dedicated worker service. Reason: Conversion is CPU/memory-heavy, long-running, and has different scaling needs. Everything else stays in the monolith.
>
> **Phase 3 (Enterprise — Month 18+): Extract as needed** — Only extract additional services when a specific scaling or isolation need is proven by production data. Never split for architectural purity alone.

### 4.2 Component Overview Diagram (C4 Level 2)

```
┌──────────────────────────────────────────────────────────────────────────┐
│                           SPDF PLATFORM                                  │
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │                    EDGE / CDN LAYER                              │    │
│  │   Cloudflare CDN → DDoS protection, TLS termination, WAF        │    │
│  └───────────────────────────┬─────────────────────────────────────┘    │
│                              │                                           │
│        ┌─────────────────────┼──────────────────────┐                  │
│        ▼                     ▼                       ▼                  │
│  ┌───────────┐   ┌────────────────────┐   ┌────────────────────┐       │
│  │   SPDF    │   │    SPDF BACKEND    │   │   SPDF CONVERSION  │       │
│  │  STUDIO   │   │    API (FastAPI)   │   │   WORKER SERVICE   │       │
│  │  (React)  │   │                   │   │   (Python+Celery)  │       │
│  │  Vercel   │   │  ┌──────────────┐ │   │                    │       │
│  │           │   │  │ Auth Layer   │ │   │  ┌──────────────┐  │       │
│  │  WASM     │   │  ├──────────────┤ │   │  │ Claude API   │  │       │
│  │  spdf-    │   │  │ Document API │ │   │  │ Integration  │  │       │
│  │  core     │   │  │ Jobs API     │ │   │  ├──────────────┤  │       │
│  │           │   │  │ Billing API  │ │   │  │ Heuristic    │  │       │
│  │           │   │  │ Admin API    │ │   │  │ Fallback     │  │       │
│  │           │   │  └──────────────┘ │   │  └──────────────┘  │       │
│  └───────────┘   └─────────┬─────────┘   └────────┬───────────┘       │
│                            │                       │                   │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                    DATA & STORAGE LAYER                          │  │
│  │  ┌────────────┐ ┌────────────┐ ┌───────────┐ ┌──────────────┐  │  │
│  │  │ Supabase   │ │ Upstash    │ │Cloudflare │ │    Doppler   │  │  │
│  │  │ PostgreSQL │ │   Redis    │ │    R2     │ │   (Secrets)  │  │  │
│  │  └────────────┘ └────────────┘ └───────────┘ └──────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────┘

SPDF Core Engine (Rust / WASM) runs in:
• Backend API (as Python extension via PyO3)
• Conversion Worker (as Python extension via PyO3)
• Studio Browser (as WASM module)
```

### 4.3 Request Flow Overview

| Request Type | Entry Point | Path | Response Time |
|---|---|---|---|
| Generate SPDF (simple) | POST /v1/generate | API → Core Engine (PyO3) → R2 → Response | < 2s synchronous |
| Convert PDF (AI) | POST /v1/convert | API → Queue → Worker → Claude API → Core Engine → R2 → Webhook | < 30s async (webhook) |
| Extract data | POST /v1/extract | API → R2 (fetch) → Core Engine → Response | < 1s synchronous |
| Studio upload | PUT /v1/upload | API → R2 → Queue → Worker → signed URL → Studio | < 5s to view |
| Sign document | POST /v1/sign | API → R2 (fetch) → Core Engine (sign) → R2 (write) → Response | < 3s synchronous |
| Validate | POST /v1/validate | API → R2 (fetch) → Core Engine (validate) → Response | < 500ms synchronous |
| Studio render | (Browser) | WASM loads spdf file → Core Engine WASM → Canvas render | Client-side only |

---

## 5. Component Architecture

### 5.1 SPDF Core Engine (Rust)

#### 5.1.1 Responsibilities

The Core Engine is the heart of the entire system. It is the single authoritative implementation of the SPDF specification.

- Parse any .spdf container into a typed, validated in-memory DOM
- Write any in-memory DOM to a conformant .spdf container
- Generate a PDF 2.0 render layer from any SPDF DOM
- Perform element-level semantic diff between two SPDF DOMs
- Apply and verify X.509 cryptographic signatures
- Manage the audit log (append entries, verify hash chain)
- Perform cryptographic erasure (redaction) with proof generation

#### 5.1.2 Internal Module Structure

```
crates/
├── spdf-core/
│   ├── src/
│   │   ├── lib.rs                ← Public API surface
│   │   ├── container/            ← ZIP I/O, manifest, checksum
│   │   │   ├── reader.rs
│   │   │   ├── writer.rs
│   │   │   └── validator.rs
│   │   ├── dom/                  ← Document Object Model
│   │   │   ├── document.rs
│   │   │   ├── elements/
│   │   │   │   ├── content.rs    ← Heading, Paragraph, Table, Image
│   │   │   │   ├── domain.rs     ← InvoiceHeader, LineItem, PaymentTerms
│   │   │   │   └── trust.rs      ← SignatureBlock, Stamp, Redaction
│   │   │   └── validator.rs
│   │   ├── layout/               ← Layout engine
│   │   │   ├── engine.rs
│   │   │   ├── text.rs           ← UAX #14 line breaking
│   │   │   └── pagination.rs
│   │   ├── style/                ← Style resolution
│   │   │   ├── cascade.rs        ← 5-level style cascade algorithm
│   │   │   └── tokens.rs
│   │   ├── render/               ← PDF render layer generation
│   │   │   ├── pdf_writer.rs     ← lopdf-based PDF 2.0 output
│   │   │   ├── glyph.rs
│   │   │   └── image.rs
│   │   ├── security/             ← Cryptographic operations
│   │   │   ├── signing.rs        ← RSA-PSS-SHA256 sign + verify
│   │   │   ├── audit.rs          ← Audit log append + chain verify
│   │   │   ├── redaction.rs
│   │   │   └── canonical.rs      ← RFC 8785 JCS canonical form
│   │   ├── assets/               ← Asset management
│   │   │   ├── registry.rs
│   │   │   ├── font.rs
│   │   │   └── svg.rs
│   │   └── error.rs              ← Error types (all 37 error codes)
│   └── Cargo.toml
├── spdf-wasm/                    ← WASM bindings for browser
└── spdf-python/                  ← PyO3 bindings for Python SDK + API
```

#### 5.1.3 Key Design Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Memory model | Document is immutable once parsed; all modifications create new document | Prevents race conditions; enables safe caching |
| Error handling | `Result<T, SpdfError>` everywhere; no panics on valid input | Panics in a library crash the caller |
| Serialization | serde + serde_json for all JSON; custom serializer for canonical form | serde is the Rust ecosystem standard |
| PDF rendering | lopdf for PDF object model; fontdue for glyph metrics | Both are pure Rust, no C dependencies |
| Testing strategy | Unit tests per module; golden-file rendering tests; fuzz testing on parser | Parser correctness is critical |

### 5.2 Backend API Service (FastAPI)

#### 5.2.1 Responsibilities

- Authenticate all incoming requests via Clerk JWT verification
- Enforce rate limits per user and tier using Redis counters
- Route requests to the correct handler module
- Call the Core Engine via PyO3 bindings for synchronous operations
- Enqueue jobs to Redis/Celery queue for async operations
- Read/write SPDF files to Cloudflare R2
- Persist document metadata and job records to PostgreSQL
- Emit structured logs and metrics to Axiom/Sentry

#### 5.2.2 Module Structure

```
services/api/
├── main.py                       ← FastAPI app factory, middleware registration
├── config.py                     ← Pydantic Settings (all config from env via Doppler)
├── dependencies.py               ← FastAPI DI: db, redis, storage, current_user
├── routers/                      ← One router file per resource domain
│   ├── documents.py              ← /v1/documents CRUD + state transitions
│   ├── convert.py                ← /v1/convert (async, returns job_id)
│   ├── generate.py               ← /v1/generate (sync for simple, async for complex)
│   ├── extract.py                ← /v1/extract (sync)
│   ├── sign.py                   ← /v1/sign, /v1/verify
│   ├── jobs.py                   ← /v1/jobs/{id} status polling
│   ├── templates.py              ← /v1/templates CRUD
│   ├── billing.py                ← /v1/billing (Stripe integration)
│   └── webhooks.py               ← /v1/webhooks (Clerk, Stripe)
├── services/                     ← Business logic, no HTTP concerns
│   ├── document_service.py
│   ├── storage_service.py
│   ├── job_service.py
│   └── billing_service.py
├── models/                       ← SQLAlchemy ORM models
├── schemas/                      ← Pydantic request/response schemas
├── middleware/
│   ├── auth.py                   ← Clerk JWT verification
│   ├── rate_limit.py             ← Redis-backed rate limiting
│   ├── request_id.py
│   └── logging.py                ← Structured JSON logging per request
├── core/
│   ├── spdf_engine.py            ← Python wrapper around spdf_native module
│   └── spdf_native.pyd           ← Compiled PyO3 binary
├── migrations/                   ← Alembic database migrations
└── tests/
```

#### 5.2.3 Request Processing Pipeline

```
Incoming HTTP Request
        │
        ▼
[Middleware: request_id.py]     → Inject X-Request-ID (UUID4)
        │
        ▼
[Middleware: logging.py]        → Log request: method, path, user-agent, request_id
        │
        ▼
[Middleware: auth.py]           → Verify Clerk JWT → extract user_id, org_id, tier
        │  (401 if invalid)
        ▼
[Middleware: rate_limit.py]     → Check Redis counter: user_id + endpoint
        │  (429 if exceeded)
        ▼
[Router: route match]           → Identify handler, validate path params
        │
        ▼
[Handler function]              → Validate request body (Pydantic)
        │  (422 if invalid)
        ▼
[Service layer]                 → Business logic, DB operations, Core Engine calls
        │
        ▼
[Response]                      → Pydantic response model → JSON
        │
        ▼
[Middleware: logging.py]        → Log response: status_code, duration_ms, request_id
```

### 5.3 Conversion Worker Service (Celery)

#### 5.3.1 Responsibilities

- Execute Claude API calls for PDF→SPDF semantic extraction
- Run heuristic fallback extraction when Claude is unavailable
- Generate render.pdf layers from semantic content
- Assemble complete SPDF containers from converted components
- Handle bulk conversion batches (enterprise migration use case)
- Retry failed jobs with exponential backoff
- Report job completion via webhook or polling-ready status update

#### 5.3.2 Structure

```
services/worker/
├── celery_app.py
├── tasks/
│   ├── conversion.py             ← pdf_to_spdf_task
│   ├── generation.py             ← generate_from_template_task
│   ├── batch.py                  ← bulk_convert_task
│   └── maintenance.py            ← cleanup_expired_jobs_task
├── intelligence/
│   ├── claude_extractor.py       ← Claude API integration
│   ├── heuristic_extractor.py    ← Rule-based fallback
│   └── confidence_scorer.py
└── pipeline/
    ├── pdf_parser.py
    ├── assembler.py
    └── render_generator.py
```

#### 5.3.3 Conversion Pipeline — Detailed Flow

```
INPUT: job_id, user_id, r2_key (uploaded PDF)

STEP 1: RETRIEVE
├── Download PDF from R2 by r2_key
├── Validate PDF is a valid PDF (magic bytes + header)
└── Update job status: PROCESSING

STEP 2: TEXT EXTRACTION (pdfplumber)
├── Extract text blocks with positions (x, y, width, height, font, size)
├── Extract images as PNG assets
└── Output: structured text_blocks[], images[], page_info[]

STEP 3: SEMANTIC CLASSIFICATION (Claude API)
├── Build prompt: system_prompt + page_text_blocks (serialized JSON)
├── Call claude-sonnet-4-6 with max_tokens=4096
├── Parse response: element_tree[] with type, text, properties
├── On failure: retry up to 3 times (exponential backoff: 1s, 2s, 4s)
└── On persistent failure: activate HEURISTIC FALLBACK (Step 3b)

STEP 3b: HEURISTIC FALLBACK (if Claude unavailable)
├── Font size > 14pt → Heading
├── Positional grid alignment → Table detection
├── Currency regex + alignment → Financial field detection
└── Confidence scores: 0.5–0.7 (lower than Claude baseline)

STEP 4: DOM ASSEMBLY
├── Assign element IDs (EID format per spec §2.4.1)
├── Build parent-child relationships from positions
└── Compute layout.json from PDF coordinates

STEP 5: VALIDATION
├── Validate DOM against SPDF schema via Core Engine
├── On E_ errors: attempt auto-correction
└── On F_ errors: fail job with detailed error report

STEP 6: CONTAINER ASSEMBLY
├── Use original PDF as render.pdf (visual fidelity guaranteed)
├── Embed extracted images as assets (content-addressed)
├── Write manifest.json with checksums
├── Write initial audit.json (CREATED + CONVERTED_FROM_PDF)
└── Write metadata.json

STEP 7: UPLOAD
├── Upload .spdf container to R2
├── Update document record in PostgreSQL
├── Update job status: COMPLETED with spdf_document_id
└── Trigger webhook if configured

OUTPUT: spdf_document_id, r2_key, confidence_report, job_status: COMPLETED
```

### 5.4 SPDF Studio (React + WASM)

#### 5.4.1 Responsibilities

- Provide a visual PDF/SPDF upload interface for non-developer users
- Render SPDF documents visually in the browser using the WASM engine
- Show the element tree alongside the visual render (split view)
- Allow property inspection of any selected element
- Provide the document signing flow
- Handle user authentication via Clerk Components
- Manage subscription upgrades via Stripe Checkout

#### 5.4.2 Front-End Architecture

```
apps/studio/
├── src/
│   ├── main.tsx
│   ├── routes/
│   │   ├── index.tsx             ← Landing / dashboard
│   │   ├── documents/            ← Document list, upload, view
│   │   ├── templates/            ← Template designer (Phase 2)
│   │   ├── settings/             ← Account, billing, API keys
│   │   └── auth/                 ← Clerk auth pages
│   ├── components/
│   │   ├── viewer/               ← SPDF viewer (WASM-powered)
│   │   │   ├── SPDFViewer.tsx    ← Canvas renderer using WASM
│   │   │   ├── ElementTree.tsx   ← DOM inspector sidebar
│   │   │   └── PropertyPanel.tsx ← Selected element properties
│   │   ├── upload/               ← Drag-drop upload with progress
│   │   ├── signing/              ← Signature capture (canvas)
│   │   └── billing/              ← Plan selection, Stripe Checkout trigger
│   ├── lib/
│   │   ├── spdf-wasm.ts          ← WASM module loader and wrapper
│   │   ├── api-client.ts         ← Typed fetch client for all API calls
│   │   └── auth.ts               ← Clerk hooks and auth state
│   ├── stores/                   ← Zustand state stores
│   └── hooks/                    ← Custom React hooks
├── public/
│   └── spdf_core.wasm            ← Compiled WASM binary (CI artifact)
├── vite.config.ts
└── tailwind.config.ts
```

### 5.5 Python SDK (Developer-Facing)

```
packages/spdf-python/
├── spdf/
│   ├── __init__.py               ← Public API: Document, Page, elements, styles
│   ├── document.py               ← Document class: open(), create(), export.*
│   ├── page.py
│   ├── elements/
│   │   ├── content.py            ← Heading, Paragraph, Table, Image, etc.
│   │   ├── domain.py             ← InvoiceHeader, LineItemTable, PaymentTerms
│   │   └── trust.py              ← SignatureBlock, Stamp
│   ├── template.py               ← Template: load(), bind(data: dict) → Document
│   ├── styles.py
│   ├── signing.py
│   ├── extract.py
│   ├── exceptions.py             ← SpdfError hierarchy (maps Rust error codes)
│   └── _native.pyd               ← PyO3 binary (installed by pip, not committed)
├── tests/
├── pyproject.toml
└── README.md
```

### 5.6 TypeScript SDK (Developer-Facing)

```
packages/spdf-js/
├── src/
│   ├── index.ts                  ← Public exports
│   ├── document.ts               ← Document class (async — all ops return Promise)
│   ├── elements/                 ← TypeScript element classes
│   ├── wasm-loader.ts            ← Lazy WASM module initialization
│   ├── node-adapter.ts           ← Node.js-specific I/O
│   └── browser-adapter.ts        ← Browser-specific I/O
├── dist/                         ← Build output (ESM + CJS + types)
├── package.json
└── tsconfig.json
```

---

## 6. Data Architecture

### 6.1 Data Storage Strategy

| Data Type | Storage | Rationale | Access Pattern |
|---|---|---|---|
| SPDF document files (.spdf) | Cloudflare R2 (object storage) | Binary blobs — R2 is S3-compatible, zero egress fees | Write once, read many via signed URL |
| Document metadata, ownership | PostgreSQL (Supabase) | Relational data — who owns what, document state machine | Frequent reads by user_id and document_id |
| User accounts, orgs | PostgreSQL + Clerk (authoritative) | Clerk owns auth; Postgres mirrors profile data needed for business logic | Read on every API call |
| Conversion jobs, status | PostgreSQL + Redis | Postgres for durability; Redis for real-time polling | Write on job create; read on polling |
| Rate limit counters | Redis (Upstash) | Must be fast; TTL-based; loss on restart acceptable | Increment + read on every API call |
| API response cache | Redis (Upstash) | Cache /extract results by document hash; 1-hour TTL | Read-through cache pattern |
| Secrets (API keys, DB URLs) | Doppler (secrets manager) | Never in env files, never in git, never in code | Injected at deploy time only |
| Session / auth tokens | Clerk (managed) | Clerk manages JWT lifecycle | Validated on every request by middleware |

### 6.2 PostgreSQL Schema Design

```sql
-- ═══════════════════════════════════════════════════════════════════════
-- SPDF DATABASE SCHEMA v1.0
-- PostgreSQL 15+ │ Supabase hosted
-- ═══════════════════════════════════════════════════════════════════════

-- EXTENSIONS
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gin";

-- ═══════════════════════════════════════════
-- USERS & ORGANIZATIONS
-- ═══════════════════════════════════════════
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    clerk_user_id   TEXT UNIQUE NOT NULL,
    email           TEXT UNIQUE NOT NULL,
    display_name    TEXT,
    tier            TEXT NOT NULL DEFAULT 'FREE'
                    CHECK (tier IN ('FREE','PRO','TEAM','ENTERPRISE')),
    api_key_hash    TEXT,                       -- bcrypt hash of active API key
    api_key_prefix  TEXT,                       -- first 8 chars for display
    monthly_quota   INTEGER NOT NULL DEFAULT 10,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ                 -- soft delete
);

CREATE TABLE organizations (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    clerk_org_id    TEXT UNIQUE NOT NULL,
    name            TEXT NOT NULL,
    tier            TEXT NOT NULL DEFAULT 'TEAM'
                    CHECK (tier IN ('TEAM','ENTERPRISE')),
    monthly_quota   INTEGER NOT NULL DEFAULT 1000,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE org_members (
    org_id          UUID REFERENCES organizations(id) ON DELETE CASCADE,
    user_id         UUID REFERENCES users(id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('owner','admin','member','viewer')),
    joined_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (org_id, user_id)
);

-- ═══════════════════════════════════════════
-- DOCUMENTS
-- ═══════════════════════════════════════════
CREATE TABLE documents (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    spdf_doc_id     TEXT UNIQUE NOT NULL,
    owner_user_id   UUID REFERENCES users(id),
    owner_org_id    UUID REFERENCES organizations(id),
    title           TEXT NOT NULL,
    document_type   TEXT NOT NULL,
    state           TEXT NOT NULL DEFAULT 'DRAFT'
                    CHECK (state IN ('DRAFT','REVIEW','SIGNED','CERTIFIED')),
    spdf_version    TEXT NOT NULL DEFAULT '1.0',
    r2_key          TEXT NOT NULL,              -- Cloudflare R2 object key
    r2_bucket       TEXT NOT NULL DEFAULT 'spdf-documents',
    file_size_bytes BIGINT,
    page_count      INTEGER,
    locale          TEXT NOT NULL DEFAULT 'en-US',
    confidence_score DECIMAL(4,3),              -- 0.000-1.000, NULL if not AI-converted
    source_format   TEXT DEFAULT 'NATIVE'
                    CHECK (source_format IN ('NATIVE','PDF_CONVERTED')),
    is_template     BOOLEAN NOT NULL DEFAULT FALSE,
    template_id     UUID REFERENCES documents(id),
    metadata        JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_documents_owner_user ON documents(owner_user_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_documents_owner_org  ON documents(owner_org_id)  WHERE deleted_at IS NULL;
CREATE INDEX idx_documents_state      ON documents(state)         WHERE deleted_at IS NULL;
CREATE INDEX idx_documents_title_trgm ON documents USING GIN (title gin_trgm_ops);
CREATE INDEX idx_documents_metadata   ON documents USING GIN (metadata);

-- ═══════════════════════════════════════════
-- CONVERSION JOBS
-- ═══════════════════════════════════════════
CREATE TABLE conversion_jobs (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID REFERENCES users(id) ON DELETE SET NULL,
    org_id          UUID REFERENCES organizations(id) ON DELETE SET NULL,
    job_type        TEXT NOT NULL
                    CHECK (job_type IN ('PDF_TO_SPDF','GENERATE','SIGN','BATCH')),
    status          TEXT NOT NULL DEFAULT 'QUEUED'
                    CHECK (status IN ('QUEUED','PROCESSING','COMPLETED','FAILED','CANCELLED')),
    input_r2_key    TEXT,
    output_doc_id   UUID REFERENCES documents(id),
    template_id     UUID REFERENCES documents(id),
    input_data      JSONB,
    result_data     JSONB,                       -- confidence report, errors
    error_code      TEXT,
    error_message   TEXT,
    celery_task_id  TEXT UNIQUE,
    attempt_count   SMALLINT NOT NULL DEFAULT 0,
    max_attempts    SMALLINT NOT NULL DEFAULT 3,
    webhook_url     TEXT,
    webhook_secret  TEXT,
    queued_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at      TIMESTAMPTZ,
    completed_at    TIMESTAMPTZ,
    expires_at      TIMESTAMPTZ DEFAULT NOW() + INTERVAL '7 days'
);

-- ═══════════════════════════════════════════
-- BILLING & USAGE
-- ═══════════════════════════════════════════
CREATE TABLE subscriptions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID REFERENCES users(id),
    org_id          UUID REFERENCES organizations(id),
    stripe_sub_id   TEXT UNIQUE,
    stripe_cust_id  TEXT,
    plan            TEXT NOT NULL,
    status          TEXT NOT NULL
                    CHECK (status IN ('active','trialing','past_due','cancelled')),
    current_period_start TIMESTAMPTZ,
    current_period_end   TIMESTAMPTZ,
    cancel_at       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE usage_events (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID REFERENCES users(id),
    org_id          UUID REFERENCES organizations(id),
    event_type      TEXT NOT NULL
                    CHECK (event_type IN ('conversion','generation','extraction','api_call')),
    document_id     UUID REFERENCES documents(id),
    job_id          UUID REFERENCES conversion_jobs(id),
    units           INTEGER NOT NULL DEFAULT 1,
    billable        BOOLEAN NOT NULL DEFAULT TRUE,
    billed          BOOLEAN NOT NULL DEFAULT FALSE,
    occurred_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_usage_user_month ON usage_events(user_id, occurred_at)
    WHERE billed = FALSE;
```

### 6.3 R2 Object Storage Layout

```
# Cloudflare R2 Bucket Structure

spdf-documents/
├── users/{user_id}/documents/{doc_id}/{doc_id}.spdf
├── users/{user_id}/documents/{doc_id}/{doc_id}_v2.spdf   # versioned
├── orgs/{org_id}/documents/{doc_id}/{doc_id}.spdf
└── templates/{template_id}/{template_id}.spdf

spdf-uploads/   (temporary, lifecycle rule: delete after 48h)
└── uploads/{job_id}/{original_filename}.pdf

# R2 Object Metadata (stored on every object)
{
  "x-spdf-document-id": "<spdf_doc_id>",
  "x-spdf-owner-user":  "<user_id>",
  "x-spdf-doc-state":   "DRAFT",
  "x-spdf-version":     "1.0",
  "Content-Type":       "application/spdf",
  "Cache-Control":      "private, max-age=3600"
}

# Signed URL config (for client-side download)
# TTL: 15 minutes for one-time downloads
# TTL: 1 hour for Studio viewer sessions
# Documents never served via public URL (always signed)
```

### 6.4 Redis Data Model

```
# ── RATE LIMITING ──────────────────────────────────────────────────────────
KEY:   ratelimit:{user_id}:{endpoint_slug}
TYPE:  string (integer counter)
TTL:   window_seconds

# ── JOB STATUS CACHE ───────────────────────────────────────────────────────
KEY:   job:status:{job_id}
TYPE:  hash
TTL:   3600 (1 hour after completion)
VALUE: { "status": "PROCESSING", "progress": "42", "step": "claude_extract" }

# ── EXTRACT RESPONSE CACHE ─────────────────────────────────────────────────
KEY:   cache:extract:{semantic_sha256}
TYPE:  string (JSON blob)
TTL:   3600
# Key is the SHA-256 of semantic.json — same file = same result

# ── IDEMPOTENCY KEYS ───────────────────────────────────────────────────────
KEY:   idempotency:{user_id}:{idempotency_key}
TYPE:  string (response body JSON)
TTL:   86400 (24 hours)
# Prevents double-charging on network retry of POST requests
```

---

## 7. API Architecture

### 7.1 API Design Principles

| Principle | Implementation |
|---|---|
| REST over resources | URLs identify resources; HTTP verbs express actions |
| Versioning in URL path | /v1/ prefix on all endpoints; breaking changes get /v2/ |
| Always return structured errors | All errors return `{error: {code, message, details, request_id}}` |
| Idempotency for mutations | POST endpoints accept X-Idempotency-Key; 24-hour dedup |
| Async for long operations | Any operation > 5s returns 202 Accepted with job_id |
| Pagination on all list endpoints | cursor-based pagination; default 20 per page, max 100 |
| OpenAPI 3.1 spec is authoritative | Code is generated from spec; spec is versioned in git |

### 7.2 Authentication & Authorization

```
# TWO AUTH PATHS supported simultaneously:

# PATH 1: JWT Bearer (Studio users, OAuth apps)
Authorization: Bearer eyJhbGciOiJSUzI1NiIsIn...  (Clerk-issued JWT)

# PATH 2: API Key (developer SDK/scripts)
Authorization: Bearer sk_live_abc123xyz...
# Or: X-API-Key: sk_live_abc123xyz...

# API Key format:
# sk_live_{26 random base62 chars}    ← production
# sk_test_{26 random base62 chars}    ← test mode (no billing)

# Authorization Matrix:
# ┌─────────────────────┬──────┬──────┬───────┬──────────┐
# │ Resource            │ OWN  │ ORG  │ ADMIN │ PUBLIC   │
# ├─────────────────────┼──────┼──────┼───────┼──────────┤
# │ Own documents       │ CRUD │  -   │  R    │   -      │
# │ Org documents       │ -    │ CR   │ CRUD  │   -      │
# │ Public templates    │ R    │  R   │  R    │   R      │
# │ Own templates       │ CRUD │  -   │ CRUD  │   -      │
# │ Jobs (own)          │ CR   │  -   │  R    │   -      │
# │ Billing             │ RU   │  -   │  R    │   -      │
# └─────────────────────┴──────┴──────┴───────┴──────────┘
```

### 7.3 Rate Limiting

| Tier | /v1/convert | /v1/generate | /v1/extract | Other endpoints | Limit window |
|---|---|---|---|---|---|
| FREE | 10/day | 50/day | 100/day | 500/day | Per calendar day UTC |
| PRO | 1,000/day | 5,000/day | 10,000/day | 50,000/day | Per calendar day UTC |
| TEAM | 10,000/day | 50,000/day | 100,000/day | 500,000/day | Per calendar day UTC |
| ENTERPRISE | Custom | Custom | Custom | Custom | Contractual |

### 7.4 Standard Error Response Format

```json
{
  "error": {
    "code":       "DOCUMENT_NOT_FOUND",
    "message":    "Document not found or you do not have access to it",
    "details":    {
      "document_id": "spdf-abc123..."
    },
    "request_id": "req_a1b2c3d4e5f6",
    "docs_url":   "https://docs.spdf.dev/errors#DOCUMENT_NOT_FOUND"
  }
}

# HTTP Status → Error class mapping:
# 400  → VALIDATION_ERROR, INVALID_INPUT, MALFORMED_JSON
# 401  → AUTH_REQUIRED, INVALID_TOKEN, EXPIRED_TOKEN
# 403  → PERMISSION_DENIED, TIER_REQUIRED
# 404  → DOCUMENT_NOT_FOUND, JOB_NOT_FOUND
# 409  → DOCUMENT_ALREADY_SIGNED, STATE_TRANSITION_INVALID
# 422  → SPDF_VALIDATION_FAILED, SCHEMA_ERROR
# 429  → RATE_LIMIT_EXCEEDED, QUOTA_EXCEEDED
# 500  → INTERNAL_ERROR (never expose stack traces)
# 503  → SERVICE_DEGRADED (with Retry-After header)
```

### 7.5 Core Endpoint Specifications

#### Document Endpoints

| Method | Path | Auth | Sync? | Description |
|---|---|---|---|---|
| POST | /v1/documents/upload | Required | Yes (202) | Upload PDF for conversion — returns upload_url + job_id |
| POST | /v1/documents/generate | Required | Yes (201) | Generate SPDF from template + data payload |
| GET | /v1/documents | Required | Yes | List user/org documents (cursor pagination) |
| GET | /v1/documents/{id} | Required | Yes | Get document metadata and download URL |
| DELETE | /v1/documents/{id} | Required | Yes | Soft-delete document (state must be DRAFT) |
| POST | /v1/documents/{id}/extract | Required | Yes (< 1s) | Extract structured data from SPDF document |
| POST | /v1/documents/{id}/sign | Required | Yes (< 3s) | Apply X.509 signature → transition to SIGNED |
| POST | /v1/documents/{id}/verify | Required | Yes (< 500ms) | Verify document signature integrity |
| POST | /v1/documents/{id}/validate | Required | Yes (< 500ms) | Validate against SPDF spec |
| POST | /v1/documents/{id}/redact | Required | Yes (< 2s) | Cryptographically erase specified elements |
| GET | /v1/documents/{id}/diff/{id2} | Required | Yes (< 3s) | Semantic diff between two documents |

#### Job Endpoints

| Method | Path | Description |
|---|---|---|
| GET | /v1/jobs/{id} | Poll job status; returns progress + result when complete |
| DELETE | /v1/jobs/{id} | Cancel a QUEUED job (cannot cancel PROCESSING) |
| GET | /v1/jobs | List user jobs, filter by status (cursor paginated) |

---

## 8. Infrastructure Architecture

### 8.1 Infrastructure Stack Decision

| Layer | Service | Cost (MVP) | Cost (Growth) | Why chosen |
|---|---|---|---|---|
| API hosting | Railway.app | $5–20/mo | $50–150/mo | Git-push deploys; scales containers automatically; zero ops overhead |
| Worker hosting | Railway.app (2nd service) | $5–15/mo | $30–100/mo | Same platform as API; scale workers independently |
| Frontend | Vercel | $0/mo | $20/mo | Purpose-built for React/Vite; edge CDN globally |
| Database | Supabase PostgreSQL | $0–25/mo | $25–100/mo | Managed Postgres with pgbouncer; real-time events |
| Cache + Queue | Upstash Redis | $0–20/mo | $20–100/mo | Serverless Redis; per-request billing; Celery-compatible |
| File storage | Cloudflare R2 | $0/mo* | $5–50/mo | **Zero egress fees**; S3-compatible; global CDN |
| CDN + DDoS | Cloudflare Free | $0/mo | $0/mo | World-class DDoS protection; WAF; SSL; DNS |
| Secrets | Doppler | $0/mo | $0/mo | Team secret management; environment-aware; audit logs |
| Auth | Clerk | $0–25/mo | $25–100/mo | Complete auth: social, magic links, enterprise SSO |
| Email | Resend | $0–20/mo | $20–50/mo | Developer-first email API; great deliverability |
| Payments | Stripe | 2.9%+30¢ | 2.9%+30¢ | Industry standard; subscription billing |
| Error tracking | Sentry | $0–26/mo | $26/mo | Stack traces + context; performance monitoring |
| Logs + metrics | Axiom | $0/mo | $25/mo | Structured log search; dashboards; 90-day retention |
| Uptime monitor | BetterUptime | $0/mo | $0/mo | Public status page; SMS+email alerts |

*R2 free tier: 10 GB storage, 10M Class A ops, 1M Class B ops per month.

### 8.2 Environment Strategy

| Environment | Purpose | Deployment | DB | Claude API |
|---|---|---|---|---|
| development | Local dev with hot reload | localhost (Docker Compose) | Local Postgres via Docker | claude-haiku-4-5 (cheapest) |
| preview | Per-PR preview deployment | Railway preview + Vercel preview | Shared dev Supabase | claude-haiku-4-5 |
| staging | Pre-production validation | Railway (separate service) | Separate Supabase project | claude-sonnet-4-6 |
| production | Live system | Railway (primary service) | Production Supabase project | claude-sonnet-4-6 |

### 8.3 CI/CD Pipeline

```yaml
# .github/workflows/ci.yml

name: CI
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  # ── JOB 1: Rust Core Engine ──────────────────────────────────────────────
  rust-core:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: "wasm32-unknown-unknown" }
      - uses: Swatinem/rust-cache@v2
      - name: Run tests
        run: cargo test --workspace --all-features
      - name: Clippy (lint)
        run: cargo clippy -- -D warnings
      - name: Security audit
        run: cargo install cargo-audit && cargo audit
      - name: Build WASM
        run: cd crates/spdf-wasm && wasm-pack build --target web
      - name: Build PyO3
        run: cd crates/spdf-python && maturin build --release

  # ── JOB 2: Python API + Worker ───────────────────────────────────────────
  python-backend:
    runs-on: ubuntu-latest
    needs: rust-core
    services:
      postgres:
        image: postgres:15
        env: { POSTGRES_PASSWORD: test }
      redis:
        image: redis:7
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with: { python-version: "3.12" }
      - name: Lint (ruff)
        run: ruff check . && ruff format --check .
      - name: Type check (mypy)
        run: mypy services/
      - name: Run tests
        run: pytest tests/ -v --cov=services --cov-report=xml

  # ── JOB 3: Frontend ─────────────────────────────────────────────────────
  frontend:
    runs-on: ubuntu-latest
    needs: rust-core
    steps:
      - run: npm ci
      - run: npm run lint
      - run: npm run type-check
      - run: npm run build
      - run: npm run test

  # ── JOB 4: Deploy (main branch only) ────────────────────────────────────
  deploy:
    needs: [rust-core, python-backend, frontend]
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - name: Deploy API to Railway
        run: railway up --service spdf-api
      - name: Deploy Worker to Railway
        run: railway up --service spdf-worker
      - name: Run smoke tests
        run: python scripts/smoke_test.py --env production
```

### 8.4 Rollback Strategy

| Scenario | Detection | Rollback Action | RTO Target |
|---|---|---|---|
| Bad API deploy: 5xx spike | Sentry alert + Axiom error rate | `railway rollback --service spdf-api` (instant) | < 5 minutes |
| DB migration failure | Migration script exits non-zero | `alembic downgrade -1` runs in start command | < 10 minutes |
| Bad worker deploy: job failures | Job failure rate alert in Axiom | `railway rollback --service spdf-worker` | < 5 minutes |
| Core engine regression (rendering) | Visual regression tests fail in CI | CI blocks deploy — regression never reaches production | N/A (prevented) |
| Data corruption | Detected via validate endpoint | Restore from Supabase PITR (point-in-time recovery) | < 1 hour |

---

## 9. Security Architecture

### 9.1 Security Principles

| Principle | Implementation in SPDF |
|---|---|
| Zero Trust Networking | All inter-service communication is authenticated. Services do not trust network position. Every call carries a credential. |
| Least Privilege | Each service has only the IAM permissions it needs. R2 bucket policies are per-service. DB users have per-table grants only. |
| Defense in Depth | Cloudflare WAF → API rate limiting → auth middleware → input validation → schema validation → storage encryption. Five independent layers. |
| Secure by Default | New documents are PRIVATE by default. New API keys have minimum scope. New users get FREE tier rate limits. |
| Fail Secure | If auth check fails → 401. If rate limit check fails → 429. If validation fails → 422. Never fail open. |
| Immutable Audit | All state-changing operations are recorded in hash-chained audit.json inside the SPDF file itself. |

### 9.2 Threat Model (STRIDE)

| Threat | Category | Attack Vector | Control | Residual Risk |
|---|---|---|---|---|
| Stolen API key used by attacker | Spoofing | Key leaked via code commit or breach | Key rotation API; bcrypt storage; anomaly detection | Low |
| Malicious PDF causes parser memory overflow | Denial of Service | Upload crafted PDF > 2GB | File size limit (50MB) enforced before parsing; timeout | Low |
| Malicious SVG executes in Studio | Tampering | Upload SVG with script tags | SVG sanitized by Core Engine; CSP header blocks execution | Very Low |
| Insecure Direct Object Reference | Spoofing | Guess or enumerate document UUIDs | All document queries include `WHERE owner_user_id = current_user` | Very Low |
| SSRF via webhook URL | Spoofing | Set webhook_url to internal service | Webhook URL validated against SSRF blocklist | Low |
| JWT replay after user deletion | Spoofing | Use valid JWT after account deleted | Clerk revokes all sessions on user deletion | Very Low |
| Claude API prompt injection | Tampering | Malicious PDF content manipulates Claude prompt | Extraction prompt uses structured output schema; response validated | Medium |
| Dependency supply chain attack | Tampering | Malicious package in npm or pip | cargo audit, pip-audit, npm audit in CI; lock files committed | Medium |

### 9.3 Secret Inventory

```
# ALL secrets live in Doppler. Zero exceptions.

ANTHROPIC_API_KEY         # Claude API key
CLERK_SECRET_KEY          # Clerk backend SDK key
CLERK_WEBHOOK_SECRET      # Clerk webhook HMAC secret
DATABASE_URL              # Supabase PostgreSQL connection string
REDIS_URL                 # Upstash Redis TLS connection string
R2_ACCESS_KEY_ID          # Cloudflare R2 key ID
R2_SECRET_ACCESS_KEY      # Cloudflare R2 secret
R2_BUCKET_NAME            # e.g. spdf-documents-prod
STRIPE_SECRET_KEY         # Stripe backend key
STRIPE_WEBHOOK_SECRET     # Stripe webhook signature secret
RESEND_API_KEY            # Resend email key
SENTRY_DSN                # Sentry error tracking DSN
AXIOM_API_TOKEN           # Axiom log shipping token
INTERNAL_API_SECRET       # API→Worker authentication HMAC key

# Rotation policy:
# - ANTHROPIC_API_KEY: Rotate quarterly or immediately on suspected compromise
# - Database passwords: Rotate every 90 days via Supabase dashboard
# - All secrets: Rotate immediately on any team member departure
```

### 9.4 Network Security

| Control | Implementation | Covers |
|---|---|---|
| TLS 1.3 termination | Cloudflare at edge; Railway enforces TLS internally | All client → API traffic |
| HSTS header | Strict-Transport-Security: max-age=31536000; includeSubDomains | Prevents downgrade attacks |
| WAF Rules | Cloudflare WAF: OWASP CRS + custom rules for /v1/ paths | SQLi, XSS, path traversal, scanner bots |
| DDoS Protection | Cloudflare Anycast DDoS mitigation (free tier) | Volumetric + protocol-layer attacks |
| CSP Header | Content-Security-Policy: default-src 'self'; script-src 'self' | Studio XSS; SVG script injection |
| CORS | Allow-Origin: studio.spdf.dev only; credentials: true | Cross-origin request forgery |
| Private networking (Railway) | API ↔ Worker communicate over Railway private network | Internal service impersonation |

---

## 10. Observability Architecture

### 10.1 The Three Pillars

| Pillar | Tool | What It Captures | Retention |
|---|---|---|---|
| Structured Logs | Axiom + stdout | Every request/response, every job state change, every error | 90 days (Axiom free tier) |
| Error Tracking | Sentry | All uncaught exceptions with stack traces, breadcrumbs, user context | 30 days (Sentry free tier) |
| Uptime Monitoring | BetterUptime | External HTTP checks every 3 minutes; SSL expiry; public status page | 90 days history |

### 10.2 Structured Log Schema

```json
// REQUEST event
{
  "level":        "info",
  "event":        "request.received",
  "request_id":   "req_a1b2c3d4e5f6",
  "method":       "POST",
  "path":         "/v1/documents/generate",
  "user_id":      "user_88291abc",
  "tier":         "PRO",
  "ip":           "103.45.67.89",
  "user_agent":   "spdf-python/1.0.0",
  "timestamp":    "2025-03-15T09:30:00.441Z"
}

// RESPONSE event
{
  "level":        "info",
  "event":        "request.completed",
  "request_id":   "req_a1b2c3d4e5f6",
  "status_code":  201,
  "duration_ms":  847,
  "document_id":  "spdf-d4b7c2a1-...",
  "timestamp":    "2025-03-15T09:30:01.288Z"
}

// ERROR event
{
  "level":        "error",
  "event":        "job.failed",
  "job_id":       "job_f3e2d1c0-...",
  "error_code":   "E_CLAUDE_TIMEOUT",
  "error_message":"Claude API did not respond within 60 seconds",
  "attempt":      3,
  "will_retry":   false,
  "fallback":     "heuristic_extractor",
  "sentry_event_id": "snt_abc123",
  "timestamp":    "2025-03-15T09:31:12.007Z"
}
```

### 10.3 Key Metrics Dashboard

| Metric | Source | Alert Threshold | Alert Channel |
|---|---|---|---|
| API p95 latency | Axiom (from response events) | > 3 seconds sustained 5 min | PagerDuty / Email |
| API error rate (5xx) | Axiom (from response events) | > 1% over 5 min | PagerDuty / Email |
| Job queue depth | Redis LIST length of Celery queue | > 100 jobs waiting > 5 min | Slack / Email |
| Job failure rate | Axiom (job.failed events) | > 5% over 10 min | Email |
| Claude API error rate | Axiom | > 10% failure in 15 min | Email |
| Conversion fallback rate | Axiom (fallback: heuristic events) | > 20% using fallback | Slack (informational) |
| DB connection pool exhaustion | Supabase metrics | > 80% pool utilization | Email |
| Uptime (external check) | BetterUptime | Any failed check | SMS + PagerDuty |

---

## 11. Deployment Architecture

### 11.1 Monorepo Structure

```
spdf/                                ← Git repository root
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                   ← CI: test + build on all PRs
│   │   ├── release.yml              ← Release: tag → PyPI + npm publish
│   │   └── security.yml             ← Weekly: cargo audit + pip-audit + npm audit
│   └── CODEOWNERS
├── crates/                          ← Rust workspace
│   ├── spdf-core/
│   ├── spdf-wasm/
│   └── spdf-python/
├── services/                        ← Backend services
│   ├── api/                         ← FastAPI application
│   │   ├── Dockerfile
│   │   ├── railway.toml
│   │   └── ...
│   └── worker/                      ← Celery worker
│       ├── Dockerfile
│       ├── railway.toml
│       └── ...
├── apps/
│   └── studio/                      ← React frontend (Vercel)
│       ├── vercel.json
│       └── ...
├── packages/                        ← Published SDK packages
│   ├── spdf-python/                 ← pip install spdf
│   └── spdf-js/                     ← npm install @spdf/sdk
├── spec/                            ← SPDF format specification
│   └── spdf-spec-v1.0.md
├── docs/                            ← Developer documentation (Mintlify)
├── scripts/
│   ├── smoke_test.py
│   ├── seed_dev_db.py
│   └── generate_openapi.py
├── docker-compose.yml               ← Local development
├── Cargo.toml                       ← Rust workspace manifest
├── justfile                         ← Task runner (just build, just test, just dev)
└── README.md
```

### 11.2 MVP Build Sequence

| Phase | Duration | Deliverable | Exit Criteria |
|---|---|---|---|
| 1 — Core Engine (Rust) | 3–4 weeks | spdf-core crate: parse, write, validate, render PDF | cargo test passes; golden file tests pass |
| 2 — Python Bindings | 1 week | PyO3 bindings; Python SDK basic API | pip install spdf; 10-line Python generates valid invoice |
| 3 — API Skeleton | 1 week | FastAPI app; auth; DB models; /health endpoint | API starts, auth works, DB migrations run |
| 4 — Storage + Jobs | 1 week | /upload, /generate, /jobs endpoints; R2 integration | Can upload PDF, create job, poll status |
| 5 — Conversion Worker | 3 weeks | Celery worker; Claude integration; heuristic fallback | Upload PDF → 30s → SPDF with 90%+ accuracy |
| 6 — Studio MVP | 2 weeks | React app; upload flow; WASM viewer; auth pages | User can sign up, upload PDF, see converted SPDF structure |
| 7 — Billing + Launch | 1 week | Stripe integration; rate limiting; Pro tier | User can upgrade to Pro; API key generated; rate limits enforced |
| 8 — Hardening | 1 week | E2E tests; monitoring; runbooks; public launch | Smoke tests pass; monitoring live; Product Hunt ready |

---

## 12. Architecture Decision Records (ADRs)

ADRs capture every significant architectural decision with full context. They are immutable history — superseded decisions are marked, not deleted.

### ADR-001 — Rust for the SPDF Core Engine
**STATUS: Accepted**

**Context:** The Core Engine must be fast, safe, and deployable in three environments: Python (via PyO3), browser (via WASM), and potentially native CLI. It processes untrusted input (user-uploaded PDFs). Memory safety is critical.

**Decision:** Use Rust for all Core Engine code. Python and TypeScript/WASM bindings are generated via PyO3 and wasm-bindgen respectively. The same Rust code is the single source of truth for all three environments.

**Consequences:** Pro: Memory safety by default eliminates buffer overflow class of parser vulnerabilities. Near-C performance. Same code runs everywhere. Con: Steep learning curve; compilation time; PyO3 build complexity in CI.

---

### ADR-002 — Modular Monolith First (Not Microservices)
**STATUS: Accepted**

**Context:** The system must be built and maintained by a single developer. Microservices introduce distributed systems complexity: network failures, service discovery, distributed tracing, independent deployments, data consistency across services.

**Decision:** Start with a modular monolith: single FastAPI application with clear internal module boundaries. The conversion worker is a separate process (Celery) but shares the same codebase. Split only when a concrete scaling or isolation need is proven by production data.

**Consequences:** The modular structure means splitting is straightforward when needed. We accept slower vertical scaling as a trade-off for operational simplicity.

---

### ADR-003 — Celery + Redis for Job Queue (Not SQS/RabbitMQ)
**STATUS: Accepted**

**Decision:** Use Celery with Upstash Redis as broker and result backend. Celery is mature, Python-native, well-documented, and works with existing Redis.

**Consequences:** SQS was considered but adds AWS vendor lock-in and billing complexity. RabbitMQ was considered but adds another managed service. Celery with Redis covers all requirements at zero additional cost given existing Redis usage.

---

### ADR-004 — Cloudflare R2 for File Storage (Not AWS S3)
**STATUS: Accepted**

**Decision:** Use Cloudflare R2 for all SPDF file storage. R2 is S3-API compatible, has **$0 egress fees**, and integrates with Cloudflare's global CDN.

**Consequences:** Zero egress fees are a fundamental competitive advantage. At 1M documents × 100KB average, S3 egress could cost $90+/month vs $0 on R2.

---

### ADR-005 — Clerk for Authentication (Not Auth0 or custom JWT)
**STATUS: Accepted**

**Decision:** Use Clerk for all authentication. Clerk provides pre-built React components, JWT generation, webhook events for user lifecycle, and enterprise SSO on higher tiers.

**Consequences:** The "build vs buy" calculus strongly favors buy for auth security primitives. Clerk free tier supports 10,000 MAU.

---

### ADR-006 — Decimal-as-String for All Financial Values
**STATUS: Accepted**

**Decision:** All financial values in SPDF semantic.json are stored as JSON strings in decimal notation (`"12345.67"`), not JSON numbers.

**Consequences:** This violates the "JSON numbers for numbers" intuition but is the correct choice for financial data. IEEE 754 cannot represent most decimal fractions exactly: 0.1 + 0.2 ≠ 0.3 in binary floating point.

---

### ADR-007 — Original PDF Retained as Render Layer in Conversions
**STATUS: Accepted**

**Decision:** Use the original PDF as the render layer in converted SPDF documents. The semantic layer is the extracted understanding; the render layer is the original document exactly as the sender intended it to look.

**Consequences:** Visual fidelity with the original is more important than render layer consistency. A converted document that looks different from the original would destroy user trust.

---

### ADR-008 — Cursor-Based Pagination (Not Offset-Based)
**STATUS: Accepted**

**Decision:** All list endpoints use cursor-based pagination. The cursor is an opaque base64-encoded token encoding (created_at, id) of the last seen record.

**Consequences:** Offset pagination degrades with OFFSET > 10,000 (full table scan to skip rows). Cursors are stable under concurrent mutations.

---

### ADR-009 — Railway for API Hosting (Not Fly.io, Render, ECS)
**STATUS: Accepted**

**Decision:** Use Railway.app for API and Worker hosting. Railway provides Dockerfile-based deploys, private networking between services, environment variable management, automatic HTTPS, and rolling deploys with health checks.

**Consequences:** Fly.io was considered but requires more config. Render was considered but inter-service networking is complex. ECS was considered but prohibitively complex for one developer.

---

### ADR-010 — Content-Addressed Asset IDs (SHA-256 Truncated)
**STATUS: Accepted**

**Decision:** Asset ID = first 16 hex characters of SHA-256(content). Same binary content always gets the same ID. Deduplication is automatic and free.

**Consequences:** UUID v4 IDs were rejected because they do not enable deduplication — the same logo embedded in 1,000 invoices would be stored 1,000 times.

---

## 13. Cross-Cutting Concerns

### 13.1 Configuration Management

```python
# services/api/config.py
from pydantic_settings import BaseSettings
from functools import lru_cache

class Settings(BaseSettings):
    # ── Core ───────────────────────────────────────────────────────────────
    environment:        str   = "development"
    debug:              bool  = False
    version:            str   = "1.0.0"

    # ── Database ─────────────────────────────────────────────────────────
    database_url:       str                   # Required — no default
    db_pool_size:       int   = 10
    db_max_overflow:    int   = 20

    # ── Redis ─────────────────────────────────────────────────────────────
    redis_url:          str                   # Required

    # ── Storage ───────────────────────────────────────────────────────────
    r2_endpoint:        str                   # Required
    r2_access_key_id:   str                   # Required
    r2_secret_access_key: str                 # Required
    r2_bucket_documents: str = "spdf-documents"

    # ── External Services ──────────────────────────────────────────────────
    anthropic_api_key:  str                   # Required
    clerk_secret_key:   str                   # Required
    stripe_secret_key:  str                   # Required

    # ── Feature Flags ──────────────────────────────────────────────────────
    enable_heuristic_fallback: bool = True
    max_upload_size_mb:        int  = 50
    claude_model:              str  = "claude-sonnet-4-6"
    claude_max_tokens:         int  = 4096

    class Config:
        env_file = ".env.local"
        case_sensitive = False

@lru_cache
def get_settings() -> Settings:
    return Settings()
```

### 13.2 Testing Strategy

| Level | What Is Tested | Tool | Coverage Target | Runs When |
|---|---|---|---|---|
| Unit — Rust | Core engine: parse, write, render, sign, validate | cargo test + proptest | 95% line coverage | Every commit |
| Fuzz — Rust | Parser: never panics on arbitrary input | cargo-fuzz (libFuzzer) | Continuous | CI + scheduled nightly |
| Golden files — Rust | Rendered PDFs match pixel-perfect reference | Custom image diff < 0.1% pixel delta | 100% of element types | Every commit |
| Unit — Python | Service layer: business logic, no I/O | pytest + unittest.mock | 90% line coverage | Every commit |
| Integration — Python | API endpoints with real DB+Redis | pytest + httpx + testcontainers | All endpoints | Every commit |
| E2E — Python | Full convert/generate/extract flows with real Claude API | pytest (marked slow) | Core flows only | Pre-deploy only |
| Contract — API | Responses match OpenAPI schema exactly | schemathesis | 100% of endpoints | Every commit |
| Smoke — Production | Critical paths work in live environment | Custom script | 5 critical flows | Post-every-deploy |

### 13.3 Performance Budgets

> **Rule:** Every endpoint that misses its budget is a P1 bug — treated like a functional bug. No performance regressions are merged without explicit justification.

| Component | Operation | p50 Budget | p95 Budget | p99 Budget |
|---|---|---|---|---|
| Core Engine | Parse 50-page SPDF DOM | 50ms | 200ms | 500ms |
| Core Engine | Generate simple invoice SPDF | 100ms | 500ms | 1,000ms |
| Core Engine | Render PDF from SPDF DOM | 200ms | 1,000ms | 2,000ms |
| API | POST /generate (simple, sync) | 300ms | 2,000ms | 4,000ms |
| API | POST /extract | 200ms | 1,000ms | 2,000ms |
| API | POST /validate | 100ms | 500ms | 1,000ms |
| API | GET /documents (list) | 50ms | 200ms | 500ms |
| Worker | PDF→SPDF conversion (1 page) | 5,000ms | 10,000ms | 20,000ms |
| Worker | PDF→SPDF conversion (10 pages) | 15,000ms | 30,000ms | 60,000ms |
| Studio | Initial SPDF render (1st page visible) | 1,000ms | 3,000ms | 5,000ms |

---

## 14. Technical Risk Register

Risks are scored P×I where P = Probability (1–5) and I = Impact (1–5). Score ≥ 12 = Critical; 8–11 = High; 4–7 = Medium; < 4 = Low.

| ID | Risk | P | I | Score | Mitigation | Status |
|---|---|---|---|---|---|---|
| TR-01 | PyO3 Rust→Python build complexity blocks rapid development | 4 | 4 | 16 ⚠ | Pre-built wheels in CI artifacts; maturin develop for local; Dockerfile handles full build | Open |
| TR-02 | Claude API latency spikes increase conversion time beyond SLA | 3 | 4 | 12 ⚠ | Async pipeline — latency hidden; timeout + heuristic fallback at 60s | Open |
| TR-03 | SPDF spec edge cases cause Core Engine to produce invalid documents | 3 | 5 | 15 ⚠ | Validator runs on every write; golden file tests; fuzz testing | Open |
| TR-04 | Supabase connection pool exhaustion under load | 2 | 4 | 8 | pgBouncer connection pooler; circuit breaker on DB calls | Monitoring |
| TR-05 | Cloudflare R2 outage makes documents unavailable | 1 | 5 | 5 | Docs always downloaded via signed URL; notify via status page | Accepted |
| TR-06 | Celery worker crashes mid-conversion — job lost | 2 | 3 | 6 | Jobs are idempotent; retry 3x; QUEUED state is durable in Postgres | Open |
| TR-07 | **Solo developer knowledge concentration risk** | 5 | 4 | 20 ⚠ | All decisions in ADRs; Claude Code generates code from architecture; full IaC in git; runbooks | Open |
| TR-08 | Prompt injection via malicious PDF content | 2 | 3 | 6 | Claude prompt uses structured output schema; response validated | Open |
| TR-09 | API key leaked in customer code repository | 3 | 3 | 9 | Keys are bcrypt-hashed; prefix-only display; one-click rotation | Open |
| TR-10 | Upstash Redis outage breaks rate limiting and job queue | 1 | 4 | 4 | Rate limit failure = fail open (warn, not block) | Accepted |
| TR-11 | Anthropic changes Claude API pricing / model deprecation | 2 | 3 | 6 | Abstracted behind claude_extractor.py; model is config-driven | Monitoring |
| TR-12 | WASM bundle size too large for acceptable Studio load time | 3 | 3 | 9 | wasm-opt for size optimization; lazy load WASM; brotli compression; budget: < 2MB gzipped | Open |

### Critical Risk Deep Dives

**TR-07 — Solo Developer Knowledge Concentration (Highest Risk)**

- Mitigation 1: ADR process ensures every decision is documented with context
- Mitigation 2: Claude Code generates code from architecture — the architecture IS the documentation
- Mitigation 3: Full Infrastructure as Code — any environment can be recreated from git in < 1 hour
- Mitigation 4: Runbooks for all operational procedures (deploy, rollback, incident response) in /docs/ops/
- Mitigation 5: No "clever" code — every function is readable and self-documenting

---

## Appendix A — Technology Version Reference

| Technology | Role | Version | License | Notes |
|---|---|---|---|---|
| Rust | Core Engine language | 1.75 (stable) | MIT/Apache 2.0 | Minimum supported: 1.70 |
| PyO3 | Rust↔Python bindings | 0.20.x | MIT | Requires Python 3.8+ |
| maturin | PyO3 build system | 1.4.x | MIT | Used in CI and local dev |
| wasm-bindgen | Rust↔WASM bindings | 0.2.90+ | MIT/Apache 2.0 | Pairs with wasm-pack |
| Python | API and Worker language | 3.12 (minimum 3.11) | PSF | 3.12 has best asyncio perf |
| FastAPI | API framework | 0.110.x | MIT | Uses Pydantic v2 |
| Celery | Task queue | 5.3.x | BSD | With Redis broker |
| SQLAlchemy | ORM | 2.0.x | MIT | Async engine required |
| Alembic | DB migrations | 1.13.x | MIT | — |
| React | Studio frontend | 18.x | MIT | — |
| Vite | Frontend bundler | 5.x | MIT | — |
| Tailwind CSS | Utility CSS | 3.4.x | MIT | — |
| Zustand | Frontend state | 4.5.x | MIT | Lightweight, no boilerplate |
| SWR | Data fetching / polling | 2.2.x | MIT | Job status polling |
| PostgreSQL | Primary database | 15.x | PostgreSQL | Hosted on Supabase |
| Redis | Queue + cache | 7.x | BSD-3 | Hosted on Upstash |
| wasm-pack | WASM bundler | 0.12.x | MIT/Apache 2.0 | Outputs ES module + TS types |
| cargo-fuzz | Parser fuzzing | 0.12.x | MIT/Apache 2.0 | libFuzzer backend |
| serde | Rust serialization | 1.0.x | MIT/Apache 2.0 | With serde_json feature |
| lopdf | PDF generation | 0.28.x | MIT | Pure Rust PDF object model |
| fontdue | Font rendering | 0.7.x | MIT | Pure Rust, no system deps |
| rsa | RSA signing | 0.9.x | MIT/Apache 2.0 | With pss feature for RSA-PSS |

---

## Appendix B — External Service Account Reference

| Service | Account Type | Free Tier Limits | Paid Upgrade Trigger |
|---|---|---|---|
| Railway | Hobby ($5/mo) | 512 MB RAM, 1 vCPU | > 512MB memory usage |
| Vercel | Hobby (free) | 100 GB bandwidth/mo | Team features needed |
| Supabase | Free | 500 MB DB, 1 GB file storage | DB > 500MB or > 50 connections |
| Upstash Redis | Pay-per-use | 10K commands/day free | Cost > $20/month |
| Cloudflare R2 | Free | 10 GB storage, 10M ops | Storage > 10 GB |
| Cloudflare CDN | Free | Unlimited bandwidth | Enterprise features |
| Clerk | Free | 10,000 MAU | > 10K MAU or SSO needed |
| Stripe | 2.9% + 30¢ | No monthly fee | Always pay-as-you-go |
| Resend | Free | 100 emails/day | > 100/day |
| Sentry | Free | 5K errors/month | > 5K errors or data retention |
| Axiom | Free | 500 GB ingest/month | > 500 GB/month |
| BetterUptime | Free | Unlimited monitors | Status page customization |
| Doppler | Free | All features free for individual | Team billing features |
| Anthropic Claude | Pay-per-token | No free tier | Budget > $100/month |

---

## Appendix C — Glossary

| Term | Definition |
|---|---|
| ADR | Architecture Decision Record — a document capturing a significant architectural decision and its rationale |
| C4 Model | A hierarchical diagram framework: Context → Containers → Components → Code |
| Celery | Python distributed task queue; used for async conversion jobs |
| CORS | Cross-Origin Resource Sharing — browser security mechanism for cross-domain API calls |
| DOM | Document Object Model — in-memory tree representation of an SPDF document's semantic content |
| EID | Element Identifier — SPDF format unique ID for each DOM element |
| Heuristic extraction | Rule-based (non-AI) PDF text extraction; used as Claude API fallback |
| IaC | Infrastructure as Code — all infrastructure defined in version-controlled files |
| JWT | JSON Web Token — compact signed credential used by Clerk for authentication |
| Modular Monolith | Single deployable application with clear internal module boundaries; not microservices |
| PITR | Point-In-Time Recovery — database backup capability to restore to any past moment |
| PyO3 | Rust library for writing Python extension modules in Rust |
| R2 | Cloudflare R2 object storage — S3-compatible, zero egress fees |
| SPDF | Structured Portable Document Format — the format this platform produces and processes |
| SSRF | Server-Side Request Forgery — attack where server fetches attacker-controlled URLs |
| Twelve-Factor | Methodology for building software-as-a-service apps (12factor.net) |
| WASM | WebAssembly — portable binary format; allows Rust Core Engine to run in browsers |

---

*— End of System Architecture Design v1.0 —*

SPDF Platform | Internal Engineering Document | Confidential
