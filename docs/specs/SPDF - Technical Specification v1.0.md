# SPDF — Structured Portable Document Format
## Technical Specification v1.0

| Field | Value |
|---|---|
| Specification ID | SPDF-SPEC-2025-001 |
| Version | 1.0 |
| Status | Draft — For Review |
| Date | March 2025 |
| Sections | 20 Normative Sections + Appendices |
| Replaces | None (first version) |
| Next Review | September 2025 |

> *This document is the normative technical specification for the Structured Portable Document Format (SPDF). It defines all rules, schemas, algorithms, and constraints required for conformant implementations.*

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Terminology and Definitions](#2-terminology-and-definitions)
3. [Design Principles](#3-design-principles)
4. [File Container Architecture](#4-file-container-architecture)
5. [Document Object Model (DOM)](#5-document-object-model-dom)
6. [Content Model](#6-content-model)
7. [Layout System](#7-layout-system)
8. [Rendering Model](#8-rendering-model)
9. [Asset Management](#9-asset-management)
10. [Metadata System](#10-metadata-system)
11. [Security Model](#11-security-model)
12. [Versioning and Compatibility](#12-versioning-and-compatibility)
13. [Error Handling and Validation](#13-error-handling-and-validation)
14. [Performance Considerations](#14-performance-considerations)
15. [Extensibility](#15-extensibility)
16. [Reference Implementation Guidelines](#16-reference-implementation-guidelines)
17. [SDK and Developer APIs](#17-sdk-and-developer-apis)
18. [Interoperability and Conversion](#18-interoperability-and-conversion)
19. [Compliance and Conformance](#19-compliance-and-conformance)
20. [Future Roadmap Considerations](#20-future-roadmap-considerations)
- [Appendix A — Complete styles.json Schema](#appendix-a--complete-stylesjson-schema)
- [Appendix B — SPDF Error Code Quick Reference](#appendix-b--spdf-error-code-quick-reference)
- [Appendix C — Normative References](#appendix-c--normative-references)
- [Appendix D — Glossary of Acronyms](#appendix-d--glossary-of-acronyms)

---

## 1. Introduction

### 1.1 Purpose of This Specification

This document constitutes the normative technical specification for the Structured Portable Document Format (SPDF), version 1.0. It defines the complete set of rules, data structures, algorithms, and constraints that govern the creation, parsing, validation, rendering, and exchange of SPDF documents.

All language in normative sections uses the key words defined in Section 2.3 (Conformance Language) and SHALL be interpreted according to those definitions.

### 1.2 Purpose of SPDF

The Portable Document Format (PDF), standardized as ISO 32000, has served as the world's primary fixed-layout document format since 1993. PDF excels at visual fidelity and print compatibility. However, its core design — a stream of positioning instructions for drawing glyphs and graphics on a virtual canvas — makes it fundamentally hostile to programmatic processing.

SPDF addresses this by redesigning the document container from the ground up with the following primary goals:

- Preserve all visual and print fidelity guarantees that made PDF the universal document standard
- Add a first-class semantic layer where every element is typed, named, and queryable
- Enable deterministic rendering across all compliant platforms without visual deviation
- Make documents natively parseable by automation systems, APIs, and AI pipelines without OCR
- Provide a cryptographically sound trust model with element-level integrity and audit capability
- Define a clean, versioned extensibility mechanism that does not break backward compatibility

### 1.3 Key Problems Solved

| Problem in PDF / Traditional Formats | How SPDF Solves It |
|---|---|
| No semantic element types — content is drawing instructions | Typed DOM: every element carries a declared type and structured properties |
| Tables are positioned text — not queryable data structures | Table type with structured rows, columns, and cell semantics |
| Data extraction requires OCR or heuristic parsing | semantic.json provides machine-readable structure as first-class content |
| No version-safe extensibility — unknown elements crash parsers | Forward-compatibility protocol: unknown keys MUST be preserved and ignored |
| Digital signatures cover the whole file or nothing | Element-level integrity: individual components carry independent hashes |
| Redaction is paint-over — data remains in file | Cryptographic erasure in both semantic and render layers with audit proof |
| PDF JavaScript enables malware delivery | SPDF specification prohibits all executable content by design |
| No built-in audit trail | Immutable audit log embedded in container, hash-chained to document root |
| Font rendering differs across platforms | Mandatory font subsetting ensures identical rendering everywhere |
| No canonical form for signing | Defined canonical serialization eliminates signing ambiguity |

### 1.4 Target Industries and Use Cases

**1.4.1 Primary Target: Financial Document Automation** — B2B invoices, purchase orders, statements of account, remittance advice, and financial reports. SPDF enables zero-manual-entry AP/AR workflows by making every financial figure, line item, and payment term a named, typed, extractable element.

**1.4.2 Legal and Compliance** — Contracts, agreements, regulatory filings, court documents, and compliance certificates. SPDF's document state machine (DRAFT → REVIEW → SIGNED → CERTIFIED), element-level locking, and cryptographic audit trail make it suitable for legally binding document workflows.

**1.4.3 Government and Public Administration** — Tax documents, permits, licenses, certificates, and citizen-facing forms. SPDF's support for structured form fields and certified state enables unforgeable government-issued documents verifiable offline.

**1.4.4 Healthcare** — Medical records, prescriptions, referral letters, and lab reports. SPDF's PII-tagging at the element level, cryptographic erasure capability, and HIPAA-aligned audit trail support structured healthcare document workflows.

**1.4.5 AI and Data Pipelines** — SPDF is designed as a first-class input format for AI document processing. The semantic layer provides LLMs and extraction models with pre-structured content, eliminating the need for layout analysis, OCR, and heuristic table reconstruction.

### 1.5 Non-Goals

The following are explicitly outside the scope of SPDF version 1.0:

- Real-time collaborative editing (planned for SPDF 2.x via Operational Transform extension)
- Reflowing or responsive layout (SPDF is a fixed-layout format)
- Interactive scripting or embedded application logic (prohibited by the security model)
- Video or audio embedding
- 3D content
- Full replacement of HTML as a web format
- Any form of executable content whatsoever (categorically prohibited)

### 1.6 Relationship to Existing Standards

| Standard | Relationship |
|---|---|
| ISO 32000-2 (PDF 2.0) | SPDF render layer MUST be a conformant PDF 2.0 document |
| RFC 8785 (JSON Canonicalization Scheme) | SPDF canonical form uses JCS for deterministic signing |
| RFC 3161 (Trusted Timestamping) | SPDF CERTIFIED state uses RFC 3161 timestamp tokens |
| W3C XML Signature (XMLDSig) | SPDF element-level signatures follow XMLDSig conceptual model |
| RFC 1951 (DEFLATE) | SPDF assets use DEFLATE compression within the ZIP container |
| IETF RFC 2119 | Conformance language (MUST, SHALL, SHOULD, MAY) follows RFC 2119 |
| Unicode 15.0 / UTF-8 (RFC 3629) | All text in SPDF MUST be encoded as UTF-8 |

---

## 2. Terminology and Definitions

### 2.1 Core Document Concepts

| Term | Definition |
|---|---|
| SPDF Document | A conformant file archive satisfying all requirements of this specification, consisting of a Semantic Layer, a Render Layer, an Asset Layer, and a Security Layer |
| Document Object Model (DOM) | The in-memory tree representation of an SPDF document's semantic content, instantiated by a conformant parser from semantic.json |
| Semantic Layer | The structured, machine-readable representation of document content, stored as semantic.json within the SPDF container |
| Render Layer | A conformant PDF 2.0 document stored as render.pdf within the SPDF container, providing visual fidelity and backward compatibility |
| Asset Layer | The collection of embedded binary resources (fonts, images, vector graphics, attachments) stored in the assets/ directory |
| Security Layer | The cryptographic integrity system comprising element hashes, document signatures, and the audit log |
| Container | The ZIP-format archive file with the .spdf extension that holds all layers |
| Manifest | The manifest.json file at the container root, containing format version, layer checksums, and structural metadata |
| Canonical Form | The deterministic, normalized serialization of the Semantic Layer used as input to cryptographic operations |

### 2.2 Element and Structure Terms

| Term | Definition |
|---|---|
| Element | The atomic unit of the SPDF DOM. Every element has a declared type, a unique identifier, a set of typed properties, and zero or more child elements |
| Element Type | The declared semantic category of an element, expressed as a string conforming to the Element Type Registry in Section 5.3 |
| Element ID | A globally unique identifier within a document, conforming to the ID format defined in Section 2.4 |
| Container Element | An element whose primary purpose is to hold other elements (e.g., Section, Group, Column) |
| Leaf Element | An element that cannot contain child elements (e.g., Text, Image, Field) |
| Template | An SPDF document with declared variable binding points, intended to be populated with data to produce a completed document |
| Variable | A named placeholder in a Template, conforming to the variable naming rules in Section 2.5 |

### 2.3 Conformance Language (RFC 2119)

| Keyword | Meaning |
|---|---|
| MUST / SHALL | The requirement is absolute. Non-conformance disqualifies an implementation. |
| MUST NOT / SHALL NOT | The prohibition is absolute. |
| SHOULD / RECOMMENDED | The preferred behavior. Deviation requires justification. |
| MAY / OPTIONAL | The feature is permitted but not required. |

### 2.4 Identifier Naming Conventions

#### 2.4.1 Element Identifiers (eid)

```
// Element ID Format (ABNF)
eid         = prefix "-" timestamp "-" sequence "-" checksum
prefix      = 2*4 ALPHA           ; lowercase scope prefix (e.g., "el", "sec", "tbl")
timestamp   = 13DIGIT             ; Unix epoch milliseconds at document creation time
sequence    = 4*8 DIGIT           ; monotonically increasing counter per document
checksum    = 4HEXDIG             ; first 4 hex chars of SHA-256(prefix + timestamp + sequence)

// Example
"eid": "el-1709251200000-00142-a3f7"
```

Element IDs MUST be unique within a document. IDs MUST NOT be reassigned after generation. Parsers MUST reject documents with duplicate element IDs.

#### 2.4.2 Schema Key Naming Conventions

| Key Type | Convention | Examples | Prohibited |
|---|---|---|---|
| Structural keys | snake_case | `element_type`, `child_elements` | camelCase, PascalCase, kebab-case |
| Element type values | PascalCase string | `"Heading"`, `"LineItem"` | lowercase, snake_case |
| Enum values | SCREAMING_SNAKE_CASE | `"SIGNED"`, `"LEFT_ALIGN"` | lowercase, mixed case |
| Extension namespace keys | `x-{vendor}:{key}` | `"x-acme:priority"` | Unprefixed vendor keys |

### 2.5 Data Type Definitions

| SPDF Type | JSON Representation | Constraints | Notes |
|---|---|---|---|
| spdf:string | JSON string | UTF-8, max 65,535 bytes encoded | BOM (U+FEFF) MUST NOT appear |
| spdf:text_block | JSON string | UTF-8, max 4,194,304 bytes encoded | For paragraph/heading content |
| spdf:integer | JSON number | Integer value, range: -2^53 to 2^53-1 | No decimal point in JSON |
| spdf:decimal | JSON string | Decimal notation, max 20 integer digits, max 6 fractional | `"12345.67"` — string to preserve precision |
| spdf:dimension | JSON number | IEEE 754 double, >= 0.0, max 14400.0 (200 inches in pt) | Points (1/72 inch) unit |
| spdf:color | JSON string | #RRGGBB hex notation, case-insensitive | sRGB color space, always 7 chars |
| spdf:timestamp | JSON string | ISO 8601 UTC: YYYY-MM-DDTHH:MM:SS.sssZ | Timezone MUST be Z (UTC). No local times. |
| spdf:boolean | JSON boolean | `true` or `false` — JSON literals only | Strings "true"/"false" are NOT valid |
| spdf:currency | JSON string | ISO 4217 three-letter code | "USD", "EUR", "INR" |
| spdf:locale | JSON string | BCP 47 language tag | "en-IN", "de-DE", "zh-Hans" |

---

## 3. Design Principles

### 3.1 Deterministic Rendering Principle

Given the same SPDF document, any two conformant Rendering Engines MUST produce pixel-identical output at the same target resolution. This is the most foundational guarantee of the format.

Requirements for deterministic rendering:
- All fonts MUST be embedded in their entirety or subset-embedded with all required glyphs present
- Color values MUST be expressed in sRGB color space within the document
- All dimensional values MUST be expressed in points (1/72 inch)
- Layout arithmetic MUST use IEEE 754 double-precision floating point with results rounded to 4 decimal places
- Line-breaking and word-wrapping rules MUST follow the Unicode Line Breaking Algorithm (UAX #14)

> **Anti-goal:** Engines MUST NOT attempt to optimize layout to match the platform's native rendering conventions. The SPDF-defined layout is authoritative.

### 3.2 Separation of Content, Layout, and Presentation

SPDF strictly separates three orthogonal concerns:

| Layer | File | Contains | What It Does NOT Contain |
|---|---|---|---|
| Content | semantic.json | Element tree, text, data, structure, types | Any positional or visual information |
| Layout | layout.json | Positions, dimensions, page assignments, grids | Any content or semantic meaning |
| Presentation | styles.json | Colors, fonts, spacing, visual decoration | Any positions or content |
| Render | render.pdf | Fused output of all three layers | Used only for backward compatibility |

### 3.3 Machine-Readability First Principle

Every structural decision in SPDF is evaluated first by whether it is unambiguously parseable by a machine.

- Every element MUST declare its type explicitly — type MUST NOT be inferred from context
- Numeric values MUST NOT be stored as formatted strings (e.g., "1,234.56" → invalid; must be "1234.56")
- Dates and times MUST use ISO 8601 UTC format
- References MUST use element IDs — positional references are prohibited
- Enumerated values MUST be drawn from a defined registry

### 3.4 Forward Compatibility Protocol

| Rule | Description |
|---|---|
| FC-1 | A conformant parser encountering an unknown element type MUST preserve the element in the DOM as an opaque node, passing it through to any writer unchanged. |
| FC-2 | A conformant parser encountering an unknown property key in a known element MUST preserve that key-value pair unchanged. |
| FC-3 | A conformant writer MUST NOT discard any preserved opaque elements or unknown keys unless explicitly instructed to strip extensions. |
| FC-4 | A conformant reader MUST NOT fail or produce an error for any document whose major version number matches the reader's supported major version. |

### 3.5 Minimal Parsing Ambiguity Principle

The format is designed to have exactly one valid interpretation for every syntactically valid document.

| Value State | JSON Representation | Semantic Meaning |
|---|---|---|
| Present with value | `"color": "#FF0000"` | The property has the specified value |
| Present and null | `"color": null` | The property explicitly has no value (overrides inheritance) |
| Absent | (key not present) | The property inherits from parent or uses type default |

### 3.6 Defense-in-Depth Security Principle

Security in SPDF is not an optional layer:
- The format PROHIBITS executable content — no JavaScript, no macros, no auto-actions
- The format PROHIBITS external URL resolution in sealed documents
- Parsers MUST enforce size limits before allocating memory
- Cryptographic operations use only NIST-approved algorithms at appropriate key sizes
- The audit log is append-only and cryptographically chained

### 3.7 Fail-Safe Degradation Principle

When a conformant SPDF viewer cannot fully render an SPDF document, it MUST fall back to the render layer (render.pdf) rather than showing a corrupt or partially rendered document. The render layer is always present precisely to guarantee this fallback path.

### 3.8 No Reinvention Principle

Where an existing, well-specified standard solves a problem correctly, SPDF adopts that standard: ZIP for the container, JSON for all structured data, PDF for the render layer, X.509/PKI for digital signatures, RFC 3161 for trusted timestamping.

---

## 4. File Container Architecture

### 4.1 Container Format

An SPDF file is a ZIP archive conforming to the ZIP specification (PKWARE Application Note, version 6.3.10, or later). The file extension SHALL be `.spdf`. SPDF parsers MUST NOT rely on the file extension for format detection; they MUST use the magic bytes defined in Section 4.2.

ZIP compliance requirements:
- The archive MUST use ZIP format version 2.0 or later
- File entries MUST use the Deflate compression method (method ID 8) or Store (method ID 0) for binary assets
- The archive MUST NOT be password-protected at the ZIP level
- File entries MUST use UTF-8 encoded filenames
- All timestamps in ZIP local file headers MUST be set to 1980-01-01 00:00:00 for canonical reproducibility

### 4.2 Magic Bytes and Format Detection

```
// SPDF Magic Bytes (hex)
50 4B 03 04       // ZIP local file header signature
53 50 44 46       // ASCII "SPDF"

// Full detection algorithm
function isSpdf(bytes) {
  if (bytes[0..4] !== [0x50, 0x4B, 0x03, 0x04]) return false;
  const firstEntryName = readZipEntryName(bytes, offset=30);
  if (firstEntryName !== "manifest.json") return false;
  const manifest = parseJson(readZipEntry(bytes, "manifest.json"));
  return manifest["spdf:format"] === "SPDF" && manifest["spdf:version"] !== undefined;
}
```

### 4.3 Required Directory Structure

```
document.spdf                         ← ZIP container
├── manifest.json                     ← REQUIRED: Container manifest
├── semantic.json                     ← REQUIRED: Document Object Model
├── layout.json                       ← REQUIRED: Layout definitions
├── styles.json                       ← REQUIRED: Style definitions
├── render.pdf                        ← REQUIRED: PDF render layer
├── metadata.json                     ← REQUIRED: Document metadata
├── audit.json                        ← REQUIRED: Immutable audit log
├── assets/                           ← REQUIRED directory (may be empty)
│   ├── fonts/
│   ├── images/
│   ├── vectors/
│   └── attachments/
├── signatures/                       ← OPTIONAL: Cryptographic signatures
│   ├── signature_001.json
│   └── certificate_001.pem
├── extensions/                       ← OPTIONAL: Vendor extension data
└── _rels/                            ← REQUIRED: Relationship index
    └── relationships.json
```

**NOTE:** The manifest.json file MUST be the first entry in the ZIP archive to enable streaming detection.

### 4.4 File Descriptions

#### manifest.json

```json
{
  "spdf:format": "SPDF",
  "spdf:version": "1.0",
  "spdf:profile": "full",
  "spdf:created_at": "2025-03-15T09:30:00.000Z",
  "spdf:generator": {
    "name": "spdf-core",
    "version": "1.0.0",
    "vendor": "SPDF Foundation"
  },
  "layers": {
    "semantic": {
      "file": "semantic.json",
      "sha256": "e3b0c44298fc1c149afb....",
      "size_bytes": 48291,
      "encoding": "utf-8"
    },
    "render": {
      "file": "render.pdf",
      "sha256": "5f4d3e2c1b0a9876543...",
      "size_bytes": 152480
    }
  },
  "assets": [
    { "id": "3d4f9a1b2c7e8f0a", "type": "font", "mime": "font/woff2", "sha256": "..." }
  ],
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "manifest_hash": "sha256:a1b2c3d4e5f6..."
}
```

#### audit.json (Hash-Chained Audit Log)

```json
{
  "spdf:audit_version": "1.0",
  "document_id": "spdf-d4b7c2a1-...",
  "entries": [
    {
      "seq": 1,
      "action": "CREATED",
      "at": "2025-03-15T09:30:00.000Z",
      "by": { "type": "SOFTWARE", "name": "ACME Billing v3.2" },
      "state_before": null,
      "state_after": "DRAFT",
      "prev_hash": "0000000000000000000000000000000000000000000000000000000000000000",
      "entry_hash": "sha256:7f3a2c1b..."
    }
  ]
}
```

### 4.5 Container Validation Rules

| Rule ID | Rule | Error Code | Severity |
|---|---|---|---|
| CONT-001 | Container MUST be a valid ZIP archive | E_INVALID_CONTAINER | Fatal |
| CONT-002 | manifest.json MUST be the first ZIP entry | E_MANIFEST_POSITION | Fatal |
| CONT-003 | All required files MUST be present | E_MISSING_REQUIRED_FILE | Fatal |
| CONT-004 | SHA-256 of each file MUST match manifest entry | E_CHECKSUM_MISMATCH | Fatal |
| CONT-005 | spdf:version in manifest MUST be a valid version string | E_INVALID_VERSION | Fatal |
| CONT-006 | manifest.json MUST be valid JSON per RFC 8259 | E_INVALID_JSON | Fatal |
| CONT-007 | Assets referenced in semantic.json MUST exist in assets/ | E_MISSING_ASSET | Error |
| CONT-008 | Assets declared in manifest MUST be referenced in semantic.json | E_UNREFERENCED_ASSET | Warning |
| CONT-009 | Files NOT listed in this spec MUST be in extensions/ directory | E_UNEXPECTED_FILE | Warning |
| CONT-010 | audit.json hash chain MUST be internally consistent | E_AUDIT_CHAIN_BROKEN | Error |
| CONT-011 | Total uncompressed container size MUST NOT exceed 2 GB | E_CONTAINER_TOO_LARGE | Fatal |
| CONT-012 | No file path may contain ".." or absolute path components | E_PATH_TRAVERSAL | Fatal |

---

## 5. Document Object Model (DOM)

### 5.1 DOM Overview

The SPDF DOM is a tree structure representing the complete semantic content of a document. The DOM root is always a Document node. The tree is stored as semantic.json and is the authoritative representation of document content.

```
Document
└── pages[]
     └── Page
          └── elements[]
               ├── Section
               │    └── elements[]
               │         ├── Heading
               │         ├── Paragraph
               │         ├── Table
               │         │    └── rows[]
               │         │         └── TableRow
               │         │              └── cells[]
               │         │                   └── TableCell
               │         ├── Image
               │         ├── InvoiceHeader
               │         ├── LineItemTable
               │         ├── SignatureBlock
               │         └── Group
               │              └── elements[]
               └── PageFooter / PageHeader
```

### 5.2 Document Root Schema

```json
{
  "spdf:semantic_version": "1.0",
  "document": {
    "eid": "doc-1709251200000-00001-0000",
    "element_type": "Document",
    "title": "Invoice #INV-2025-001",
    "locale": "en-IN",
    "direction": "LTR",
    "document_state": "DRAFT",
    "page_count": 2,
    "component_registry": {},
    "variable_bindings": {},
    "pages": []
  }
}
```

### 5.3 Element Type Registry

| Element Type | Category | Leaf? | Description |
|---|---|---|---|
| Document | Root | No | Root node. Exactly one per document. |
| Page | Structure | No | Represents one physical page. |
| PageHeader | Structure | No | Repeating header region. |
| PageFooter | Structure | No | Repeating footer region. |
| Section | Structure | No | Logical division of content. |
| Group | Layout | No | Non-semantic container for layout grouping. |
| Column | Layout | No | Vertical column in a multi-column layout. |
| Heading | Content | Yes | Heading text at levels 1–6. |
| Paragraph | Content | Yes | Block of body text with inline formatting. |
| Table | Content | No | Structured data table with headers and rows. |
| TableRow | Content | No | Row within a Table. |
| TableCell | Content | No | Cell within a TableRow. May contain nested elements. |
| Image | Media | Yes | Raster image asset reference. |
| VectorImage | Media | Yes | SVG vector image asset reference. |
| CodeBlock | Content | Yes | Monospaced code or preformatted text. |
| HorizontalRule | Decoration | Yes | Horizontal divider line. |
| PageBreak | Layout | Yes | Explicit page break directive. |
| Attachment | Data | Yes | Binary file attachment declaration. |
| InvoiceHeader | Domain | No | Structured invoice header. |
| LineItem | Domain | Yes | Single line item in an invoice or purchase order. |
| LineItemTable | Domain | No | Table of LineItems with computed totals. |
| PaymentTerms | Domain | Yes | Structured payment terms and due date. |
| SignatureBlock | Trust | Yes | Signature placeholder or completed signature. |
| Stamp | Trust | Yes | Official stamp or seal element. |
| FormField | Interactive | Yes | Data input field (in DRAFT state only). |
| Annotation | Review | Yes | Comment or annotation on a document region. |
| Redaction | Security | Yes | Record of cryptographic erasure of removed content. |
| VariablePlaceholder | Template | Yes | Binding point in a Template document. |

### 5.4 Universal Element Properties

All elements, regardless of type, MUST support the following properties:

```json
{
  "eid":            "el-1709251200000-00014-c2f9",
  "element_type":   "Heading",
  "version":        1,
  "created_at":     "2025-03-15T09:30:00.000Z",
  "modified_at":    "2025-03-15T09:35:14.221Z",
  "style_id":       "heading-primary",
  "integrity_hash": "sha256:a1b2c3...",
  "locked":         false,
  "visible":        true,
  "accessible_label": null,
  "custom_data":    null,
  "x_extensions":   {}
}
```

### 5.5 Domain-Specific Element Schemas

#### 5.5.1 Heading Element

```json
{
  "element_type": "Heading",
  "eid": "el-1709251200000-00020-b3a1",
  "level": 1,
  "text": "Invoice Summary",
  "numbering": null,
  "anchor_id": "section-invoice-sum"
}
```

#### 5.5.2 Table Element

```json
{
  "element_type": "Table",
  "eid": "el-1709251200000-00031-d4e5",
  "header_rows": 1,
  "footer_rows": 0,
  "col_definitions": [
    { "id": "col-desc",  "header": "Description", "width_pct": 45.0, "data_type": "text"    },
    { "id": "col-qty",   "header": "Qty",         "width_pct": 10.0, "data_type": "decimal" },
    { "id": "col-price", "header": "Unit Price",  "width_pct": 20.0, "data_type": "decimal" },
    { "id": "col-total", "header": "Total",       "width_pct": 25.0, "data_type": "decimal" }
  ],
  "rows": [
    {
      "element_type": "TableRow",
      "row_type": "DATA",
      "cells": [
        { "element_type": "TableCell", "col_id": "col-qty",   "value": "3",         "data_type": "decimal" },
        { "element_type": "TableCell", "col_id": "col-price", "value": "5000.00",   "currency": "INR" },
        { "element_type": "TableCell", "col_id": "col-total", "value": "15000.00",  "currency": "INR", "formula": "qty * price" }
      ]
    }
  ]
}
```

#### 5.5.3 SignatureBlock Element

```json
{
  "element_type": "SignatureBlock",
  "eid": "el-1709251200000-00089-e2b7",
  "required_from": {
    "role": "CLIENT",
    "name": null,
    "email": null
  },
  "signature_state": "PENDING",
  "signed_at": null,
  "signed_by": null,
  "signature_image_asset": null,
  "signature_hash": null,
  "lock_on_sign": true,
  "elements_locked": []
}
```

---

## 6. Content Model

### 6.1 Text Content Rules

**Character Encoding:** All text content in SPDF MUST be encoded as UTF-8 (RFC 3629). The Byte Order Mark (U+FEFF) MUST NOT appear in any text property value.

**Inline Formatting Spans:**

```json
{
  "element_type": "Paragraph",
  "text": null,
  "inline_spans": [
    { "text": "Payment is due ",       "style": null },
    { "text": "within 30 days",        "style": { "bold": true, "color": "#B71C1C" } },
    { "text": " from invoice date.",   "style": null }
  ]
}

// Supported inline style properties:
{
  "bold":          true,
  "italic":        false,
  "underline":     "none",        // "none" | "single" | "double" | "dotted"
  "strikethrough": false,
  "superscript":   false,
  "subscript":     false,
  "font_family":   null,
  "font_size":     12.0,          // points
  "color":         "#RRGGBB",
  "background":    null
}
```

### 6.2 Numeric and Financial Content

```
// Decimal values MUST be stored as JSON strings, NOT JSON numbers
// Reason: JSON number parsing loses precision for values like 0.1 + 0.2

// CORRECT
"unit_price": "1234.50"
"tax_rate": "18.00"
"total": "1456.71"

// INCORRECT — parser cannot guarantee precision
"unit_price": 1234.50
"tax_rate": 18
```

**NOTE:** Financial calculations MUST be performed using the values in the semantic layer, not values extracted from the render layer.

### 6.3 Image Content Rules

| Property | Rule |
|---|---|
| Supported raster formats | PNG (ISO 15948), JPEG (ISO 10918-1), JPEG 2000 (ISO 15444-1), WebP |
| Maximum image dimensions | 16,384 × 16,384 pixels per image |
| Maximum single image file size | 50 MB uncompressed; 25 MB compressed |
| Color profiles | sRGB is the canonical color space. CMYK images MUST be converted to sRGB during write |
| Alpha channel | Supported in PNG and WebP |
| Alt text (accessibility) | The `alt_text` property is REQUIRED on all Image elements |
| Asset reference | Images MUST be embedded as assets. External URL references are NOT permitted in sealed documents |

### 6.4 Font Content Rules

- All fonts used in a document MUST be embedded in the assets/fonts/ directory
- Subset embedding is REQUIRED when the font contains more than 512 glyphs
- Supported font formats: WOFF2 (preferred), TrueType (.ttf), OpenType (.otf)
- CFF (Type 1) fonts are NOT supported in SPDF v1.0
- Variable fonts (OpenType 1.8+) are SUPPORTED

---

## 7. Layout System

### 7.1 Coordinate System

- Origin (0, 0) is the top-left corner of the page content area (inside margins)
- The positive x-axis extends to the right
- The positive y-axis extends downward
- All values are in points (pt): 1 pt = 1/72 inch
- Floating-point arithmetic uses IEEE 754 double precision; results are rounded to 4 decimal places before storage

### 7.2 Page Model

```json
{
  "pages": [
    {
      "page_number": 1,
      "page_eid": "el-...-00002-a1b2",
      "size": {
        "width": 595.28,
        "height": 841.89
      },
      "orientation": "PORTRAIT",
      "margins": {
        "top":    56.69,
        "right":  56.69,
        "bottom": 56.69,
        "left":   56.69
      },
      "bleed": null,
      "grid": {
        "columns": 1,
        "gutter":  14.17
      },
      "background_color": null,
      "watermark_ref": null
    }
  ]
}
```

### 7.3 Standard Page Sizes

| Name | Width (pt) | Height (pt) | Width (mm) | Height (mm) | Common Use |
|---|---|---|---|---|---|
| A4 | 595.28 | 841.89 | 210.0 | 297.0 | International standard (EU, India, most of world) |
| Letter | 612.00 | 792.00 | 215.9 | 279.4 | United States, Canada |
| Legal | 612.00 | 1008.00 | 215.9 | 355.6 | US legal documents |
| A3 | 841.89 | 1190.55 | 297.0 | 420.0 | Large format drawings |
| Executive | 521.86 | 756.00 | 184.1 | 266.7 | US executive stationery |
| CUSTOM | Specified | Specified | min: 72pt | max: 14400pt | Custom dimensions |

### 7.4 Element Layout Properties

```json
// layout.json — element layout entries
{
  "elements": {
    "el-1709251200000-00014-c2f9": {
      "page": 1,
      "x": 0.0,
      "y": 42.52,
      "width": 483.31,
      "height": 28.35,
      "z_index": 0,
      "overflow": "VISIBLE",
      "padding": { "top": 0, "right": 0, "bottom": 8.50, "left": 0 }
    }
  }
}
```

### 7.5 Text Layout Rules

**Text Alignment Rules:**

| Alignment Value | Behavior | Last Line Behavior |
|---|---|---|
| LEFT | Left-align all lines | N/A |
| RIGHT | Right-align all lines | N/A |
| CENTER | Center all lines within container width | N/A |
| JUSTIFY | Full justification via inter-word spacing | Last line is LEFT-aligned |
| JUSTIFY_ALL | Full justification including last line | Last line also justified |
| START | Align to writing direction start | N/A |
| END | Align to writing direction end | N/A |

### 7.6 Pagination Rules

- **Orphan lines:** A paragraph MUST have at least 2 lines remaining at the bottom of a page.
- **Widow lines:** A paragraph MUST have at least 2 lines on the new page.
- `keep_together: true` — the entire element must appear on one page
- `keep_with_next: true` — this element must appear on the same page as the immediately following sibling
- Elements of type SignatureBlock, Table (< 5 rows), and InvoiceHeader MUST default to `keep_together: true`

---

## 8. Rendering Model

### 8.1 Rendering Pipeline

```
[1] Container Validation
    Input:  .spdf file bytes
    Output: Validated container, all checksums confirmed

[2] Manifest Parse
    Input:  manifest.json bytes
    Output: Manifest object, version confirmed, layers listed

[3] Semantic Layer Parse
    Input:  semantic.json bytes
    Output: In-memory DOM (typed element tree)

[4] Layout Layer Parse
    Input:  layout.json bytes + DOM
    Output: DOM with layout annotations applied

[5] Style Resolution
    Input:  styles.json + DOM with layout
    Output: Fully resolved style for every element

[6] Asset Loading
    Input:  Asset declarations from DOM, assets/ directory
    Output: Decoded fonts and images loaded into memory

[7] Layout Engine
    Input:  Styled DOM with layout annotations
    Output: Final box model — every element has computed bounds

[8] Renderer
    Input:  Computed box model + loaded assets
    Output: Page rasters (screen) or PDL commands (print)

FALLBACK PATH (if any stage 1-7 fails):
Container → render.pdf → PDF Viewer (standard PDF rendering path)
```

### 8.2 Style Resolution Algorithm

Style properties are resolved using a cascade with the following priority order (highest priority first):
1. Element inline style (properties directly on the element in semantic.json)
2. Named style from styles.json referenced by `style_id`
3. Parent element computed style (inheritance — only inheritable properties)
4. Document-level default styles from styles.json `defaults` block
5. SPDF built-in type defaults

**Inheritable Style Properties:**

| Property | Type | Built-in Default |
|---|---|---|
| font_family | string | "Inter", "Helvetica", "Arial", sans-serif |
| font_size | spdf:dimension | 11.0 pt |
| font_weight | integer 100–900 | 400 (Regular) |
| color | spdf:color | "#1A1A1A" |
| line_height | decimal multiplier | 1.4 |
| text_align | enum | LEFT |
| direction | enum | LTR |

### 8.3 Render Layer Generation

The render layer (render.pdf) is a conformant PDF 2.0 document generated deterministically from the SPDF DOM.

Requirements:
- The render layer MUST be a conformant PDF 2.0 file (ISO 32000-2)
- The render layer MUST NOT contain any JavaScript, form fields, or active elements
- The render layer MUST embed all fonts referenced in the document
- The render layer MUST use the PDF coordinate system (origin bottom-left) mapped from SPDF coordinates (origin top-left)

### 8.4 Screen Rendering Requirements

| Requirement | Target | Tolerance |
|---|---|---|
| Glyph position accuracy | Matches layout.json coordinates | ±0.5 pt |
| Color accuracy | sRGB values match styles.json | ±2 in each 8-bit channel |
| Line/border width | Matches styles.json border width | ±0.25 pt |
| Font metrics | Must use embedded font metrics, not system font metrics | Zero deviation |
| Color contrast | Foreground/background MUST be at least 4.5:1 (WCAG 2.1 AA) | — |

---

## 9. Asset Management

### 9.1 Asset Identification

Every asset in an SPDF document is identified by a content-addressed ID: the first 16 hexadecimal characters of the SHA-256 hash of the asset's raw binary content.

```javascript
// Asset ID computation
function computeAssetId(binaryContent) {
  const fullHash = sha256(binaryContent);   // 64 hex characters
  return fullHash.slice(0, 16);             // first 16 hex characters
}

// Asset storage paths
// assets/fonts/3d4f9a1b2c7e8f0a.woff2
// assets/images/7b2a8c3d4e9f1a0b.png
// assets/vectors/1a2b3c4d5e6f7a8b.svg
// assets/attachments/9f8e7d6c5b4a3210.pdf
```

### 9.2 Supported Asset Formats

| Asset Type | Format | MIME Type | Max Size | Notes |
|---|---|---|---|---|
| Raster Image | PNG | image/png | 25 MB | Lossless; preferred for logos |
| Raster Image | JPEG | image/jpeg | 25 MB | Lossy; preferred for photographs |
| Raster Image | WebP | image/webp | 25 MB | Modern; smaller than PNG at equivalent quality |
| Vector Graphic | SVG 1.1 | image/svg+xml | 5 MB | MUST be static SVG; scripts and animations are stripped |
| Font | WOFF2 | font/woff2 | 10 MB | Preferred format; best compression |
| Font | TrueType | font/ttf | 15 MB | Widely compatible |
| Font | OpenType | font/otf | 15 MB | Full feature set |
| Attachment | Any IANA type | per file | 100 MB | No execution; rendered only by external apps |
| ICC Profile | ICC | application/vnd.iccprofile | 1 MB | Color management profile for print output |

### 9.3 SVG Asset Sanitization

Before embedding, SVG assets MUST be sanitized by the SPDF writer:
- Remove all `<script>` elements and `on*` event attributes
- Remove all `<foreignObject>` elements
- Remove all external resource references
- Remove all animation elements (`<animate>`, `<animateTransform>`, etc.)
- Remove `<use>` elements with external href targets
- Preserve all static visual elements, paths, shapes, text, gradients, and filters

> **Security Requirement:** If the SVG cannot be fully sanitized without changing its visual output, the writer MUST refuse to embed it and MUST return an E_ASSET_UNSAFE error.

### 9.4 Asset Compression Rules

| Asset Type | ZIP Method | Rationale |
|---|---|---|
| PNG images | Store (0) | PNG is already DEFLATE-compressed |
| JPEG images | Store (0) | JPEG is already compressed; re-compression corrupts data |
| WOFF2 fonts | Store (0) | WOFF2 uses Brotli compression internally |
| TTF/OTF fonts | Deflate (8) | Uncompressed binary; significant size reduction possible |
| SVG vectors | Deflate (8) | XML text; typically 60–80% reduction |
| Attachments | Store (0) | Content type unknown; avoid re-compression overhead |

---

## 10. Metadata System

### 10.1 metadata.json Schema

```json
{
  "spdf:metadata_version": "1.0",

  // Core Document Identity
  "spdf:document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "spdf:title":       "Invoice #INV-2025-001",
  "spdf:description": "B2B invoice for software development services",

  // Authorship
  "spdf:author":           "ACME Software Solutions Pvt. Ltd.",
  "spdf:author_id":        "org-f3e2d1c0-b9a8-7654-3210-fedcba987654",

  // Temporal
  "spdf:created_at":       "2025-03-15T09:30:00.000Z",
  "spdf:modified_at":      "2025-03-15T14:22:10.441Z",
  "spdf:valid_from":       "2025-03-15T00:00:00.000Z",
  "spdf:valid_until":      "2025-04-14T23:59:59.999Z",

  // Localization
  "spdf:locale":           "en-IN",
  "spdf:direction":        "LTR",
  "spdf:currency":         "INR",
  "spdf:timezone":         "Asia/Kolkata",

  // Classification
  "spdf:document_type":    "Invoice",
  "spdf:document_subtype": "B2B_Service",
  "spdf:keywords":         ["invoice", "software", "services", "2025"],

  // Access and Policy
  "spdf:access_level":     "CONFIDENTIAL",
  "spdf:retention_days":   2557,
  "spdf:gdpr_data_classes": ["contact_info", "financial_data"],
  "spdf:legal_basis":      "contract",

  // Provenance
  "spdf:source_system":    "ACME Billing v3.2",
  "spdf:schema_version":   "1.0",

  // Application-defined metadata
  "x-acme:client_id":      "client-00219",
  "x-acme:project_code":   "PROJ-2025-44"
}
```

### 10.2 Standard Metadata Keys

| Key | Type | Required? | Description |
|---|---|---|---|
| spdf:document_id | spdf:string (UUID) | REQUIRED | Globally unique document identifier, immutable once set |
| spdf:title | spdf:string | REQUIRED | Human-readable document title |
| spdf:author | spdf:string | REQUIRED | Primary author name or organization |
| spdf:created_at | spdf:timestamp | REQUIRED | Document creation timestamp, UTC, immutable |
| spdf:modified_at | spdf:timestamp | REQUIRED | Last modification timestamp, UTC |
| spdf:locale | spdf:locale | REQUIRED | Primary language/locale of document content |
| spdf:document_type | spdf:string | REQUIRED | Document type from the type registry |
| spdf:schema_version | spdf:string | REQUIRED | SPDF version used to create this document |

### 10.3 Document Type Registry

| Type Value | Description |
|---|---|
| Invoice | B2B or B2C commercial invoice |
| PurchaseOrder | Request to purchase goods or services |
| CreditNote | Reduction of a previous invoice |
| DebitNote | Increase of a previous invoice |
| Statement | Account statement or balance summary |
| Contract | Legal contract or agreement |
| Certificate | Official certificate of any kind |
| Report | Analytical or operational report |
| Form | Data collection form |
| Proposal | Business proposal or quotation |
| General | Unclassified general document |

### 10.4 PII Element Tagging

```json
{
  "element_type": "Paragraph",
  "eid": "el-...-00041-b3c2",
  "text": "Priya Mehta — priya.mehta@acme.com — +91-98765-43210",
  "pii_class": ["personal_name", "email_address", "phone_number"],
  "gdpr_basis": "contract",
  "erasable": true
}

// Standard pii_class values:
// "personal_name", "email_address", "phone_number", "postal_address",
// "ip_address", "date_of_birth", "national_id", "financial_account", "health_data"
```

---

## 11. Security Model

### 11.1 Security Architecture Overview

| Level | Mechanism | What It Protects | When Applied |
|---|---|---|---|
| L1 — Format | No executable content in spec | System from document-based exploits | Always — by definition |
| L2 — Container | SHA-256 checksums in manifest | Individual files from modification after writing | All documents at write time |
| L3 — Element | Per-element integrity hash | Individual elements from modification after hashing | Selectively applied |
| L4 — Document | X.509 digital signature | Entire document from modification after signing | SIGNED and CERTIFIED states only |

### 11.2 Document Hashing Algorithm

```javascript
// SPDF Canonical Form Algorithm

// Step 1: Select inputs
// Includes: semantic.json, layout.json, styles.json, metadata.json
// Excludes: render.pdf (derived), audit.json (append-only), manifest.json (self-referential)

// Step 2: Apply RFC 8785 JSON Canonicalization Scheme (JCS) to each file
const canonical_semantic  = jcs(semanticJson);
const canonical_layout    = jcs(layoutJson);
const canonical_styles    = jcs(stylesJson);
const canonical_metadata  = jcs(metadataJson);

// Step 3: Compute SHA-256 of each canonical form
const h_semantic  = sha256(canonical_semantic);
const h_layout    = sha256(canonical_layout);
const h_styles    = sha256(canonical_styles);
const h_metadata  = sha256(canonical_metadata);

// Step 4: Concatenate hashes in deterministic order
const composite = h_semantic + h_layout + h_styles + h_metadata;

// Step 5: Compute document root hash
const document_hash = sha256(composite);

// Step 6: Sign the document root hash
const signature = rsa_pss_sign(document_hash, private_key, {
  hash: "SHA-256",
  salt_length: 32,
  padding: "PSS"
});
```

### 11.3 Digital Signature Schema

```json
{
  "spdf:sig_version": "1.0",
  "signature_id": "sig-a1b2c3d4-...",
  "document_id": "spdf-d4b7c2a1-...",
  "signer": {
    "type": "PERSON",
    "name": "Arjun Sharma",
    "email": "arjun.sharma@acme.com",
    "title": "Authorized Signatory",
    "organization": "ACME Software Solutions Pvt. Ltd."
  },
  "signed_at": "2025-03-15T14:22:10.441Z",
  "document_hash": "sha256:a1b2c3d4e5f6...",
  "signature_algorithm": "RSA-PSS-SHA256",
  "signature_value": "base64:MEUCIQDj...",
  "certificate": {
    "subject_dn": "CN=Arjun Sharma, O=ACME Software, C=IN",
    "issuer_dn": "CN=DigiCert Document Signing CA, O=DigiCert",
    "not_before": "2024-01-01T00:00:00.000Z",
    "not_after": "2026-01-01T00:00:00.000Z",
    "fingerprint_sha256": "sha256:...",
    "pem_file": "signatures/certificate_001.pem"
  },
  "timestamp_token": null,
  "scope": {
    "type": "FULL_DOCUMENT",
    "locked_eids": []
  }
}
```

### 11.4 Document State Machine

```
                  ┌──────────┐
         ┌────────▶│  DRAFT   │◀─────────────┐
         │         └────┬─────┘              │
         │              │ submit_for_review   │ reject/unreject
         │         ┌────▼─────┐              │
         │         │  REVIEW  │──────────────┘
         │         └────┬─────┘
         │              │ sign(X.509 cert)
         │         ┌────▼─────┐
         │         │  SIGNED  │  (immutable — no reverse transition)
         │         └────┬─────┘
         │              │ certify(RFC 3161 timestamp)
         │         ┌────▼──────────┐
         │         │  CERTIFIED    │  (immutable — highest trust)
         │         └───────────────┘
```

**FORBIDDEN TRANSITIONS:**
- SIGNED → DRAFT: Prohibited. Breaking cryptographic seal.
- CERTIFIED → *: No transition from CERTIFIED. Ever.

### 11.5 Cryptographic Erasure (True Redaction)

```json
// Redaction element (written in place of erased elements)
{
  "element_type": "Redaction",
  "eid": "el-...-00199-e3f4",
  "erased_eids": ["el-...-00041-b3c2"],
  "reason": "GDPR Article 17 — Right to Erasure",
  "erased_at": "2025-06-01T08:00:00.000Z",
  "erased_by": { "type": "USER", "name": "Data Protection Officer" },
  "proof_hash": "sha256:content_was_here_but_is_now_gone...",
  "replacement_text": "[PII ERASED]"
}
```

### 11.6 Encryption

| Property | Specification |
|---|---|
| Algorithm | AES-256-GCM (NIST SP 800-38D) |
| Key derivation (password-based) | PBKDF2-SHA256, minimum 310,000 iterations |
| Key derivation (certificate-based) | RSA-OAEP-SHA256 key wrapping |
| IV/Nonce | 96-bit (12-byte) random nonce per encryption operation; MUST NOT be reused |

---

## 12. Versioning and Compatibility

### 12.1 SPDF Version Number Format

```
version = MAJOR "." MINOR
MAJOR = non-negative integer
MINOR = non-negative integer

Examples:
"spdf:version": "1.0"    // version 1.0
"spdf:version": "1.3"    // version 1.3
"spdf:version": "2.0"    // version 2.0 (new major)

// MAJOR version increment: breaking changes to schema or behavior
// MINOR version increment: backward-compatible additions only
```

### 12.2 Version Compatibility Matrix

| Reader Version | Document Version | Behavior | Conformance |
|---|---|---|---|
| 1.0 | 1.0 | Full support — all features available | Full |
| 1.3 | 1.0 | Forward-read: newer reader opens older doc | Required |
| 1.0 | 1.3 | Backward-read: older reader opens newer doc — works for known elements | Required |
| 1.0 | 2.0 | Major version mismatch — reader MUST display upgrade prompt, render via render.pdf | Required |
| 2.0 | 1.0 | Reader opens older major version — MUST support with deprecation warnings | Required |

### 12.3 Breaking Change Policy

**What triggers a MAJOR version bump:**
- Renaming or removing a required property
- Changing the semantics of an existing property
- Removing a required file from the container
- Changing the container format
- Changing the canonical form algorithm (would invalidate all existing signatures)
- Removing or renaming an existing element type

### 12.4 Conformance Profiles

| Profile | Description | Required Capabilities |
|---|---|---|
| spdf:minimal | Read-only viewer; render layer fallback only | Parse manifest; open render.pdf |
| spdf:reader | Full semantic parsing and screen rendering | Parse full DOM; layout resolution; screen renderer |
| spdf:writer | Can produce conformant SPDF documents | All reader + DOM construction; render.pdf generation |
| spdf:signer | Can sign and verify SPDF documents | All writer + X.509 operations; state machine enforcement |
| spdf:full | All capabilities including extensions | All signer + all optional features |

---

## 13. Error Handling and Validation

### 13.1 Error Severity Levels

| Severity | Code Prefix | Definition | Required Handler Behavior |
|---|---|---|---|
| Fatal | F_ | The document cannot be safely processed | MUST abort processing; MUST NOT render partial document |
| Error | E_ | A specific feature is broken; rest of document may be valid | MUST log; SHOULD render document with affected area clearly marked as error |
| Warning | W_ | Deviation from spec that does not prevent processing | SHOULD log; MAY continue silently in production mode |
| Info | I_ | Informational diagnostic | MAY log for debugging |

### 13.2 Error Code Registry

**Fatal Errors — Container and Format:**

| Code | Error | Description |
|---|---|---|
| F_INVALID_CONTAINER | Invalid ZIP archive | The file is not a valid ZIP archive or is corrupted |
| F_MANIFEST_MISSING | manifest.json not found | manifest.json is absent from the container root |
| F_MANIFEST_POSITION | manifest.json not first entry | manifest.json must be the first ZIP entry |
| F_INVALID_JSON | Invalid JSON in required file | A required JSON file contains malformed JSON |
| F_INVALID_VERSION | Unrecognized format version | spdf:version is absent, malformed, or from an unsupported major version |
| F_CHECKSUM_MISMATCH | File integrity failure | SHA-256 of a file does not match its manifest entry |
| F_PATH_TRAVERSAL | Path traversal attempt | A file path contains .. or absolute path components |
| F_CONTAINER_TOO_LARGE | Container exceeds size limit | Uncompressed container exceeds 2 GB |
| F_MISSING_REQUIRED_FILE | Required file absent | A required file (semantic.json, layout.json, etc.) is missing |
| F_DUPLICATE_EID | Duplicate element ID | Two or more elements share the same eid value |
| F_CIRCULAR_REFERENCE | Circular element reference | An element's child chain contains a cycle |
| F_DOM_TOO_LARGE | DOM exceeds limit | DOM exceeds 1,000,000 elements |
| F_NESTING_TOO_DEEP | Element nesting exceeds 64 levels | Nesting depth limit exceeded |

**Errors — Schema and Content:**

| Code | Error | Description |
|---|---|---|
| E_MISSING_REQUIRED_PROP | Missing required property | A required property is absent from an element |
| E_INVALID_TYPE | Invalid property value type | A property value does not match its declared SPDF type |
| E_INVALID_ENUM | Unknown enum value | A property uses a value not in the declared enum registry |
| E_ASSET_MISSING | Referenced asset not found | An element references an asset ID that is not in the container |
| E_ASSET_UNSAFE | Asset failed security check | An SVG asset could not be safely sanitized |
| E_SIGNATURE_INVALID | Cryptographic signature failure | The document hash does not match the signature value |
| E_AUDIT_CHAIN_BROKEN | Audit log chain broken | An audit entry's prev_hash does not match the preceding entry's entry_hash |
| E_LOCKED_ELEMENT_MODIFIED | Write to locked element | An attempt was made to modify an element with locked: true |
| E_STATE_TRANSITION_INVALID | Invalid document state transition | The requested state transition is not permitted by the state machine |

### 13.3 Validation Modes

**Strict Validation (Write Mode):** ALL errors, including warnings, are treated as errors and MUST cause generation to fail.

**Lenient Validation (Read Mode):** Tolerates unknown extension keys and element types (preserved as opaque data), and minor version mismatches. MUST still treat all F_ and E_ codes as non-recoverable.

---

## 14. Performance Considerations

### 14.1 Performance Targets

| Operation | Document Size | Target Time | Memory Limit |
|---|---|---|---|
| Container validation | Any | < 100ms | < 10 MB |
| Full DOM parse | 50 pages | < 200ms | < 50 MB |
| Full DOM parse | 500 pages | < 1,500ms | < 400 MB |
| Render to screen (first page) | 50 pages | < 300ms | < 100 MB |
| Render to PDF (full document) | 50 pages | < 1,000ms | < 200 MB |
| Element extraction (any) | Any | < 50ms | < 5 MB |
| Semantic diff (2 documents) | 50 pages each | < 2,000ms | < 100 MB |
| Signature verification | Any | < 200ms | < 10 MB |

### 14.2 Maximum Allocation Limits

Implementations MUST enforce allocation limits to prevent denial-of-service from maliciously crafted documents:

| Resource | Maximum Allocation | Behavior on Overflow |
|---|---|---|
| DOM node count | 1,000,000 elements per document | F_DOM_TOO_LARGE: abort parse |
| Text content per element | 4 MB | E_CONTENT_TOO_LARGE: truncate with warning |
| Image decompressed size | 256 MB per image | E_ASSET_TOO_LARGE: skip image, show placeholder |
| Font decompressed size | 32 MB per font | E_ASSET_TOO_LARGE: substitute fallback font |
| Attachment size | 100 MB per attachment | E_ATTACHMENT_TOO_LARGE: declare but do not load |
| Nesting depth (element tree) | 64 levels deep | F_NESTING_TOO_DEEP: abort parse |
| Table rows | 100,000 rows per table | E_TABLE_TOO_LARGE: truncate with warning |

---

## 15. Extensibility

### 15.1 Extension Philosophy

Extensions in SPDF are strictly additive: they may add new element types, add new properties to existing types, or add new container files. Extensions MUST NOT modify the semantics of existing element types, override required properties, or replace existing container files.

> **THE GOLDEN RULE OF SPDF EXTENSIONS:** A document containing extensions MUST be fully functional for non-extension-aware implementations. If removing all extension data from a document breaks its semantic meaning, the extension is violating this specification.

### 15.2 Extension Namespaces

```json
// manifest.json — extension namespace declarations
"extensions": [
  {
    "namespace": "x-acme",
    "name": "ACME Corporation Document Extensions",
    "version": "2.1",
    "uri": "https://docs.acme.com/spdf-extensions/v2",
    "required": false,
    "files": [
      "extensions/x-acme/acme_data.json"
    ]
  }
]

// Namespace rules:
// VALID:   "x-acme", "x-google", "x-gov-in"
// INVALID: "acme" (missing x- prefix), "x-spdf" (reserved), "x-" (empty vendor)
```

### 15.3 Custom Element Types

```json
// Custom element type example
{
  "element_type": "x-acme:PriorityBadge",
  "eid": "el-...-00210-f4a3",
  "x-acme:priority_level": "HIGH",
  "x-acme:escalation_date": "2025-04-01T00:00:00.000Z",
  // Universal properties still required
  "version": 1,
  "created_at": "2025-03-15T09:30:00.000Z",
  "modified_at": "2025-03-15T09:30:00.000Z",
  "locked": false,
  "visible": true,
  // Fallback rendering hint for non-extension renderers
  "x_render_fallback": {
    "type": "Paragraph",
    "text": "[PRIORITY: HIGH — Escalation: 2025-04-01]"
  }
}
```

---

## 16. Reference Implementation Guidelines

### 16.1 Implementation Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                    SPDF IMPLEMENTATION STACK                  │
├──────────────────────────────────────────────────────────────┤
│  Layer 5: SDK / Developer API                                 │
│           Document builder, template engine, extraction API   │
├──────────────────────────────────────────────────────────────┤
│  Layer 4: Security Layer                                      │
│           Signing, verification, encryption, audit log        │
├──────────────────────────────────────────────────────────────┤
│  Layer 3: Rendering Engine                                    │
│           Layout calculation, glyph placement, PDF write      │
├──────────────────────────────────────────────────────────────┤
│  Layer 2: Style Engine                                        │
│           Style cascade, inheritance resolution               │
├──────────────────────────────────────────────────────────────┤
│  Layer 1: Core Parser / Writer                                │
│           ZIP I/O, JSON parse, schema validation, DOM build   │
└──────────────────────────────────────────────────────────────┘
```

### 16.2 Recommended Rust Crates

| Crate | Version | Purpose |
|---|---|---|
| serde + serde_json | 1.0.x | JSON serialization and deserialization |
| zip | 0.6.x | ZIP container read/write with streaming support |
| sha2 | 0.10.x | SHA-256 hashing |
| rsa + pkcs8 | 0.9.x | RSA-PSS-SHA256 signing and verification |
| x509-cert | 0.2.x | X.509 certificate parsing and validation |
| fontdue | 0.7.x | Font parsing, metric extraction, and glyph rasterization |
| image | 0.24.x | Raster image decoding (PNG, JPEG, WebP) |
| resvg | 0.35.x | SVG rasterization |
| lopdf | 0.28.x | PDF 2.0 document generation (render layer) |
| wasm-bindgen | 0.2.x | WebAssembly compilation and JavaScript interop |
| ring | 0.17.x | AES-256-GCM encryption |

### 16.3 Parser Security Requirements

- Parse the manifest FIRST before allocating memory for other layers
- Apply all allocation limits BEFORE parsing content
- Validate element IDs for uniqueness as they are parsed
- Reject ZIP entries with path components containing `..` or absolute paths BEFORE extracting
- Limit ZIP decompression to 2 GB total
- Sanitize all SVG assets during loading (defense in depth)

---

## 17. SDK and Developer APIs

### 17.1 Python SDK API

#### Document Generation

```python
from spdf import Document, Page
from spdf.elements import Heading, Paragraph, Table
from spdf.elements.domain import InvoiceHeader, LineItemTable, PaymentTerms
from spdf.styles import Style, StyleSheet

# Create Document
doc = Document(
    title="Invoice #INV-2025-001",
    locale="en-IN",
    document_type="Invoice",
    author="ACME Software Solutions"
)

# Apply Brand Stylesheet
doc.styles.load("acme_invoice_v2.spdf-style")

# Page 1
page = Page(size="A4")

# Invoice header
header = InvoiceHeader(
    invoice_number="INV-2025-001",
    issue_date="2025-03-15",
    due_date="2025-04-14",
    vendor={
        "name": "ACME Software Solutions Pvt. Ltd.",
        "address": "B-401, Tech Park, Pune 411057",
        "gstin": "27AABCA1234F1Z5"
    },
    client={
        "name": "GlobalTech India Pvt. Ltd.",
        "gstin": "29AACG5678H1ZQ"
    }
)
page.add(header)

# Line items
invoice_table = LineItemTable(
    currency="INR",
    tax_scheme="GST_18",
    items=[
        { "description": "Backend API Development", "qty": 80, "unit": "hours",
          "unit_price": "2500.00", "hsn_code": "998314" },
        { "description": "DevOps Setup & Configuration", "qty": 40, "unit": "hours",
          "unit_price": "2000.00", "hsn_code": "998315" },
    ]
)
page.add(invoice_table)

doc.add_page(page)

# Export
doc.export.spdf("invoice_INV-2025-001.spdf")
doc.export.pdf("invoice_INV-2025-001.pdf")
doc.export.json("invoice_INV-2025-001.json")
```

#### Data Extraction API

```python
from spdf import Document

doc = Document.open("received_invoice.spdf")

# Direct Element Access
header = doc.find_first(element_type="InvoiceHeader")
print(header.invoice_number)     # "INV-2025-001"
print(header.due_date)           # datetime(2025, 4, 14)

# Table Extraction
line_table = doc.find_first(element_type="LineItemTable")
for item in line_table.items:
    print(f"{item.description}: {item.qty} × {item.unit_price} = {item.total}")

# Financial Summary
summary = doc.financial_summary()
print(summary.subtotal)    # Decimal("200000.00")
print(summary.tax_amount)  # Decimal("36000.00")
print(summary.total)       # Decimal("236000.00")

# Structured Export
as_dict = doc.extract.to_dict()
as_json = doc.extract.to_json()
as_df   = doc.extract.line_items_dataframe()   # pandas DataFrame

# XPath-like Query
totals = doc.query("//LineItemTable//TableCell[@data_type='decimal']")
sig_blocks = doc.query("//SignatureBlock[@signature_state='PENDING']")
```

### 17.2 TypeScript SDK API

```typescript
import { Document, Page, InvoiceHeader, LineItemTable, SignatureBlock } from "@spdf/sdk";

const doc = await Document.create({
  title: "Invoice #INV-2025-001",
  locale: "en-IN",
  documentType: "Invoice",
  author: "ACME Software Solutions",
});

const page = new Page({ size: "A4" });

page.add(new InvoiceHeader({
  invoiceNumber: "INV-2025-001",
  issueDate: "2025-03-15",
  dueDate: "2025-04-14",
  vendor: { name: "ACME Software Solutions Pvt. Ltd.", gstin: "27AABCA1234F1Z5" },
  client: { name: "GlobalTech India Pvt. Ltd.", gstin: "29AACG5678H1ZQ" },
}));

page.add(new LineItemTable({
  currency: "INR",
  taxScheme: "GST_18",
  items: [
    { description: "Backend API Development", qty: 80, unitPrice: "2500.00" },
    { description: "DevOps Setup", qty: 40, unitPrice: "2000.00" },
  ],
}));

doc.addPage(page);

const spdfBuffer = await doc.export.spdf();   // Uint8Array — full SPDF
const pdfBuffer  = await doc.export.pdf();    // Uint8Array — render.pdf only
```

### 17.3 REST API Contract

#### Core Endpoints

| Method | Path | Request Body | Response | Description |
|---|---|---|---|---|
| POST | /v1/convert | multipart: file (PDF) | application/spdf | Convert PDF to SPDF via AI extraction |
| POST | /v1/generate | JSON: {template_id, data} | application/spdf | Generate SPDF from template + data |
| POST | /v1/extract | multipart: file (SPDF) | application/json | Extract structured data from SPDF |
| POST | /v1/sign | multipart: file + cert | application/spdf | Cryptographically sign a document |
| POST | /v1/verify | multipart: file (SPDF) | application/json | Verify signatures and document integrity |
| POST | /v1/validate | multipart: file (SPDF) | application/json | Validate SPDF against spec |
| POST | /v1/render | multipart: file (SPDF) | application/pdf | Render SPDF to PDF |
| POST | /v1/diff | multipart: file_a, file_b | application/json | Semantic diff between two documents |
| GET | /v1/health | none | application/json | API health check |

#### Extraction Response Schema

```json
{
  "document_id": "spdf-d4b7c2a1-...",
  "document_type": "Invoice",
  "document_state": "SIGNED",
  "confidence": 0.94,
  "structured_data": {
    "InvoiceHeader": {
      "invoice_number": "INV-2025-001",
      "issue_date": "2025-03-15T00:00:00.000Z",
      "due_date": "2025-04-14T23:59:59.999Z",
      "vendor": { "name": "...", "gstin": "..." },
      "client": { "name": "...", "gstin": "..." },
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
      "tax_amount": "50400.00",
      "total": "330400.00",
      "currency": "INR"
    }
  },
  "processing_time_ms": 134
}
```

---

## 18. Interoperability and Conversion

### 18.1 SPDF → PDF Conversion

```
SPDF → PDF export algorithm:
1. Validate SPDF container integrity (manifest checksums)
2. Extract render.pdf from the container
3. Optionally regenerate render.pdf from the semantic layer if needed
4. Return or save render.pdf

Data fidelity:
  Visual output: 100% identical (render layer = PDF)
  Semantic data: LOST (PDF has no structured data layer)
  Signature:     PRESERVED as PDF digital signature metadata
  Audit log:     NOT INCLUDED in PDF output
```

### 18.2 PDF → SPDF Conversion (AI-Assisted)

| Conversion Method | Accuracy | Speed | Use Case |
|---|---|---|---|
| Heuristic Analysis | 60–70% field accuracy | Fast (< 1s/page) | Structured, template-like PDFs |
| AI-Assisted (LLM) | 90–95% field accuracy | Slower (2–8s/page) | Diverse, non-standard, or complex PDFs |
| Hybrid (heuristic first, AI for unknowns) | 85–92% field accuracy | Medium (1–4s/page) | High-volume production conversion |

**AI-Assisted Conversion Pipeline:**

```
Step 1: PDF Structure Analysis
  - Extract text stream with positional data
  - Identify font metrics for heading detection
  - Detect table structures via positional clustering
  - Extract embedded images as assets

Step 2: AI Semantic Classification (LLM prompt)
  - Feed page-by-page text with positions to LLM
  - Prompt: classify each text block as: element_type, text, properties
  - LLM returns structured JSON element tree per page
  - Assign confidence score per element

Step 3: SPDF DOM Construction
  - Merge per-page element trees into document DOM
  - Assign element IDs and parent relationships
  - Validate against schema

Step 4: Layout Reconstruction
  - Map PDF coordinates to SPDF layout.json
  - Infer page size and margins from PDF page boxes

Step 5: Container Assembly
  - Write semantic.json, layout.json, styles.json
  - Embed assets
  - Generate render.pdf (use original PDF as render layer)
  - Compute checksums and write manifest
  - Append CREATED and CONVERTED_FROM_PDF entries to audit.json
```

---

## 19. Compliance and Conformance

### 19.1 Conformance Classes

| Class | Identifier | Description | Key Requirements |
|---|---|---|---|
| SPDF Minimal Reader | SPDF-MR-1.0 | Can open any SPDF file and display using render.pdf | Container validation; manifest parse; PDF viewer integration |
| SPDF Reader | SPDF-R-1.0 | Full semantic parsing and screen rendering | Full DOM parse; layout engine; style cascade; screen renderer |
| SPDF Writer | SPDF-W-1.0 | Can produce conformant SPDF documents | All Reader + DOM construction; render.pdf generation |
| SPDF Signer | SPDF-S-1.0 | Can sign and verify SPDF documents | All Writer + X.509 signing; state machine enforcement |
| SPDF Converter | SPDF-C-1.0 | Can convert between SPDF and other formats | All Writer + PDF→SPDF pipeline |
| SPDF Full | SPDF-F-1.0 | All capabilities | All above + encryption; accessibility; streaming; extensions |

### 19.2 Conformance Test Suite (CTS) Categories

| Category | Tests | Description |
|---|---|---|
| Container Validation | 42 tests | Magic bytes, manifest structure, file integrity, path traversal resistance |
| Schema Validation | 187 tests | Element type schemas, required properties, type checking, enum validation |
| DOM Construction | 94 tests | Element ID uniqueness, circular reference detection, null/absent semantics |
| Layout Engine | 68 tests | Page model, coordinate system, pagination rules, multi-column |
| Rendering Fidelity | 156 tests | Visual regression against golden renders at 300 DPI |
| Asset Handling | 51 tests | Font embedding, image decoding, SVG sanitization, compression |
| Security | 88 tests | Signature creation, verification, tamper detection, encryption, erasure |
| Error Handling | 73 tests | Error severity, malformed inputs, resource limits, graceful degradation |
| Versioning | 29 tests | Forward compat, backward compat, profile negotiation |
| Conversion | 44 tests | PDF→SPDF, SPDF→PDF, round-trip fidelity |
| Extensions | 38 tests | Namespace validation, opaque preservation, extension round-trip |
| Accessibility | 32 tests | Alt text, reading order, heading hierarchy, contrast |
| **TOTAL** | **902 tests** | |

---

## 20. Future Roadmap Considerations

*This section is INFORMATIVE. Nothing here is normative for SPDF v1.0.*

### 20.1 SPDF 1.x — Minor Version Roadmap

| Version | Tentative Features | Status |
|---|---|---|
| 1.1 | Right-to-left (RTL) text layout; Arabic and Hebrew bidirectional support | Design phase |
| 1.1 | CJK (Chinese, Japanese, Korean) layout improvements; vertical text support | Design phase |
| 1.2 | Barcode and QR code element types as first-class semantic elements | Proposed |
| 1.2 | Enhanced form elements: multi-page forms, conditional logic, calculated fields | Proposed |
| 1.3 | Digital watermark specification | Proposed |
| 1.4 | Structured annotation model; threaded comments; review workflow | Proposed |
| 1.4 | Chart element type: bar, line, pie charts from structured data | Proposed |

### 20.2 SPDF 2.0 — Major Version Considerations

**Real-Time Collaborative Documents** — Using Operational Transformation (OT) or CRDTs. Operations on the DOM would be expressed as JSON Patch (RFC 6902) operations. The SIGNED state must remain inviolable — collaborative editing only in DRAFT and REVIEW states.

**Streaming Documents** — Documents that can be incrementally received and rendered as data arrives, enabling large reports generated in real time from databases or AI systems.

**AI-Native Document Features:**
- `ai_hint` property on any element — provides a natural language description for AI classification
- `ai_embedding` property — stores a precomputed vector embedding for fast semantic search
- Native document summarization support — `spdf:summary` block with AI-generated document summary embedded in metadata
- Structured provenance — declaration of which AI model processed each element

### 20.3 Standards Track

- SPDF 1.0: Published as an open specification by SPDF Foundation (complete)
- SPDF 1.2+: Submit to OASIS Technical Committee for review and adoption
- SPDF 2.0: Submit to ISO for consideration as an ISO document format standard alongside ISO 32000 (PDF)
- Long-term: W3C consideration for web-native SPDF viewer specification

> **SPDF Vision Statement:** The long-term vision for SPDF is to become the document format that future generations default to — the way HTTP and HTML became the default for information sharing. PDF won in 1993 because it solved the problem of its era perfectly. SPDF is designed to solve the problems of 2025 and beyond: automation, AI, programmability, compliance, and trustworthy digital records. The format is open. The specification is free. The future is shared.

---

## Appendix A — Complete styles.json Schema

```json
{
  "spdf:styles_version": "1.0",
  "defaults": {
    "font_family":    ["Inter", "Helvetica Neue", "Arial", "sans-serif"],
    "font_size":       11.0,
    "font_weight":    400,
    "line_height":    1.4,
    "color":          "#1A1A1A",
    "text_align":     "LEFT",
    "direction":      "LTR",
    "page_background":"#FFFFFF"
  },
  "named_styles": {
    "heading-h1": {
      "font_size": 24.0, "font_weight": 700, "color": "#0D1B4B",
      "spacing_before": 14.0, "spacing_after": 6.0
    },
    "heading-h2": {
      "font_size": 18.0, "font_weight": 700, "color": "#1E3A5F",
      "spacing_before": 10.0, "spacing_after": 4.0
    },
    "body-text": {
      "font_size": 11.0, "font_weight": 400, "color": "#1A1A1A",
      "spacing_before": 0, "spacing_after": 6.0
    },
    "invoice-total": {
      "font_size": 14.0, "font_weight": 700, "color": "#0D1B4B",
      "background_color": "#E8EEF8",
      "border": { "top": { "style": "SOLID", "width": 1.0, "color": "#2244A8" } }
    },
    "table-header-cell": {
      "font_weight": 700, "color": "#FFFFFF",
      "background_color": "#0D1B4B",
      "padding": { "top": 6.0, "right": 8.0, "bottom": 6.0, "left": 8.0 }
    }
  },
  "brand_tokens": {
    "primary":    "#0D1B4B",
    "secondary":  "#E91E63",
    "accent":     "#00695C",
    "text":       "#1A1A1A",
    "background": "#FFFFFF"
  }
}
```

---

## Appendix B — SPDF Error Code Quick Reference

| Code | Name | Severity |
|---|---|---|
| F_INVALID_CONTAINER | Invalid ZIP archive | Fatal |
| F_MANIFEST_MISSING | manifest.json not found | Fatal |
| F_MANIFEST_POSITION | manifest.json not first entry | Fatal |
| F_INVALID_JSON | Invalid JSON in required file | Fatal |
| F_INVALID_VERSION | Unrecognized format version | Fatal |
| F_CHECKSUM_MISMATCH | File integrity failure | Fatal |
| F_PATH_TRAVERSAL | Path traversal attempt detected | Fatal |
| F_CONTAINER_TOO_LARGE | Container exceeds 2 GB limit | Fatal |
| F_MISSING_REQUIRED_FILE | Required file absent | Fatal |
| F_DUPLICATE_EID | Duplicate element ID | Fatal |
| F_CIRCULAR_REFERENCE | Circular element reference | Fatal |
| F_DOM_TOO_LARGE | DOM exceeds 1,000,000 elements | Fatal |
| F_NESTING_TOO_DEEP | Element nesting exceeds 64 levels | Fatal |
| E_MISSING_REQUIRED_PROP | Required property missing | Error |
| E_INVALID_TYPE | Property value type mismatch | Error |
| E_INVALID_ENUM | Unknown enum value | Error |
| E_ASSET_MISSING | Referenced asset not found | Error |
| E_ASSET_UNSAFE | Asset failed security sanitization | Error |
| E_INVALID_DECIMAL | Malformed decimal value | Error |
| E_INVALID_TIMESTAMP | Malformed ISO 8601 timestamp | Error |
| E_INVALID_COLOR | Malformed #RRGGBB color | Error |
| E_UNRESOLVED_EID | EID reference not found in document | Error |
| E_SIGNATURE_INVALID | Digital signature verification failed | Error |
| E_AUDIT_CHAIN_BROKEN | Audit log hash chain inconsistency | Error |
| E_LOCKED_ELEMENT_MODIFIED | Write to locked element attempted | Error |
| E_STATE_TRANSITION_INVALID | Invalid document state transition | Error |
| E_CONTENT_TOO_LARGE | Text content exceeds 4 MB limit | Error |
| E_ASSET_TOO_LARGE | Asset exceeds size limit | Error |
| E_LAYOUT_CONFLICT | Unresolvable layout constraint | Error |
| W_UNKNOWN_EXTENSION | Unknown extension key encountered | Warning |
| W_UNKNOWN_ELEMENT_TYPE | Unknown element type encountered | Warning |
| W_UNREFERENCED_ASSET | Asset declared but not referenced | Warning |
| W_UNEXPECTED_FILE | File outside spec found in container | Warning |
| W_LOW_CONTRAST | Element fails WCAG 4.5:1 contrast ratio | Warning |
| W_DEPRECATED_FEATURE | Document uses a deprecated feature | Warning |

---

## Appendix C — Normative References

| Reference | Document | URL |
|---|---|---|
| ISO 32000-2 | Document management — PDF 2.0 | https://www.iso.org/standard/75839.html |
| RFC 8259 | The JavaScript Object Notation (JSON) Data Interchange Format | https://tools.ietf.org/html/rfc8259 |
| RFC 8785 | JSON Canonicalization Scheme (JCS) | https://tools.ietf.org/html/rfc8785 |
| RFC 3629 | UTF-8, a transformation format of ISO 10646 | https://tools.ietf.org/html/rfc3629 |
| RFC 2119 | Key words for use in RFCs to Indicate Requirement Levels | https://tools.ietf.org/html/rfc2119 |
| RFC 3161 | Internet X.509 PKI Time-Stamp Protocol | https://tools.ietf.org/html/rfc3161 |
| RFC 3986 | Uniform Resource Identifier (URI): Generic Syntax | https://tools.ietf.org/html/rfc3986 |
| PKWARE AppNote | ZIP File Format Specification v6.3.10 | https://pkware.com/appnote |
| Unicode UAX #14 | Unicode Line Breaking Algorithm | https://unicode.org/reports/tr14/ |
| NIST SP 800-38D | Recommendation for Block Cipher Modes: GCM and GMAC | https://doi.org/10.6028/NIST.SP.800-38D |
| NIST FIPS 180-4 | Secure Hash Standard (SHA-256) | https://doi.org/10.6028/NIST.FIPS.180-4 |
| RFC 8017 | PKCS #1: RSA Cryptography Specifications (RSA-PSS) | https://tools.ietf.org/html/rfc8017 |
| BCP 47 | Tags for Identifying Languages | https://tools.ietf.org/html/bcp47 |
| ISO 4217 | Currency codes | https://www.iso.org/iso-4217-currency-codes.html |
| WCAG 2.1 | Web Content Accessibility Guidelines 2.1 | https://www.w3.org/TR/WCAG21/ |

---

## Appendix D — Glossary of Acronyms

| Acronym | Expansion |
|---|---|
| AES | Advanced Encryption Standard — symmetric encryption algorithm (AES-256-GCM) |
| CRDT | Conflict-Free Replicated Data Type — for distributed collaborative editing |
| DOM | Document Object Model — in-memory tree representation of document structure |
| EID | Element Identifier — unique ID of an SPDF DOM element |
| GCM | Galois/Counter Mode — authenticated encryption mode for AES |
| JCS | JSON Canonicalization Scheme — RFC 8785 deterministic JSON serialization |
| OT | Operational Transform — algorithm for collaborative real-time editing |
| PEM | Privacy Enhanced Mail — Base64-encoded certificate format |
| PII | Personally Identifiable Information — data subject to privacy regulations |
| PKI | Public Key Infrastructure — framework for X.509 certificates |
| PSS | Probabilistic Signature Scheme — RSA signature padding |
| SPDF | Structured Portable Document Format — this specification |
| TLS | Transport Layer Security — encrypted transport protocol |
| UAX | Unicode Standard Annex — supplementary Unicode specifications |
| UTC | Coordinated Universal Time — required timezone for all SPDF timestamps |
| WASM | WebAssembly — portable binary instruction format for browser execution |
| WCAG | Web Content Accessibility Guidelines — W3C accessibility standard |
| WOFF2 | Web Open Font Format 2 — compressed font format for embedding |
| X.509 | ITU-T standard for public key certificates used for digital signatures |

---

*— End of SPDF Technical Specification v1.0 —*

Copyright © 2025 SPDF Foundation. This specification is published under the Creative Commons Attribution 4.0 International License.
