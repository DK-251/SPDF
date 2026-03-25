# SPDF — Product Requirements Document
**PRD v1.0 — Initial Release**

| Field | Value |
|---|---|
| Document Type | Product Requirements Document |
| Version | 1.0 — Initial Release |
| Date | March 2025 |
| Stage | Pre-Development — Architecture Phase |
| Next Document | System Architecture Design (SAD) |

> *"The format is the product. The product serves the user. The user changes the world."*

---

## Document Control & Change History

| Version | Date | Author | Changes | Status |
|---|---|---|---|---|
| 1.0 | March 2025 | Founder + Claude AI | Initial PRD — all sections | Approved for architecture phase |

> **Document Policy:** This PRD is a living document. It will be updated after each development phase as user feedback is collected. All requirement IDs are stable — they will not change as the document evolves. Priority levels: P0 = must have for MVP │ P1 = required for launch │ P2 = post-launch │ P3 = future consideration

---

## Table of Contents

1. [Problem Statement](#1-problem-statement)
2. [First Target Market — The Entry Point Strategy](#2-first-target-market)
3. [Target Users](#3-target-users)
4. [Core Use Cases](#4-core-use-cases)
5. [Functional Requirements](#5-functional-requirements)
6. [Non-Functional Requirements](#6-non-functional-requirements)
7. [Compatibility Goals](#7-compatibility-goals)
8. [Success Metrics & KPIs](#8-success-metrics--kpis)
9. [Risks and Mitigations](#9-risks-and-mitigations)
10. [Constraints and Assumptions](#10-constraints-and-assumptions)
11. [MVP Definition](#11-mvp-definition)
12. [Open Questions](#12-open-questions)
13. [Document Series & Tracking](#13-document-series--tracking)

---

## 1. Problem Statement

### 1.1 The Core Problem

PDF was designed in 1993 as a final-output format — a digital equivalent of print. Every element in a PDF is an absolute drawing instruction with no semantic meaning. There is no concept of "heading", "table", "signature", or "data field" inside a PDF file. There are only positioned paint strokes on a virtual canvas.

This design decision — correct for 1993 — has become the single largest bottleneck in modern enterprise document processing. Every system that needs to understand a PDF must first reverse-engineer meaning from pixel coordinates. This is expensive, error-prone, and fundamentally unsolvable at the format level.

**The Cost of PDF's Structural Limitation — Real Data**

| Metric | Value |
|---|---|
| Average cost to manually process one invoice | $15.00 – $22.75 (Parseur, DocuClipper 2024) |
| Invoices still requiring manual data entry | 57% (DocuClipper 2024) |
| Invoices containing errors due to manual processing | 39% (DocuClipper 2024) |
| Average time to manually process one invoice | 14.6 days (DocuClipper 2024) |
| Average time with full automation | 7.9 days — a 46% reduction (Market Growth Report 2024) |
| Enterprise spend on PDF parsing infrastructure annually | $2B – $5B (industry estimate) |

### 1.2 Why Existing Solutions Fail

The market has attempted to solve this with OCR tools, AI extraction layers, and CLM platforms. All of these solutions share the same fundamental flaw: they attempt to extract meaning from a format that was never designed to carry meaning.

| Current Solution | What It Does | Why It Fails |
|---|---|---|
| Adobe Acrobat / PDF editors | Edit PDFs visually | Layout breaks on edit; no semantic structure; expensive |
| OCR tools (AWS Textract, Google Doc AI) | Extract text from PDFs | 60-85% accuracy ceiling; layout-dependent; costly at scale |
| Contract Lifecycle Management (CLM) | Manage contracts in software | Still PDF-dependent underneath; expensive; 54% user resistance (ILTA 2024) |
| PDF generation libraries (reportlab, iText) | Generate PDFs from code | Output is immediately a black box; terrible developer experience |
| E-signature platforms (DocuSign, Adobe Sign) | Sign PDFs electronically | No structural understanding; document is still a black box after signing |

### 1.3 The Root Cause

> **Root Cause Analysis:** The problem is not the tools. The problem is the format. Every tool built on top of PDF inherits its fundamental limitation: documents that are paintings, not data. The only complete solution is a new format that carries semantic meaning from the moment of creation — not extracted from it afterward.

---

## 2. First Target Market

> **Strategic Principle: Win Narrow First**
> A new document format cannot succeed as a general-purpose format immediately. PDF itself did not launch as "the universal document format". It launched as "the format for sharing print-ready documents across systems." SPDF must pick ONE narrow, painful, well-funded problem to solve first. Win there completely. Then expand.

### 2.1 Entry Point Evaluation Framework

| Evaluation Criterion | Weight | Why It Matters |
|---|---|---|
| Pain Intensity | 25% | The more painful the problem, the more users will change behavior to solve it |
| Willingness to Pay | 25% | New formats need funded early adopters who can justify the switching cost |
| Document Volume | 20% | High document volume = high API usage = natural revenue growth with customer success |
| Network Effect Potential | 15% | Documents sent out become distribution — recipients discover SPDF organically |
| Technical Feasibility | 15% | Some markets require regulatory approvals or specialized integrations that delay launch |

### 2.2 Market Candidates — Scored Evaluation

| Market | Pain | WTP | Volume | Network | Feasibility | TOTAL |
|---|---|---|---|---|---|---|
| B2B Invoice Generation & Processing | 95 | 90 | 98 | 90 | 95 | **93.5** |
| Legal Contract Management | 92 | 95 | 80 | 75 | 78 | 84.5 |
| Healthcare Records | 88 | 85 | 95 | 60 | 55 | 78.0 |
| Government Forms | 75 | 60 | 95 | 70 | 45 | 70.0 |
| Academic / Research Documents | 65 | 45 | 70 | 80 | 90 | 68.0 |
| HR Documents | 70 | 70 | 75 | 65 | 85 | 72.5 |

### 2.3 THE CHOSEN FIRST TARGET MARKET

> **TARGET MARKET #1: B2B Invoice Generation & Processing**
> *Developers and finance teams who generate, send, and process business invoices*

### 2.4 Why B2B Invoices Win

**Reason 1 — The Pain Is Quantified and Extreme**
- $15–22.75 to process ONE invoice manually (industry average 2024)
- 57% of invoice data still manually re-entered into accounting systems
- 39% of invoices contain errors requiring costly corrections
- AP teams spend 10+ hours per week on invoice processing
- Root cause: invoices are PDFs — black boxes with data locked inside

**Reason 2 — Both Sides of the Transaction Have the Problem**

| Invoice SENDER Pain | Invoice RECEIVER Pain |
|---|---|
| Builds PDF generation code in reportlab/iText — painful, fragile | Must OCR or manually re-enter every received invoice |
| Generated invoice is immediately a black box | Cannot automatically match invoice to PO |
| Cannot track which invoices were opened | Cannot feed invoice data to ERP without manual work |
| Cannot extract own data back from sent invoices | 39% error rate on manually processed invoices |
| Expensive custom code for each client's format | Late payments due to processing delays damage relationships |

**Reason 3 — Massive, Measurable, Growing Market**
- Global AP automation market: $3.08B in 2024, growing at 12.8% CAGR (Grand View Research)
- AI invoice management market: $2.8B in 2024, projected $47.1B by 2034 at 32.6% CAGR
- 300 billion invoices processed annually worldwide
- 75% of AP departments now use some form of AI or automation (Medius 2024)
- Only 9% fully automated — 91% still have significant manual processes to replace

**Reason 4 — Developer-Friendly Entry Point**

Every company with a billing system has a developer who builds and maintains invoice PDF generation. These developers hate their PDF tools. They are the exact audience who will discover SPDF on GitHub, try the SDK in an afternoon, and immediately understand its value.

**Reason 5 — Natural Network Effect**

When a company generates invoices in SPDF format and sends them to their clients, those clients receive a document that looks exactly like a PDF. But if those clients are SPDF-aware, they get a fully structured invoice they can process automatically. The format spreads through the financial supply chain organically.

**Reason 6 — Regulatory Tailwind**

E-invoicing mandates are expanding globally. The EU's eInvoicing Directive, India's GST e-invoicing mandate, and similar regulations in 60+ countries are forcing companies to adopt structured invoice formats.

### 2.5 The Beachhead Persona

**PRIMARY PERSONA: "The Developer Building the Invoice System"**

- **Name:** Arjun / Alex / Andrea (Developer, 3–8 years experience)
- **Role:** Backend developer or full-stack developer at a B2B SaaS company or fintech
- **Current situation:** Has built or inherited an invoice PDF generation system using reportlab, wkhtmltopdf, or a PDF library. Clients complain the invoices look bad or are hard to process. Finance team asks for data back from invoices — impossible without the original database.
- **What they want:** A clean SDK that generates beautiful invoices without fighting the format. Invoices their clients can actually process automatically.
- **How they discover SPDF:** GitHub search for "PDF generation Python" or "invoice generation SDK"; Hacker News post; Dev.to article

### 2.6 The Expansion Path From Invoices

| Phase | Market | Why It Follows | Timeline |
|---|---|---|---|
| Phase 1 | B2B Invoice Generation & Processing | Entry point — high pain, high volume, developer-friendly | Months 1–12 |
| Phase 2 | Financial Reports & Statements | Same developer audience, same finance teams, natural extension | Months 9–18 |
| Phase 3 | Legal Contracts & Agreements | Enterprise credibility from finance; legal needs audit trail already built | Months 15–24 |
| Phase 4 | Healthcare Records & Forms | Compliance (HIPAA) needs already proven in finance | Months 20–30 |
| Phase 5 | Government & Compliance Docs | Standards process begins | Year 3+ |
| Phase 6 | Universal Document Standard | Format adoption broad enough to be default choice for any document | Year 5+ |

---

## 3. Target Users

### 3.1 User Segments — Priority Order

| Priority | Segment | Size | Entry Path | Value Proposition |
|---|---|---|---|---|
| 1 — PRIMARY | Developers (invoice/billing systems) | 30M globally | GitHub, npm, pip, HN | Replace reportlab/iText with clean SDK that generates structured invoices |
| 2 — PRIMARY | Finance / AP Teams | 500M+ globally | Product Hunt, LinkedIn | Receive structured invoices that flow directly into ERP — zero manual entry |
| 3 — SECONDARY | Freelancers & Consultants | 500M+ globally | Free tier, Studio | Professional branded invoices in minutes |
| 4 — SECONDARY | SMB Owners | 400M+ globally | Free tier, word of mouth | No-code invoice generation with full payment tracking |
| 5 — ENTERPRISE | CFOs / Finance Directors | 50M+ globally | Direct sales, LinkedIn | Eliminate AP processing cost entirely |
| 6 — ENTERPRISE | Legal Teams | 5M+ globally | Enterprise inbound | Structured contracts with audit trail |

### 3.2 User Personas — Detailed

**Persona 1 — The Developer (Primary Acquisition)**

> Arjun Sharma — Backend Developer, FinTech Startup, Bengaluru | Experience: 5 years | Stack: Python, FastAPI, PostgreSQL
>
> - Maintains invoice generation system built on reportlab
> - Spends 2–3 hours/week fixing PDF rendering edge cases
> - Finance team cannot extract data from generated invoices
> - **SPDF Value:** 10 lines of Python → beautiful structured invoice, exportable to PDF
> - **Acquisition:** GitHub search → README → pip install spdf → working in 30 minutes

**Persona 2 — The AP Manager (Business Buyer)**

> Sarah Chen — AP Manager, Mid-size Manufacturing Company, Singapore | Team: 8 people | Volume: 3,000 invoices/month
>
> - Team manually keys 57% of invoice data into SAP
> - 39% of invoices have errors requiring follow-up
> - **SPDF Value:** Structured invoices → zero manual entry → process in 1 day not 14
> - **Acquisition:** LinkedIn → vendor sends first SPDF invoice → requests all vendors use it

**Persona 3 — The Freelancer (Free Tier Entry)**

> Priya Mehta — Independent UX Designer, Mumbai | Clients: 8–12 active | Invoices: 15–20/month
>
> - Uses Canva to design invoices, exports to PDF
> - Manually tracks which invoices are paid in a spreadsheet
> - **SPDF Value:** Branded template → generate in seconds → client receipt confirmation built in
> - **Acquisition:** Product Hunt → free tier → immediate value → upgrade for templates

**Persona 4 — The Enterprise CFO (Revenue Target)**

> Michael Torres — CFO, B2B SaaS Company, $50M ARR, London | AP Volume: 8,000 invoices/month | Current AP cost: $120,000/year
>
> - AP team of 12 spends 70% of time on data entry from received PDFs
> - Monthly close takes 8 days because invoice data is not immediately available
> - **SPDF Value:** Eliminate $80,000+/year in AP processing → ROI in 3 months
> - **Acquisition:** Enterprise sales → pilot → measured ROI → full deployment

---

## 4. Core Use Cases

### 4.1 Use Case Priority Matrix

| ID | Use Case | User | Phase | Priority |
|---|---|---|---|---|
| UC-01 | Generate structured invoice via SDK | Developer | MVP | P0 |
| UC-02 | Export SPDF invoice as PDF (backward compat) | Developer | MVP | P0 |
| UC-03 | Receive SPDF invoice — extract structured data | Developer / AP Team | MVP | P0 |
| UC-04 | Create invoice via Studio (no-code) | Freelancer / SMB | MVP | P0 |
| UC-05 | Convert existing PDF invoice to SPDF (Claude) | AP Team / Enterprise | MVP | P1 |
| UC-06 | Sign and seal an invoice document | All Users | MVP | P1 |
| UC-07 | Build and save branded invoice template | Designer / SMB | MVP | P1 |
| UC-08 | Bulk convert PDF archive to SPDF | Enterprise | Phase 2 | P1 |
| UC-09 | Query invoice data via natural language | Enterprise / AP Team | Phase 2 | P2 |
| UC-10 | Compare two invoice versions semantically | Enterprise / Legal | Phase 2 | P2 |
| UC-11 | ERP integration — auto-import invoice data | Enterprise | Phase 2 | P1 |
| UC-12 | Annotate invoice without breaking integrity | Legal / Finance | Phase 2 | P2 |
| UC-13 | Apply DRM and access control to invoice | Enterprise | Phase 3 | P2 |
| UC-14 | Generate invoice from CRM/ERP data webhook | Developer | Phase 2 | P2 |

### 4.2 Use Case Detail

**UC-01 — Generate Structured Invoice via SDK**

A developer integrates the SPDF Python or JavaScript SDK into their application backend. They pass structured invoice data to the SDK, which generates a complete SPDF document.

Happy Path:
1. Developer installs SDK: `pip install spdf`
2. Initializes document with brand and template
3. Passes invoice data as Python dict / JSON object
4. SDK generates SPDF document in < 500ms
5. Export to PDF for client delivery

Edge Cases: Multi-currency invoice, long line item list, missing required field, custom brand fonts not embedded.

**UC-03 — Receive SPDF Invoice — Extract Structured Data**

An AP team receives an SPDF invoice (which opens as a PDF). They pass it to the SPDF API to extract all structured data directly into their ERP system. Zero manual entry.

Happy Path:
1. SPDF invoice received (opens as PDF in email client)
2. AP automation calls SPDF API: `POST /extract`
3. Returns structured JSON: `{ vendor, invoice_number, line_items[], total, due_date, payment_terms }`
4. Data flows directly to ERP via existing integration
5. Invoice approved in < 1 day vs 14.6 day manual average

**UC-05 — Convert Existing PDF Invoice to SPDF**

An enterprise with 100,000+ historical PDF invoices wants to migrate their archive to SPDF.

| Confidence Score | Action | Human Review Required |
|---|---|---|
| 95–100% | Auto-certified SPDF — no review needed | No |
| 80–94% | SPDF generated with flagged elements highlighted | Spot check recommended |
| 60–79% | SPDF draft — elements marked as uncertain | Review required |
| Below 60% | SPDF skeleton only — significant elements unextracted | Full human review |

---

## 5. Functional Requirements

### 5.1 Requirement Priority Key

| Priority | Label | Definition |
|---|---|---|
| P0 | CRITICAL | Must work for MVP. Product does not ship without this. |
| P1 | HIGH | Required for public launch. Core user value depends on this. |
| P2 | MEDIUM | Important for growth. Needed before enterprise sales. |
| P3 | LOW | Future enhancement. Does not block any milestone. |

### 5.2 Format Specification Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-SPEC-01 | **SPDF Schema Definition** — A complete, versioned JSON Schema defining every valid SPDF document structure. | P0 — CRITICAL |
| FR-SPEC-02 | **Dual-Layer Container Format** — Every SPDF file must be a ZIP container holding: semantic.json, render.pdf, assets/, manifest.json. | P0 — CRITICAL |
| FR-SPEC-03 | **Semantic Element Type Registry** — Complete registry of semantic element types: Document, Page, Heading, Paragraph, InvoiceHeader, LineItem, LineItemTable, Subtotal, Tax, Total, PaymentTerms, PaymentDetails, Signature, Stamp, Image, Divider, Footer. | P0 — CRITICAL |
| FR-SPEC-04 | **Document State Machine** — Every SPDF document must have a state property: DRAFT, REVIEW, SIGNED, CERTIFIED. State transitions must be one-directional. | P1 — HIGH |
| FR-SPEC-05 | **Version Compatibility** — SPDF specification must be versioned with a major.minor scheme embedded in every file. | P1 — HIGH |

### 5.3 Core Engine Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-ENG-01 | **SPDF Document Parser** — Rust core engine must parse any valid SPDF file into a typed in-memory DOM. Round-trip parse + serialize must produce a byte-identical file. | P0 — CRITICAL |
| FR-ENG-02 | **SPDF Document Writer** — Core engine must serialize any valid DOM to a compliant SPDF binary with all four container layers. | P0 — CRITICAL |
| FR-ENG-03 | **PDF Render Layer Generation** — Core engine must render any SPDF document to a pixel-perfect PDF. | P0 — CRITICAL |
| FR-ENG-04 | **Cryptographic Document Signing** — Implement cryptographic signing using X.509 certificates and SHA-256. Signing must cover both semantic and render layers. | P1 — HIGH |
| FR-ENG-05 | **Semantic Document Diff** — Core engine must produce a structured diff between any two SPDF documents at the element level. | P1 — HIGH |

### 5.4 Python SDK Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-PY-01 | **Document Generation API** — Fluent API for creating SPDF documents. Complete invoice in fewer than 25 lines of Python code. | P0 — CRITICAL |
| FR-PY-02 | **Template System** — Load a pre-designed SPDF template and populate it with data via a plain Python dict. | P1 — HIGH |
| FR-PY-03 | **Data Extraction API** — Methods to extract structured data from an SPDF document. Returns typed Python objects (Decimal, datetime, dataclasses). | P0 — CRITICAL |
| FR-PY-04 | **PDF Export** — Export any SPDF document to a standard PDF file with a single method call. | P0 — CRITICAL |

### 5.5 REST API Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-API-01 | **PDF to SPDF Conversion Endpoint** — POST /convert accepts a PDF file and returns an SPDF document with confidence scores. | P0 — CRITICAL |
| FR-API-02 | **SPDF Generation from Template Endpoint** — POST /generate accepts a template ID and JSON data payload. Response time under 2 seconds. | P0 — CRITICAL |
| FR-API-03 | **Structured Data Extraction Endpoint** — POST /extract accepts an SPDF document and returns structured JSON data. | P1 — HIGH |
| FR-API-04 | **Authentication and Rate Limiting** — All endpoints require API key. Rate limits: Free (100 req/day), Pro (10,000 req/day), Enterprise (custom). | P0 — CRITICAL |

### 5.6 SPDF Studio Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-STU-01 | **PDF Upload and Conversion** — Accept any PDF file by drag-and-drop. Claude converts to SPDF. User can click any element to inspect its structured properties. | P0 — CRITICAL |
| FR-STU-02 | **Invoice Template Designer** — Visual template designer for invoice documents: brand colors, logos, layout sections, data binding variables. | P1 — HIGH |
| FR-STU-03 | **Document Signing Flow** — Guided signing flow. Upon signing, document transitions to SIGNED state and all editable elements lock permanently. | P1 — HIGH |

### 5.7 AI Intelligence Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-AI-01 | **PDF Semantic Extraction via Claude** — Classify and type every meaningful element in any invoice PDF. Target: 92%+ field-level accuracy on a 100-invoice test set. | P0 — CRITICAL |
| FR-AI-02 | **Confidence Scoring** — Every Claude-extracted element must carry a confidence score from 0.0 to 1.0, calibrated within a 5% band across all score ranges. | P1 — HIGH |
| FR-AI-03 | **Natural Language Document Query** — Natural language questions about any SPDF document with 90%+ answer accuracy. | P2 — MEDIUM |

---

## 6. Non-Functional Requirements

### 6.1 Performance Requirements

| Requirement ID | Metric | Target | Measurement Method |
|---|---|---|---|
| NFR-PERF-01 | SDK document generation time (simple invoice) | < 500ms p95 | Automated benchmark: 1,000 runs |
| NFR-PERF-02 | SDK document generation time (complex, 20 pages) | < 2,000ms p95 | Automated benchmark: 500 runs |
| NFR-PERF-03 | API /convert endpoint response time (1-page PDF) | < 10 seconds p95 | Load test: 100 concurrent requests |
| NFR-PERF-04 | API /generate endpoint response time | < 2 seconds p95 | Load test: 200 concurrent requests |
| NFR-PERF-05 | API /extract endpoint response time | < 1 second p95 | Load test: 500 concurrent requests |
| NFR-PERF-06 | Studio PDF upload to render time | < 15 seconds p95 | User timing: 50 test uploads |
| NFR-PERF-07 | SPDF file parse time (50-page document) | < 200ms | Automated benchmark in Rust |
| NFR-PERF-08 | PDF export from SPDF (any document) | < 1 second p95 | Automated benchmark: 1,000 runs |
| NFR-PERF-09 | Semantic diff (two 20-page documents) | < 3 seconds | Automated benchmark: 200 pairs |
| NFR-PERF-10 | API uptime SLA | 99.5% monthly | External uptime monitor |

### 6.2 Security Requirements

| Requirement ID | Requirement | Standard |
|---|---|---|
| NFR-SEC-01 | All data in transit encrypted via TLS 1.3 minimum | OWASP Transport Layer Protection |
| NFR-SEC-02 | All stored SPDF files encrypted at rest (AES-256) | NIST SP 800-111 |
| NFR-SEC-03 | API keys stored as salted bcrypt hashes — never plaintext | OWASP Authentication Cheat Sheet |
| NFR-SEC-04 | SPDF parser must reject any file containing executable content | Format specification requirement |
| NFR-SEC-05 | SQL injection prevention on all database queries | OWASP SQL Injection Prevention |
| NFR-SEC-06 | Rate limiting on all endpoints to prevent abuse | OWASP API Security Top 10 |
| NFR-SEC-07 | All user-uploaded files virus-scanned before processing | ClamAV or equivalent |
| NFR-SEC-08 | Zero third-party JavaScript in the SPDF format itself | Format specification requirement |
| NFR-SEC-09 | Dependency vulnerability scanning in CI/CD pipeline | cargo audit, pip-audit, npm audit |
| NFR-SEC-10 | Secret management via Doppler — no secrets in code or env files | Security best practice |

### 6.3 Reliability Requirements

| Requirement ID | Requirement | Target |
|---|---|---|
| NFR-REL-01 | API availability (monthly) | 99.5% uptime |
| NFR-REL-02 | API availability (enterprise SLA) | 99.9% uptime |
| NFR-REL-03 | Claude conversion job failure rate | < 2% of conversions |
| NFR-REL-04 | Data loss on infrastructure failure | Zero — all writes confirmed before response |
| NFR-REL-05 | Maximum recovery time from outage | < 30 minutes (RTO) |
| NFR-REL-06 | Maximum data loss window | < 1 hour (RPO) |
| NFR-REL-07 | SDK crash rate on valid SPDF files | Zero tolerance — no panics on valid input |
| NFR-REL-08 | Backward compatibility | SPDF 1.x files always openable by 1.x readers |

### 6.4 Scalability Requirements

| Requirement ID | Requirement | Target |
|---|---|---|
| NFR-SCL-01 | API horizontal scaling | Stateless design — scale to any number of instances |
| NFR-SCL-02 | Conversion queue throughput | 1,000 PDF conversions/hour on standard tier |
| NFR-SCL-03 | Conversion queue throughput — enterprise | 100,000 PDF conversions/hour on dedicated tier |
| NFR-SCL-04 | Database connection pooling | Support 10,000 concurrent API connections |
| NFR-SCL-05 | File storage scalability | No hard limit — R2 object storage scales infinitely |
| NFR-SCL-06 | SDK performance at scale | Linear — 100x documents takes 100x time, not more |

### 6.5 Usability Requirements

| Requirement ID | Requirement | Target |
|---|---|---|
| NFR-USE-01 | SDK time to first working invoice (developer) | < 30 minutes from pip install to valid SPDF |
| NFR-USE-02 | Studio time to first document upload and view | < 5 minutes from account creation |
| NFR-USE-03 | API documentation completeness | 100% of endpoints documented with examples |
| NFR-USE-04 | Error message clarity | Every error includes: what failed, why, and how to fix it |
| NFR-USE-05 | SDK type coverage | 100% of public API methods have complete type hints |
| NFR-USE-06 | Accessibility of Studio (WCAG) | WCAG 2.1 AA compliance |

---

## 7. Compatibility Goals

### 7.1 PDF Viewer Compatibility — THE NON-NEGOTIABLE

> **ABSOLUTE REQUIREMENT:** Every SPDF file must open perfectly in every PDF viewer. The end user must never know they are opening an SPDF file.

| Viewer / Platform | Test Target | Acceptance Criteria |
|---|---|---|
| Adobe Acrobat Reader (Windows, Mac) | P0 — Must pass | Opens without warnings, renders identically, prints correctly |
| Chrome browser PDF viewer | P0 — Must pass | Opens without warnings, renders identically |
| Firefox browser PDF viewer (PDF.js) | P0 — Must pass | Opens without warnings, renders identically |
| Safari (macOS, iOS) | P0 — Must pass | Opens without warnings, renders identically |
| macOS Preview | P0 — Must pass | Opens without warnings, renders identically |
| Microsoft Edge PDF viewer | P0 — Must pass | Opens without warnings, renders identically |
| Foxit Reader | P1 — Should pass | Opens without warnings, renders identically |
| Android PDF viewer (stock) | P1 — Should pass | Opens without warnings, renders identically |
| Google Drive PDF preview | P1 — Should pass | Renders correctly in Drive preview |
| Email clients (Gmail, Outlook, Apple Mail) | P1 — Should pass | PDF attachment preview renders correctly |
| Industrial print systems | P2 — Nice to have | Prints identically to equivalent PDF |

### 7.2 Platform & OS Compatibility

| Component | Windows | macOS | Linux | iOS | Android |
|---|---|---|---|---|---|
| Python SDK (3.10, 3.11, 3.12) | P0 | P0 | P0 | N/A | N/A |
| JavaScript SDK (Node 18, 20, 22) | P0 | P0 | P0 | N/A | N/A |
| Rust CLI Tool | P0 | P0 | P0 | N/A | N/A |
| SPDF Studio (web app) | P0 | P0 | P0 | P1 | P1 |
| SPDF Core (Rust binary) | P0 | P0 | P0 | P2 | P2 |
| SPDF WASM (browser) | P0 | P0 | P0 | P1 | P1 |

### 7.3 ERP & Enterprise System Compatibility (Phase 2)

| System | Integration Type | Priority | Notes |
|---|---|---|---|
| SAP S/4HANA | REST API connector + structured data output | P1 | Most common enterprise ERP |
| Oracle ERP Cloud | REST API connector | P1 | Second most common enterprise ERP |
| Microsoft Dynamics 365 | REST API connector | P1 | Common in mid-market |
| QuickBooks Online | Native integration + webhook | P1 | SMB accounting |
| Xero | Native integration | P2 | Common in Asia-Pacific markets |
| Tally (India) | Native integration | P2 | Critical for Indian market |
| Salesforce | CRM → invoice generation | P2 | Trigger SPDF invoice from closed deal |
| Zoho Books | Native integration | P2 | Growing SMB market globally |

### 7.4 Programming Language SDK Compatibility

| Language | SDK Type | Priority | Rationale |
|---|---|---|---|
| Python 3.10+ | Native (PyO3 + Rust core) | P0 | Primary data/backend language for invoice systems |
| JavaScript / TypeScript | WASM + Native bindings | P0 | Frontend Studio + Node.js backend |
| Rust | Native (core) | P0 | Core engine |
| Java / Kotlin | JNI bindings | P2 | Enterprise backend systems |
| .NET / C# | P/Invoke bindings | P2 | Microsoft Dynamics, enterprise Windows systems |
| Go | CGO bindings | P2 | Growing in fintech and cloud-native services |
| PHP | FFI bindings | P3 | Legacy web applications |

---

## 8. Success Metrics & KPIs

### 8.1 North Star Metric

> **NORTH STAR: Number of SPDF invoices generated per month**
>
> - Month 3: 1,000 invoices/month
> - Month 6: 10,000 invoices/month
> - Month 12: 100,000 invoices/month
> - Year 2: 1,000,000 invoices/month

### 8.2 Acquisition Metrics

| Metric | Month 3 | Month 6 | Month 12 | Year 2 |
|---|---|---|---|---|
| GitHub Stars | 200+ | 1,000+ | 5,000+ | 15,000+ |
| SDK Downloads (pip + npm, monthly) | 500+ | 5,000+ | 25,000+ | 200,000+ |
| Registered Studio Users | 100+ | 1,000+ | 10,000+ | 100,000+ |
| Active Developers (used SDK in 30 days) | 50+ | 500+ | 3,000+ | 25,000+ |
| API Keys Created | 20+ | 200+ | 2,000+ | 20,000+ |
| Free Tier Active Users | 50+ | 800+ | 8,000+ | 80,000+ |

### 8.3 Activation Metrics

| Metric | Definition | Target |
|---|---|---|
| Developer time-to-first-SPDF | Minutes from pip install to first valid SPDF generated | < 30 minutes |
| Studio time-to-first-document | Minutes from signup to first document viewed | < 5 minutes |
| API time-to-first-call | Minutes from API key creation to first successful API call | < 15 minutes |
| Documentation completion rate | % of users who complete the "Getting Started" guide | > 60% |
| Template usage rate | % of Studio users who create or use a template | > 40% in first week |

### 8.4 Revenue Metrics

| Metric | Month 6 | Month 12 | Year 2 | Year 3 |
|---|---|---|---|---|
| Monthly Recurring Revenue (MRR) | $500–1,000 | $5,000–10,000 | $30,000–60,000 | $100,000–200,000 |
| Annual Recurring Revenue (ARR) | $6,000–12,000 | $60,000–120,000 | $360K–720K | $1.2M–2.4M |
| Paying customers | 5–10 | 50–100 | 300–600 | 1,000–2,000 |
| Enterprise customers (>$2K/mo) | 0 | 2–3 | 10–20 | 30–50 |
| API revenue (monthly) | $200–500 | $2,000–5,000 | $15,000–30,000 | $50,000–100,000 |
| Average Revenue Per User (ARPU) | $29 | $40–60 | $60–100 | $80–120 |

### 8.5 Quality Metrics

| Metric | Target | Measurement |
|---|---|---|
| Claude conversion accuracy (invoice fields) | 92%+ field-level accuracy | Test set of 200 invoices with ground truth |
| PDF render fidelity (visual regression) | Zero regressions per release | Automated pixel comparison against golden files |
| SDK crash rate on valid input | 0.00% — zero tolerance | Continuous fuzzing + integration tests |
| API error rate (5xx) | < 0.1% of requests | Sentry monitoring |
| User-reported rendering issues | < 5 per 10,000 documents | Support ticket tracking |
| NPS score (developer users) | > 50 | Quarterly survey |
| NPS score (business users) | > 40 | Quarterly survey |

---

## 9. Risks and Mitigations

### 9.1 Risk Register

| Risk ID | Risk | Category | Probability | Impact | Risk Score | Mitigation |
|---|---|---|---|---|---|---|
| R-01 | PDF render layer fails on edge-case fonts | Technical | High | High | Critical | Extensive font test suite; fallback font substitution; user reporting pipeline |
| R-02 | Claude API cost exceeds revenue at scale | Financial | Medium | High | High | Cache conversion results; prompt optimization; revenue gates on free tier usage |
| R-03 | Adobe or Google announces competing open format | Competitive | Low | Critical | High | First mover advantage; open spec published immediately |
| R-04 | Developer adoption slower than projected | Market | Medium | High | High | Free tier with zero friction; extensive documentation; developer advocacy content |
| R-05 | Enterprise sales cycle too long for cash flow | Business | High | Medium | High | SMB-first revenue; API usage billing creates recurring revenue before enterprise closes |
| R-06 | ZIP container format causes confusion with PDF viewers | Technical | Medium | High | High | Extensive compatibility testing; render layer validation on every build |
| R-07 | SPDF spec vulnerabilities discovered post-launch | Security | Low | Critical | High | Security review before launch; responsible disclosure program; rapid patch process |
| R-08 | Claude API unavailability affects conversion | Technical | Low | High | Medium | Queue-based processing; retry logic; fallback to basic extraction |
| R-09 | Solo developer bandwidth bottleneck | Operational | High | Medium | Medium | Claude Code handles 70%+ of coding; strict scope discipline |
| R-10 | Legal challenge to open specification | Legal | Very Low | Critical | Medium | W3C/OASIS process from day one; legal review before publication |
| R-11 | Data breach of customer documents | Security | Low | Critical | High | Zero-knowledge architecture option; encryption at rest; SOC2 process from month 6 |
| R-12 | Rendering difference on obscure PDF viewers | Technical | High | Low | Medium | Document supported viewers; render layer compatibility matrix |

### 9.2 Top 3 Risks — Detailed Mitigation Plans

**R-01 — PDF Render Layer Font Failures**

- Build a test corpus of 500+ PDF files specifically chosen to cover font edge cases
- Implement automatic font subsetting
- Build a font fallback registry
- Visual regression tests on every CI build
- Ship with 5 embedded system fonts that cover 99% of business document needs

**R-02 — Claude API Cost at Scale**

- Cache conversion results by PDF hash — identical PDFs are never converted twice
- Rate limit free tier conversions (10/month)
- Optimize Claude prompts for token efficiency — target < 2,000 tokens per page
- Build a cost dashboard — monitor cost per conversion vs revenue per conversion in real time
- Implement a conversion budget per user tier

**R-04 — Developer Adoption Slower Than Projected**

- Write the "getting started" experience before writing any other code — under 30 minutes
- Publish a "Why I replaced reportlab with SPDF" technical blog post on Hacker News on launch day
- Create a migration guide: "Replace your reportlab invoice code with SPDF in 45 minutes"
- Build a showcase gallery of SPDF-generated invoices
- Launch on Product Hunt with a live demo that works in under 60 seconds

---

## 10. Constraints and Assumptions

### 10.1 Constraints

| Constraint | Type | Impact | Mitigation |
|---|---|---|---|
| Solo development team | Resource | All features must be achievable by 1 developer + Claude Code | Strict scope: only P0/P1 in MVP |
| Claude Enterprise API dependency | Technical | AI conversion quality depends on Anthropic uptime | Queue-based async processing; graceful degradation |
| $25,000 total budget (Year 1) | Financial | No paid team, minimal paid marketing | Open source for developer acquisition; content marketing |
| Rust expertise curve | Technical | Rust requires deep knowledge for correctness | Claude Code writes Rust; architect reviews; extensive tests |
| PDF spec complexity (756 pages) | Technical | Edge cases are near-infinite; perfect compatibility impossible | Target 99% of real-world documents; document known limitations |
| ZIP container file extension convention | Technical | Files have .spdf extension but are ZIPs — may confuse tools | Use magic bytes + manifest to declare format |

### 10.2 Assumptions

| Assumption | Basis | Risk if Wrong |
|---|---|---|
| Developers will try a new SDK if it solves a clear pain point | Historical evidence: Stripe, Twilio, FastAPI all grew via developer adoption | Mitigation: Free tier, excellent docs, fast time-to-value |
| Claude can extract invoice fields at 90%+ accuracy | Claude's document understanding capability demonstrated on internal tests | Mitigation: Extensive prompt engineering; confidence scoring; human review fallback |
| PDF viewers will open ZIP containers with .spdf extension | PDF viewers check file content (magic bytes) not just extension | Mitigation: Full compatibility testing on 10+ viewers before launch |
| B2B invoice pain is sufficient to motivate format adoption | $15–22.75/invoice processing cost is well documented | Mitigation: Directly measurable ROI — track with pilot customers |
| The open source spec will attract community contributions | Historical: Linux, Kubernetes, VS Code all grew through open source | Mitigation: Write the spec for clarity; provide good first issues |

---

## 11. MVP Definition

> **MVP PRINCIPLE:** The minimum product that lets a developer generate a structured SPDF invoice and export it as a PDF that looks professional — in under 30 minutes from pip install. Everything else comes after this works perfectly.

### 11.1 MVP Scope — What Is Included

| Component | MVP Feature | Quality Bar |
|---|---|---|
| SPDF Spec | Invoice-specific schema (18 element types), document states, signing | Fully documented, published on GitHub |
| Core Engine (Rust) | Parse, write, render to PDF, basic signing | 100% test coverage on happy paths |
| Python SDK | Document generation, template loading, PDF export, basic extraction | pip install spdf + hello world in < 30 min |
| JavaScript SDK | WASM-based browser/Node generation and export | npm install spdf + hello world in < 30 min |
| CLI Tool | spdf convert, spdf validate, spdf export | Works on Windows, Mac, Linux |
| REST API | POST /convert, /generate, /extract (basic), /sign | API key auth, rate limiting, OpenAPI docs |
| Studio (basic) | Upload PDF, view structured elements, export SPDF | Free signup, zero friction |
| Documentation | Getting started, SDK reference, API reference, 3 tutorials | < 30 min to first working invoice |

### 11.2 MVP Scope — What Is Explicitly Excluded

| Feature | Reason Excluded | When Added |
|---|---|---|
| Enterprise SSO / SAML | No enterprise customers at MVP | Phase 2 (Month 7–9) |
| Bulk PDF migration pipeline | Not needed for developer adoption | Phase 2 (Month 7–9) |
| Team workspaces and collaboration | Solo use is sufficient for MVP | Phase 2 (Month 7–9) |
| ERP integrations (SAP, Oracle, etc.) | Requires enterprise customers to build against | Phase 2 (Month 9–12) |
| Natural language document query | Not needed for core invoice use case | Phase 2 (Month 7–9) |
| Template marketplace | Need templates to exist first | Phase 3 (Month 10–12) |
| Blockchain anchoring | Adds complexity without MVP value | Phase 3+ |
| Java/.NET/Go SDK bindings | Core audience is Python/JS first | Phase 2–3 |
| Mobile apps (iOS/Android) | Web Studio works on mobile sufficiently for MVP | Year 2 |
| On-premise deployment | No air-gapped enterprise customers at MVP | Year 2 |

### 11.3 MVP Success Criteria — The Definition of Done

The MVP is complete when ALL of the following are true:

1. A developer can `pip install spdf`, generate a complete, valid, beautiful invoice in < 25 lines of Python, export it as a professional PDF, in under 30 minutes from first install
2. The exported PDF opens without errors or warnings in: Adobe Acrobat, Chrome, Firefox, Safari, macOS Preview, Edge
3. The SPDF specification v1.0 is published on GitHub with: full schema documentation, element type reference, security model
4. The REST API is live with: /convert, /generate, /extract endpoints operational; API key authentication working; OpenAPI documentation auto-generated
5. SPDF Studio is live with: any PDF uploadable and viewable as SPDF structure; free tier working with Stripe payment for Pro upgrade
6. First 5 external developers have used the SDK and given feedback; at least 3 of 5 report the experience as "better than what I had before"

---

## 12. Open Questions

| ID | Question | Options | Decision Needed By | Owner |
|---|---|---|---|---|
| OQ-01 | File extension: .spdf or .sdoc or keep .pdf? | .spdf (clear branding) vs new extension vs .pdf (invisible) | Before spec publication | Founder |
| OQ-02 | Semantic layer format: JSON or MessagePack? | JSON (readable, debuggable) vs MessagePack (smaller, faster) | Before core engine build | Architect |
| OQ-03 | Open source license for core vs platform? | MIT for spec+SDK vs Apache 2.0 vs SSPL (like MongoDB) | Before GitHub launch | Legal |
| OQ-04 | Should render layer be optional in SDK-generated docs? | Always include (larger files) vs optional (breaking compat) | Before SDK release | Architect |
| OQ-05 | First geographic market: India, US, or EU? | India (GST e-invoicing mandate) vs US (large AP market) vs EU (eInvoicing directive) | Month 1 | Founder |
| OQ-06 | Claude model: Sonnet vs Opus for conversion? | Sonnet (fast, cheaper) vs Opus (more accurate, expensive) | Before API launch | Founder |
| OQ-07 | Authentication: JWT vs API keys vs both? | API keys (simpler for developers) vs JWT (more secure) vs both | Before API build | Architect |

---

## 13. Document Series & Tracking

### 13.1 Document Series

| # | Document | Status | Purpose |
|---|---|---|---|
| 01 | Vision & Architecture Document | ✅ Complete | Market opportunity, technical vision, investment plan |
| 02 | Product Requirements Document (this document) | ✅ Complete | What to build, for whom, and how to measure success |
| 03 | System Architecture Design (SAD) | ⏳ Next — awaiting PRD approval | Technical blueprint: components, data flows, APIs |
| 04 | Database Schema Design | ⏳ Pending SAD | Table structures, relationships, indexes |
| 05 | API Contract Specification | ⏳ Pending SAD | OpenAPI/Swagger spec for all endpoints |
| 06 | SPDF Format Specification v1.0 | ⏳ Pending SAD | The open standard document |
| 07 | Development Sprint Plan | ⏳ Pending spec | Week-by-week build plan with Claude Code |
| 08 | Test Plan & QA Strategy | ⏳ Pending build start | Test cases, coverage targets, automation strategy |
| 09 | Launch Runbook | ⏳ Pending MVP completion | Product Hunt launch, HN post, first 1,000 users plan |

> **NEXT STEP:** Once this PRD is approved, the System Architecture Design (SAD) begins. Awaiting your review and confirmation to proceed.

---

*— End of Product Requirements Document —*
