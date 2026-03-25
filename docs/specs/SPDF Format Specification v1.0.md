# SPDF — Structured Portable Document Format
## Specification Version 1.0

**Status:** Draft for Public Review
**Published:** March 2025
**License:** [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/)
**Repository:** [github.com/spdf-foundation/spec](https://github.com/spdf-foundation/spec)
**Discussion:** [github.com/spdf-foundation/spec/discussions](https://github.com/spdf-foundation/spec/discussions)

---

> *The document standard for the next 30 years.*
>
> SPDF keeps every promise PDF made in 1993 — visual fidelity, print quality, offline reliability, digital signatures, universal compatibility — and adds what 1993 could not imagine: a fully structured semantic layer that makes every element of every document programmable, queryable, and trustworthy.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Conformance Language](#2-conformance-language)
3. [Design Principles](#3-design-principles)
4. [File Container Architecture](#4-file-container-architecture)
5. [Document Object Model](#5-document-object-model)
6. [Element Type Registry](#6-element-type-registry)
7. [Security Model](#7-security-model)
8. [Error Codes](#8-error-codes)
9. [Versioning and Compatibility](#9-versioning-and-compatibility)
10. [Conformance Classes](#10-conformance-classes)
11. [Normative References](#11-normative-references)
12. [Change Log](#12-change-log)

---

## 1. Introduction

### 1.1 What Is SPDF?

SPDF (Structured Portable Document Format) is an open document format designed to replace PDF as the world's primary fixed-layout document standard.

PDF solved the problem of its era perfectly: how do you ensure a document looks identical on every computer? Adobe's answer — a stream of drawing instructions that paint characters at exact pixel coordinates — produced perfect visual fidelity. The world adopted it universally.

That same design decision has become the single greatest limitation in modern document processing. A PDF is a photograph of a document. It captures exactly what something looks like, but contains no information about what anything *means*. When a developer, AI system, or enterprise application opens a PDF, there is no concept of "this is a heading", "this is a table", "this is a payment amount." There are only paint strokes on a virtual canvas.

SPDF solves this by maintaining everything the world relies on about PDF while adding a complete semantic layer that makes every element of every document programmable, queryable, and trustworthy.

### 1.2 Core Properties

An SPDF document guarantees all of the following simultaneously:

| Property | Guarantee |
|---|---|
| **Visual fidelity** | Renders identically on every device, OS, and viewer — forever |
| **Print fidelity** | What you see is what prints, at any resolution |
| **Offline reliability** | Works with no network connection, no external dependencies |
| **Backward compatibility** | Opens as a standard PDF in every existing PDF viewer |
| **Semantic structure** | Every element is typed, named, and queryable |
| **Cryptographic integrity** | Digital signatures, element-level locking, true redaction |
| **AI readability** | Machine-readable from creation — no OCR, no extraction pipeline |
| **Accessibility** | Screen reader support built into the format by default |

### 1.3 Relationship to PDF

SPDF does not compete with PDF viewers. It extends the container. An SPDF file **is** a valid ZIP archive containing, among other things, a standard PDF file. Every PDF viewer in the world opens the PDF layer identically. SPDF-aware systems additionally read the semantic layer and unlock full programmatic control.

```
document.spdf  (ZIP container)
├── render.pdf         ← standard PDF — opens in any PDF viewer
├── semantic.json      ← structured element tree — readable by any JSON parser
├── layout.json        ← positions and dimensions
├── styles.json        ← colours, fonts, visual properties
├── metadata.json      ← document metadata
├── audit.json         ← cryptographic audit trail
└── assets/            ← embedded fonts, images, attachments
```

This architecture means adoption is free. There is no switching cost. Senders can generate SPDF documents today. Recipients who are not yet SPDF-aware open the PDF layer exactly as before. Recipients who are SPDF-aware get full structured data with no additional processing.

### 1.4 Design Goals

**This specification defines:**
- The container format and file structure
- The Document Object Model (DOM) and all element types
- The security model: signing, verification, and cryptographic erasure
- The error code registry
- Versioning and forward-compatibility rules
- Conformance classes for implementors

**This specification does not define** (reserved for v1.1):
- The layout engine and rendering pipeline
- Multi-column and advanced typography rules
- The template variable binding system
- Real-time collaborative editing extensions

### 1.5 Who Should Read This

- **Developers** implementing SPDF parsers, writers, or renderers
- **SDK authors** building tools that generate or consume SPDF documents
- **Standards reviewers** evaluating the format for adoption or standardisation
- **Security researchers** auditing the cryptographic model

If you are a developer who wants to *use* SPDF rather than implement it, the [SDK documentation](https://docs.spdf.dev) is a faster starting point.

### 1.6 Relationship to Existing Standards

| Standard | Relationship |
|---|---|
| ISO 32000-2 (PDF 2.0) | The SPDF render layer MUST be a conformant PDF 2.0 document |
| RFC 8785 (JCS) | SPDF canonical form for signing uses JSON Canonicalization Scheme |
| RFC 3161 | SPDF CERTIFIED state uses RFC 3161 trusted timestamp tokens |
| PKWARE ZIP | The SPDF container is a ZIP archive (PKWARE AppNote v6.3.10+) |
| RFC 2119 | Conformance language (MUST, SHALL, SHOULD, MAY) follows RFC 2119 |
| Unicode 15.0 / UTF-8 | All text in SPDF MUST be encoded as UTF-8 (RFC 3629) |
| BCP 47 | Language tags use BCP 47 format |
| ISO 4217 | Currency codes use the ISO 4217 three-letter standard |

---

## 2. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** in this specification are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

A **conformant implementation** is one that satisfies all MUST and MUST NOT requirements applicable to its declared conformance class (see Section 10).

An implementation that satisfies all MUST requirements but fails a SHOULD requirement is conformant but not recommended. Deviation from SHOULD requirements SHOULD be documented and justified.

---

## 3. Design Principles

These principles are the rationale behind every decision in the specification. When the spec is ambiguous or silent on an edge case, implementations SHOULD resolve the ambiguity in the manner most consistent with these principles.

### 3.1 Deterministic Rendering

Given the same SPDF document, any two conformant rendering engines MUST produce pixel-identical output at the same target resolution. This is the most foundational guarantee of the format.

- All fonts MUST be embedded. The same document must render identically on a system that has never seen those fonts before
- All dimensional values MUST be expressed in points (1/72 inch) — a device-independent unit
- Colour values MUST be expressed in sRGB within the document

### 3.2 Separation of Concerns

SPDF strictly separates content, layout, and presentation into distinct files within the container. This enables:

- Content extraction without parsing layout
- Re-layout for different page sizes without changing content
- Theme changes without touching content or layout

```
semantic.json  →  WHAT the document says   (text, data, structure, types)
layout.json    →  WHERE elements appear     (positions, dimensions, pages)
styles.json    →  HOW elements look         (colours, fonts, spacing)
render.pdf     →  FUSED output              (for backward compatibility)
```

### 3.3 Machine-Readability First

Every structural decision is evaluated first by whether it is unambiguously parseable by a machine:

- Every element MUST declare its type explicitly — type MUST NOT be inferred from context or position
- Numeric values MUST NOT be stored as formatted strings (e.g. `"1,234.56"` is invalid; `"1234.56"` is correct)
- Dates and times MUST use ISO 8601 UTC — human-readable date strings are prohibited
- All financial values MUST be stored as decimal strings, never as floating-point numbers

### 3.4 Forward Compatibility

Documents written for a newer minor version can be safely opened by an older minor version reader without data loss of known elements.

| Rule | Statement |
|---|---|
| **FC-1** | A conformant parser encountering an unknown element type MUST preserve it as an opaque node, passing it through to any writer unchanged |
| **FC-2** | A conformant parser encountering an unknown property key on a known element MUST preserve that key-value pair unchanged |
| **FC-3** | A conformant writer MUST NOT discard preserved opaque elements or unknown keys |
| **FC-4** | A conformant reader MUST NOT fail for any document whose major version number matches the reader's supported major version |

### 3.5 Fail-Safe Degradation

When a conformant SPDF viewer cannot fully render an SPDF document, it MUST fall back to the render layer (`render.pdf`) rather than showing a corrupt or partially rendered document. The render layer is always present precisely to guarantee this fallback path.

An SPDF document with a corrupt semantic layer but intact `render.pdf` MUST still be openable by any PDF viewer in the world.

### 3.6 Security by Design

Security is not a feature added to SPDF — it is a constraint applied at the format level:

- The specification **prohibits** executable content. No JavaScript, no macros, no auto-run actions. This is not configurable
- Parsers MUST enforce size limits before allocating memory
- Cryptographic operations use only NIST-approved algorithms
- The audit log is append-only and hash-chained — retroactive modification is detectable

### 3.7 No Reinvention

Where an existing, well-specified standard solves a problem correctly, SPDF adopts that standard:

- ZIP for the container (not a custom archive format)
- JSON for structured data (not a custom DSL)
- PDF for the render layer (not a custom page description language)
- X.509 / PKI for digital signatures (not a custom certificate format)
- RFC 3161 for trusted timestamps (not a custom timestamping protocol)

---

## 4. File Container Architecture

### 4.1 Container Format

An SPDF file is a ZIP archive conforming to the PKWARE Application Note, version 6.3.10 or later. The file extension SHALL be `.spdf`.

SPDF parsers MUST NOT rely on the file extension for format detection. They MUST use the magic bytes defined in Section 4.2.

**ZIP compliance requirements:**
- The archive MUST use ZIP format version 2.0 or later
- File entries MUST use the Deflate compression method (ID 8) or Store (ID 0)
- The archive MUST NOT be password-protected at the ZIP level
- File entries MUST use UTF-8 encoded filenames (General Purpose Bit Flag bit 11 MUST be set)
- All timestamps in ZIP local file headers MUST be set to `1980-01-01 00:00:00` for canonical reproducibility

### 4.2 Format Detection

Parsers MUST detect SPDF files using the following algorithm:

```javascript
function isSpdf(bytes) {
  // Step 1: Verify ZIP signature
  if (bytes[0..4] !== [0x50, 0x4B, 0x03, 0x04]) return false;

  // Step 2: Verify manifest.json is the first ZIP entry
  const firstEntryName = readZipEntryName(bytes, offset=30);
  if (firstEntryName !== "manifest.json") return false;

  // Step 3: Verify SPDF format declaration in manifest
  const manifest = parseJson(readZipEntry(bytes, "manifest.json"));
  return manifest["spdf:format"] === "SPDF"
      && manifest["spdf:version"] !== undefined;
}
```

`manifest.json` MUST be the first entry in the ZIP archive. Implementations MUST reject containers where this is not the case.

### 4.3 Required Directory Structure

Every conformant SPDF file MUST contain exactly this structure:

```
document.spdf                     ← ZIP container (.spdf extension)
├── manifest.json                 ← REQUIRED: container manifest and checksums
├── semantic.json                 ← REQUIRED: document object model
├── layout.json                   ← REQUIRED: element positions and dimensions
├── styles.json                   ← REQUIRED: style definitions
├── render.pdf                    ← REQUIRED: PDF 2.0 render layer
├── metadata.json                 ← REQUIRED: document metadata
├── audit.json                    ← REQUIRED: immutable audit log
├── assets/                       ← REQUIRED directory (MAY be empty)
│   ├── fonts/                    ← WOFF2, TTF, or OTF font files
│   ├── images/                   ← PNG, JPEG, JPEG2000, or WebP images
│   ├── vectors/                  ← Sanitised SVG files
│   └── attachments/              ← Binary file attachments
├── signatures/                   ← OPTIONAL: cryptographic signatures
│   ├── signature_001.json
│   └── certificate_001.pem
├── extensions/                   ← OPTIONAL: vendor extension data
│   └── x-{vendor-name}/
└── _rels/                        ← REQUIRED: relationship index
    └── relationships.json
```

Any files NOT listed in this specification MUST be placed inside the `extensions/` directory. Files at the container root that are not in the required list MUST trigger a `W_UNEXPECTED_FILE` warning.

### 4.4 manifest.json

The manifest is the authoritative index of the container. It declares the format version, lists all required files with their SHA-256 checksums, and declares any extension namespaces in use.

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
      "sha256": "e3b0c44298fc1c149afb4c8996fb92427ae41e4649b934ca495991b7852b855",
      "size_bytes": 48291,
      "encoding": "utf-8"
    },
    "layout": {
      "file": "layout.json",
      "sha256": "d7a8fbb307d7809469d3b5f7a3694c12f76a3e5e2b3d2a1f4e8c9b0a7d6f5e4",
      "size_bytes": 12044,
      "encoding": "utf-8"
    },
    "styles": {
      "file": "styles.json",
      "sha256": "b1a9bc2e3d4f5678abcdef0123456789abcdef0123456789abcdef0123456789",
      "size_bytes": 3820,
      "encoding": "utf-8"
    },
    "render": {
      "file": "render.pdf",
      "sha256": "5f4d3e2c1b0a98765432fedcba9876543210fedcba9876543210fedcba987654",
      "size_bytes": 152480
    }
  },
  "assets": [
    {
      "id": "3d4f9a1b2c7e8f0a",
      "type": "font",
      "mime": "font/woff2",
      "sha256": "a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890"
    },
    {
      "id": "7b2a8c3d4e9f1a0b",
      "type": "image",
      "mime": "image/png",
      "sha256": "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210"
    }
  ],
  "extensions": [],
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "manifest_hash": "sha256:a1b2c3d4e5f67890123456789012345678901234567890123456789012345678"
}
```

### 4.5 audit.json

The audit log is an append-only record of every state-changing operation performed on the document. It is hash-chained: each entry contains the SHA-256 of the previous entry, making retroactive modification detectable.

```json
{
  "spdf:audit_version": "1.0",
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
  "entries": [
    {
      "seq": 1,
      "action": "CREATED",
      "at": "2025-03-15T09:30:00.000Z",
      "by": {
        "type": "SOFTWARE",
        "name": "ACME Billing v3.2",
        "id": null
      },
      "state_before": null,
      "state_after": "DRAFT",
      "affected_elements": [],
      "prev_hash": "0000000000000000000000000000000000000000000000000000000000000000",
      "entry_hash": "sha256:7f3a2c1b4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1"
    },
    {
      "seq": 2,
      "action": "STATE_CHANGED",
      "at": "2025-03-15T14:22:10.441Z",
      "by": {
        "type": "USER",
        "name": "Arjun Sharma",
        "id": "user-8a3b2c1d"
      },
      "state_before": "DRAFT",
      "state_after": "SIGNED",
      "affected_elements": [],
      "change_summary": "Document signed by Authorized Signatory",
      "prev_hash": "sha256:7f3a2c1b4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1",
      "entry_hash": "sha256:9d4e3f2a1b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1"
    }
  ],
  "log_hash": "sha256:1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2"
}
```

**Audit chain verification:** Each `entry_hash` is `SHA-256(canonical_json(entry_without_entry_hash))`. Each `prev_hash` is the `entry_hash` of the immediately preceding entry. The first entry's `prev_hash` is the zero hash. Any modification to any entry is detectable by recomputing the chain.

### 4.6 Container Validation Rules

| Rule ID | Rule | Error Code | Severity |
|---|---|---|---|
| CONT-001 | Container MUST be a valid ZIP archive | `E_INVALID_CONTAINER` | Fatal |
| CONT-002 | `manifest.json` MUST be the first ZIP entry | `E_MANIFEST_POSITION` | Fatal |
| CONT-003 | All required files MUST be present | `E_MISSING_REQUIRED_FILE` | Fatal |
| CONT-004 | SHA-256 of each file MUST match its manifest entry | `E_CHECKSUM_MISMATCH` | Fatal |
| CONT-005 | `spdf:version` in manifest MUST be a valid version string | `E_INVALID_VERSION` | Fatal |
| CONT-006 | `manifest.json` MUST be valid JSON per RFC 8259 | `E_INVALID_JSON` | Fatal |
| CONT-007 | Assets referenced in `semantic.json` MUST exist in `assets/` | `E_MISSING_ASSET` | Error |
| CONT-008 | Assets declared in manifest SHOULD be referenced in `semantic.json` | `W_UNREFERENCED_ASSET` | Warning |
| CONT-009 | Files not listed in this spec MUST be in `extensions/` | `W_UNEXPECTED_FILE` | Warning |
| CONT-010 | The `audit.json` hash chain MUST be internally consistent | `E_AUDIT_CHAIN_BROKEN` | Error |
| CONT-011 | Total uncompressed container size MUST NOT exceed 2 GB | `E_CONTAINER_TOO_LARGE` | Fatal |
| CONT-012 | No file path may contain `..` or absolute path components | `E_PATH_TRAVERSAL` | Fatal |

---

## 5. Document Object Model

### 5.1 Overview

The SPDF DOM is a tree structure stored in `semantic.json`. It is the authoritative representation of a document's content. The render layer (`render.pdf`) is derived from it — not the other way around.

```
Document
└── pages[]
    └── Page
        └── elements[]
            ├── Section
            │   └── elements[]
            │       ├── Heading
            │       ├── Paragraph
            │       ├── Table
            │       │   └── rows[]
            │       │       └── TableRow
            │       │           └── cells[]
            │       │               └── TableCell
            │       ├── Image
            │       ├── InvoiceHeader       ← domain element
            │       ├── LineItemTable       ← domain element
            │       ├── SignatureBlock      ← trust element
            │       └── FormField
            └── PageFooter / PageHeader
```

### 5.2 Data Types

All property values in SPDF use the following type system:

| SPDF Type | JSON Representation | Constraints |
|---|---|---|
| `spdf:string` | JSON string | UTF-8, max 65,535 bytes |
| `spdf:text_block` | JSON string | UTF-8, max 4,194,304 bytes |
| `spdf:integer` | JSON number (integer) | −2⁵³ to 2⁵³−1, no decimal point |
| `spdf:decimal` | JSON **string** | Decimal notation: `"1234.56"`. **Never a JSON number** |
| `spdf:dimension` | JSON number | IEEE 754 double, ≥ 0.0, max 14400.0 (points) |
| `spdf:color` | JSON string | `#RRGGBB` hex, case-insensitive, always 7 chars |
| `spdf:timestamp` | JSON string | ISO 8601 UTC: `YYYY-MM-DDTHH:MM:SS.sssZ`. Timezone MUST be `Z` |
| `spdf:boolean` | JSON boolean | `true` or `false` literals. Strings `"true"`/`"false"` are NOT valid |
| `spdf:eid` | JSON string | Element ID format (Section 5.3) |
| `spdf:locale` | JSON string | BCP 47 language tag: `"en-IN"`, `"de-DE"` |
| `spdf:currency` | JSON string | ISO 4217 three-letter code: `"USD"`, `"EUR"`, `"INR"` |

> **Why decimal values are strings:** IEEE 754 floating-point cannot represent most decimal fractions exactly. `0.1 + 0.2 ≠ 0.3` in binary floating point. All financial values, tax rates, and quantities MUST be stored as decimal strings and parsed using arbitrary-precision decimal arithmetic.

### 5.3 Element Identifiers

Every element in the DOM has a globally unique identifier within its document, following this format:

```
eid = prefix "-" timestamp "-" sequence "-" checksum

prefix    = 2–4 lowercase alpha chars (scope hint, e.g. "el", "sec", "tbl")
timestamp = 13 digits (Unix epoch milliseconds at document creation time)
sequence  = 4–8 digits (monotonically increasing counter per document)
checksum  = 4 hex digits (first 4 chars of SHA-256(prefix + timestamp + sequence))

Example: "el-1709251200000-00142-a3f7"
```

**Rules:**
- Element IDs MUST be unique within a document
- Element IDs MUST NOT be reassigned after generation
- Parsers MUST reject documents with duplicate element IDs

### 5.4 Null, Absent, and Empty Value Semantics

SPDF distinguishes three distinct states for any property:

| State | JSON | Meaning |
|---|---|---|
| Value present | `"color": "#FF0000"` | Property has this explicit value |
| Null (explicit) | `"color": null` | Property is explicitly unset — overrides any inherited value |
| Key absent | *(key not in object)* | Property uses its type default or inherits from parent or style |
| Empty string | `"text": ""` | Property is an empty string — distinct from null |
| Empty array | `"elements": []` | Container has no children — distinct from null |

### 5.5 Document Root

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

### 5.6 Universal Element Properties

Every element — regardless of type — MUST support these properties:

```json
{
  "eid": "el-1709251200000-00014-c2f9",
  "element_type": "Heading",
  "version": 1,
  "created_at": "2025-03-15T09:30:00.000Z",
  "modified_at": "2025-03-15T09:35:14.221Z",
  "style_id": "heading-primary",
  "integrity_hash": "sha256:a1b2c3...",
  "locked": false,
  "visible": true,
  "accessible_label": null,
  "custom_data": null,
  "x_extensions": {}
}
```

| Property | Type | Required | Description |
|---|---|---|---|
| `eid` | `spdf:eid` | REQUIRED | Globally unique element identifier |
| `element_type` | string | REQUIRED | Type string from the Element Type Registry (Section 6) |
| `version` | `spdf:integer` | REQUIRED | Starts at 1. Incremented on each modification |
| `created_at` | `spdf:timestamp` | REQUIRED | Element creation time. Immutable after creation |
| `modified_at` | `spdf:timestamp` | REQUIRED | Last modification time |
| `style_id` | string \| null | OPTIONAL | Reference to a named style in `styles.json` |
| `integrity_hash` | string \| null | OPTIONAL | SHA-256 of the element's canonical form. Set at signing |
| `locked` | `spdf:boolean` | REQUIRED | If `true`, element cannot be edited or deleted |
| `visible` | `spdf:boolean` | REQUIRED | If `false`, element is in DOM but not rendered |
| `accessible_label` | string \| null | OPTIONAL | Accessibility label for screen readers |
| `custom_data` | object \| null | OPTIONAL | Arbitrary application-level metadata |
| `x_extensions` | object | REQUIRED | Vendor extension properties (empty object if none) |

### 5.7 Inline Text Formatting

Paragraph and TableCell elements MAY contain rich inline formatting via an `inline_spans` array. A paragraph uses either `text` (plain) or `inline_spans` (formatted), never both.

```json
{
  "element_type": "Paragraph",
  "eid": "el-...",
  "text": null,
  "inline_spans": [
    { "text": "Payment is due ", "style": null },
    {
      "text": "within 30 days",
      "style": { "bold": true, "color": "#B71C1C" }
    },
    { "text": " from invoice date.", "style": null }
  ]
}
```

Supported inline style properties: `bold`, `italic`, `underline` (`none` | `single` | `double` | `dotted`), `strikethrough`, `superscript`, `subscript`, `font_family`, `font_size`, `color`, `background`.

---

## 6. Element Type Registry

### 6.1 Registry Rules

- All type strings are normative. Implementations MUST recognise the types listed in this section
- Implementations encountering unlisted types MUST follow the Forward Compatibility Protocol (Section 3.4) — preserve as opaque nodes, do not error
- Custom types defined by extensions MUST be prefixed with a vendor namespace: `"x-acme:PriorityBadge"`

### 6.2 Structural Elements

| Type | Leaf? | Description |
|---|---|---|
| `Document` | No | Root node. Exactly one per document |
| `Page` | No | Represents one physical page |
| `PageHeader` | No | Repeating header region, rendered on specified pages |
| `PageFooter` | No | Repeating footer region, rendered on specified pages |
| `Section` | No | Logical division of content (chapter, clause, group) |
| `Group` | No | Non-semantic container for layout grouping |
| `Column` | No | Vertical column in a multi-column layout |

### 6.3 Content Elements

| Type | Leaf? | Description |
|---|---|---|
| `Heading` | Yes | Heading text at levels 1–6. REQUIRED property: `level` (integer 1–6), `text` |
| `Paragraph` | Yes | Block of body text. REQUIRED: `text` or `inline_spans` (not both) |
| `Table` | No | Structured data table. REQUIRED: `col_definitions[]`, `rows[]` |
| `TableRow` | No | Row within a Table. REQUIRED: `row_type` (`HEADER` \| `DATA` \| `FOOTER` \| `SUBTOTAL`), `cells[]` |
| `TableCell` | No | Cell within a TableRow. REQUIRED: `col_id`. MAY contain nested elements |
| `Image` | Yes | Raster image asset reference. REQUIRED: `asset_id`, `alt_text` |
| `VectorImage` | Yes | SVG vector image asset reference. REQUIRED: `asset_id`, `alt_text` |
| `CodeBlock` | Yes | Monospaced preformatted text. REQUIRED: `text`, `language` |
| `HorizontalRule` | Yes | Horizontal divider line |
| `PageBreak` | Yes | Explicit page break directive |
| `Attachment` | Yes | Binary file attachment declaration |

**Heading element example:**
```json
{
  "element_type": "Heading",
  "eid": "el-1709251200000-00020-b3a1",
  "level": 1,
  "text": "Invoice #INV-2025-001",
  "numbering": null,
  "anchor_id": "section-invoice-header",
  "version": 1,
  "created_at": "2025-03-15T09:30:00.000Z",
  "modified_at": "2025-03-15T09:30:00.000Z",
  "locked": false,
  "visible": true,
  "style_id": "heading-h1",
  "integrity_hash": null,
  "accessible_label": null,
  "custom_data": null,
  "x_extensions": {}
}
```

**Table element example:**
```json
{
  "element_type": "Table",
  "eid": "el-1709251200000-00031-d4e5",
  "caption": null,
  "header_rows": 1,
  "footer_rows": 0,
  "col_definitions": [
    { "id": "col-desc",  "header": "Description", "width_pct": 45.0, "data_type": "text" },
    { "id": "col-qty",   "header": "Qty",          "width_pct": 10.0, "data_type": "decimal" },
    { "id": "col-price", "header": "Unit Price",   "width_pct": 22.5, "data_type": "decimal" },
    { "id": "col-total", "header": "Total",        "width_pct": 22.5, "data_type": "decimal" }
  ],
  "rows": [
    {
      "element_type": "TableRow",
      "eid": "el-1709251200000-00032-f1a2",
      "row_type": "DATA",
      "cells": [
        {
          "element_type": "TableCell",
          "col_id": "col-desc",
          "content": [{ "element_type": "Paragraph", "eid": "...", "text": "Backend API Development" }]
        },
        { "element_type": "TableCell", "col_id": "col-qty",   "value": "80",       "data_type": "decimal" },
        { "element_type": "TableCell", "col_id": "col-price", "value": "2500.00",  "currency": "INR" },
        { "element_type": "TableCell", "col_id": "col-total", "value": "200000.00","currency": "INR",
          "formula": "qty * price" }
      ]
    }
  ]
}
```

### 6.4 Domain Elements

Domain elements carry structured, typed data for specific business document categories. They are the primary source of machine-readable value in SPDF — an `InvoiceHeader` is not just formatted text, it is a structured data object with typed fields.

| Type | Leaf? | Description |
|---|---|---|
| `InvoiceHeader` | No | Structured invoice header with vendor, client, dates, and reference numbers |
| `LineItem` | Yes | Single line item: description, quantity, unit price, total, currency |
| `LineItemTable` | No | Table of `LineItem` elements with computed subtotal, tax, and total rows |
| `PaymentTerms` | Yes | Payment terms, due date, and bank/payment details |

**InvoiceHeader element example:**
```json
{
  "element_type": "InvoiceHeader",
  "eid": "el-1709251200000-00010-a1b2",
  "invoice_number": "INV-2025-001",
  "issue_date": "2025-03-15T00:00:00.000Z",
  "due_date": "2025-04-14T23:59:59.999Z",
  "currency": "INR",
  "vendor": {
    "name": "ACME Software Solutions Pvt. Ltd.",
    "address": "B-401, Tech Park, Pune 411057",
    "gstin": "27AABCA1234F1Z5",
    "email": "billing@acme.com",
    "phone": null
  },
  "client": {
    "name": "GlobalTech India Pvt. Ltd.",
    "address": "C-202, IT Hub, Bengaluru 560103",
    "gstin": "29AACG5678H1ZQ"
  },
  "po_reference": null,
  "tax_scheme": "GST_18"
}
```

**LineItem element example:**
```json
{
  "element_type": "LineItem",
  "eid": "el-1709251200000-00051-c3d4",
  "description": "Backend API Development",
  "quantity": "80",
  "unit": "hours",
  "unit_price": "2500.00",
  "total": "200000.00",
  "currency": "INR",
  "hsn_sac_code": "998314",
  "tax_rate": "18.00",
  "tax_amount": "36000.00",
  "discount_rate": null,
  "discount_amount": null
}
```

### 6.5 Trust Elements

Trust elements manage document integrity, signing, and audit functionality.

| Type | Leaf? | Description |
|---|---|---|
| `SignatureBlock` | Yes | Signature placeholder or completed digital signature |
| `Stamp` | Yes | Official stamp or seal |
| `Annotation` | Yes | Comment or annotation on a document region |
| `Redaction` | Yes | Record of cryptographic erasure of removed content |

**SignatureBlock element example:**
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
  "certificate_ref": null,
  "lock_on_sign": true,
  "elements_locked": []
}
```

After signing, `signature_state` becomes `"SIGNED"`, and `signed_at`, `signed_by`, `signature_hash`, and `certificate_ref` are populated.

### 6.6 Interactive and Template Elements

| Type | Leaf? | Description |
|---|---|---|
| `FormField` | Yes | Data input field (in DRAFT state only). Types: `TEXT`, `NUMBER`, `DATE`, `CHECKBOX`, `SELECT` |
| `VariablePlaceholder` | Yes | Binding point in a Template document. REQUIRED: `variable_name` |

---

## 7. Security Model

### 7.1 Architecture Overview

The SPDF security model operates at four independent levels. Together they form a defence-in-depth architecture where tampering with any part of a SIGNED or CERTIFIED document is cryptographically detectable.

| Level | Mechanism | What It Protects | When Applied |
|---|---|---|---|
| L1 — Format | No executable content, by specification | The system from document-based exploits | Always — by definition |
| L2 — Container | SHA-256 checksums in manifest | Individual files from modification after writing | All documents at write time |
| L3 — Element | Per-element `integrity_hash` | Individual elements from modification after hashing | Selectively applied; required on trust elements |
| L4 — Document | X.509 digital signature | Entire document from modification after signing | SIGNED and CERTIFIED states only |

### 7.2 Prohibited Content

The following MUST NOT be present in any conformant SPDF document. Parsers MUST reject files containing any of these:

- JavaScript or any scripting language
- Executable attachments (`.exe`, `.sh`, `.bat`, `.ps1`, and similar)
- Auto-run or auto-open actions
- External URL resolution triggers in sealed documents
- Form submission actions (forms are data containers only; submission is handled by the application)

This is not a configurable security policy. It is a fundamental property of the format.

### 7.3 Document State Machine

Every SPDF document has a `document_state` property. State transitions are one-directional — documents cannot revert from SIGNED to DRAFT.

```
                    submit_for_review
  ┌─────────┐ ─────────────────────────► ┌─────────┐
  │  DRAFT  │                            │ REVIEW  │
  │         │ ◄─────────────────────────  │         │
  └─────────┘       reject               └────┬────┘
                                              │ sign(X.509)
                                              ▼
                                         ┌─────────┐
                                         │ SIGNED  │  ← immutable
                                         └────┬────┘
                                              │ certify(RFC 3161)
                                              ▼
                                        ┌──────────┐
                                        │CERTIFIED │  ← immutable, highest trust
                                        └──────────┘
```

| Transition | Trigger | Effect |
|---|---|---|
| `DRAFT` → `REVIEW` | User submits for review | Document becomes read-only. Comments and annotations only |
| `REVIEW` → `DRAFT` | Reviewer rejects | Document returns to editable state |
| `REVIEW` → `SIGNED` | Authorised signer applies X.509 signature | Document becomes permanently immutable. All elements lock |
| `SIGNED` → `CERTIFIED` | Document owner adds RFC 3161 timestamp | Adds trusted timestamp proof. Immutability maintained |

**Forbidden transitions:** `SIGNED` → `DRAFT`, `SIGNED` → `REVIEW`, `CERTIFIED` → any state. These are prohibited without exception.

### 7.4 Document Signing Algorithm

Before signing, a canonical form of the document is computed using the following deterministic algorithm:

```javascript
// Step 1: Compute RFC 8785 JCS canonical form of each semantic layer
const canonical_semantic  = jcs(semantic_json);
const canonical_layout    = jcs(layout_json);
const canonical_styles    = jcs(styles_json);
const canonical_metadata  = jcs(metadata_json);

// Step 2: Compute SHA-256 of each canonical form
const h_semantic  = sha256(canonical_semantic);
const h_layout    = sha256(canonical_layout);
const h_styles    = sha256(canonical_styles);
const h_metadata  = sha256(canonical_metadata);

// Step 3: Concatenate hashes in deterministic order and compute root hash
const document_hash = sha256(h_semantic + h_layout + h_styles + h_metadata);

// Step 4: Sign the document hash
const signature = rsa_pss_sign(document_hash, private_key, {
  hash: "SHA-256",
  salt_length: 32,
  padding: "PSS"
});
```

The render layer (`render.pdf`) is intentionally excluded from the hash. It is a derived artefact. The semantic layers are the authoritative content.

### 7.5 Signature Schema

Signatures are stored in `signatures/signature_NNN.json`:

```json
{
  "spdf:sig_version": "1.0",
  "signature_id": "sig-a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "document_id": "spdf-d4b7c2a1-9e3f-4b2d-8a1c-7f6e5d4c3b2a",
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
    "serial_number": "0x1234567890ABCDEF",
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

`timestamp_token` is `null` for SIGNED state and populated with an RFC 3161 token for CERTIFIED state.

### 7.6 Cryptographic Erasure (True Redaction)

PDF "redaction" by painting black rectangles over text is reversible — the content remains in the file. SPDF defines cryptographic erasure: content is removed from both the semantic and render layers, and the erasure is permanently recorded.

**Algorithm:**

```
1. Identify target elements by eid
2. Compute proof_hash = SHA-256(canonical_form_of_target_elements)
3. Remove elements from semantic.json
4. Regenerate render.pdf without those elements
5. Write a Redaction element at the erased position:

   {
     "element_type": "Redaction",
     "eid": "el-...-00199-e3f4",
     "erased_eids": ["el-...-00041-b3c2"],
     "reason": "GDPR Article 17 — Right to Erasure",
     "erased_at": "2025-06-01T08:00:00.000Z",
     "erased_by": { "type": "USER", "name": "Data Protection Officer" },
     "proof_hash": "sha256:content_was_here_and_is_now_gone...",
     "replacement_text": "[PII ERASED]"
   }

6. Append ELEMENT_ERASED entry to audit.json
```

The `proof_hash` proves the content existed at a specific point in time without revealing what it was. The content is gone. This is mathematically provable.

### 7.7 Asset Security

**SVG sanitisation:** SVG files are a significant attack surface. Before embedding, SPDF writers MUST sanitise all SVG assets by removing:

- All `<script>` elements and `on*` event attributes
- All `<foreignObject>` elements
- All external resource references (`href`, `xlink:href`, `src` pointing to URLs)
- All `<animate>`, `<animateTransform>`, `<animateMotion>`, and `<set>` elements
- All SMIL animation attributes

If an SVG cannot be fully sanitised without changing its visual output, the writer MUST refuse to embed it and MUST return an `E_ASSET_UNSAFE` error. Silent embedding of unsafe SVGs is prohibited.

**Image limits:**
- Maximum image dimensions: 16,384 × 16,384 pixels
- Maximum single image size: 25 MB compressed
- Supported formats: PNG, JPEG, JPEG 2000, WebP
- All images MUST be converted to sRGB colour space

**Font embedding:** All fonts used in a document MUST be embedded in `assets/fonts/`. Subset embedding is REQUIRED when the font contains more than 512 glyphs.

---

## 8. Error Codes

### 8.1 Severity Levels

| Severity | Prefix | Required Handler Behaviour |
|---|---|---|
| Fatal | `F_` | MUST abort processing. MUST NOT render a partial document |
| Error | `E_` | MUST log. SHOULD render document with affected area marked |
| Warning | `W_` | SHOULD log. MAY continue silently in production |

### 8.2 Fatal Errors

| Code | Description |
|---|---|
| `F_INVALID_CONTAINER` | File is not a valid ZIP archive or is corrupted |
| `F_MANIFEST_MISSING` | `manifest.json` is absent from the container root |
| `F_MANIFEST_POSITION` | `manifest.json` is not the first ZIP entry |
| `F_INVALID_JSON` | A required JSON file contains malformed JSON |
| `F_INVALID_VERSION` | `spdf:version` is absent, malformed, or from an unsupported major version |
| `F_CHECKSUM_MISMATCH` | SHA-256 of a file does not match its manifest entry |
| `F_PATH_TRAVERSAL` | A file path in the archive contains `..` or absolute path components |
| `F_CONTAINER_TOO_LARGE` | Uncompressed container exceeds 2 GB |
| `F_MISSING_REQUIRED_FILE` | A required file (semantic.json, layout.json, etc.) is absent |
| `F_DUPLICATE_EID` | Two or more elements share the same `eid` value |
| `F_CIRCULAR_REFERENCE` | An element's child chain contains a cycle |
| `F_DOM_TOO_LARGE` | DOM exceeds 1,000,000 elements |
| `F_NESTING_TOO_DEEP` | Element nesting exceeds 64 levels |

### 8.3 Errors

| Code | Description |
|---|---|
| `E_MISSING_REQUIRED_PROP` | A required property is absent from an element |
| `E_INVALID_TYPE` | A property value does not match its declared SPDF type |
| `E_INVALID_ENUM` | A property uses a value not in the declared enum registry |
| `E_ASSET_MISSING` | An element references an asset ID that is not in the container |
| `E_ASSET_UNSAFE` | An SVG asset could not be safely sanitised |
| `E_INVALID_DECIMAL` | A `spdf:decimal` value is not valid decimal notation |
| `E_INVALID_TIMESTAMP` | A `spdf:timestamp` is not ISO 8601 UTC format |
| `E_INVALID_COLOR` | A `spdf:color` is not `#RRGGBB` hex notation |
| `E_UNRESOLVED_EID` | A property references an EID that does not exist in the document |
| `E_SIGNATURE_INVALID` | The document hash does not match the signature value |
| `E_AUDIT_CHAIN_BROKEN` | An audit entry's `prev_hash` does not match the preceding entry's `entry_hash` |
| `E_LOCKED_ELEMENT_MODIFIED` | An attempt was made to modify an element with `locked: true` |
| `E_STATE_TRANSITION_INVALID` | The requested state transition is not permitted |
| `E_CONTENT_TOO_LARGE` | Text content exceeds the 4 MB per-element limit |
| `E_ASSET_TOO_LARGE` | An asset file exceeds its type's size limit |
| `E_TABLE_TOO_LARGE` | A table exceeds 100,000 rows |

### 8.4 Warnings

| Code | Description |
|---|---|
| `W_UNKNOWN_EXTENSION` | Unknown extension key encountered (preserved per FC-2) |
| `W_UNKNOWN_ELEMENT_TYPE` | Unknown element type encountered (preserved per FC-1) |
| `W_UNREFERENCED_ASSET` | Asset declared in manifest but not referenced in semantic.json |
| `W_UNEXPECTED_FILE` | File found in container root that is not in this specification |
| `W_LOW_CONTRAST` | Element fails WCAG 2.1 AA 4.5:1 colour contrast ratio |
| `W_DEPRECATED_FEATURE` | Document uses a feature marked deprecated in this version |

### 8.5 Validation Error Format

All SPDF implementations MUST return validation errors in this structure:

```json
{
  "errors": [
    {
      "code": "E_MISSING_REQUIRED_PROP",
      "severity": "Error",
      "message": "Required property 'level' is missing from element 'el-...-00014-c2f9'",
      "element_eid": "el-1709251200000-00014-c2f9",
      "element_type": "Heading",
      "property_path": "semantic.json#/document/pages/0/elements/2",
      "property_name": "level",
      "spec_reference": "SPDF Spec v1.0 Section 6.3",
      "remediation": "Add an integer 'level' property with value 1–6 to this Heading element"
    }
  ],
  "warnings": [],
  "is_valid": false,
  "validation_mode": "strict",
  "validated_at": "2025-03-15T09:30:00.000Z",
  "spdf_version": "1.0"
}
```

---

## 9. Versioning and Compatibility

### 9.1 Version Format

```
version = MAJOR "." MINOR

Examples: "1.0", "1.3", "2.0"
```

- **MAJOR** increment: breaking changes to schema or behaviour
- **MINOR** increment: backward-compatible additions only
- Patch versions (e.g. `1.0.1`) are NOT used. Errata to the spec do not change the version number

### 9.2 Compatibility Rules

| Reader Version | Document Version | Behaviour |
|---|---|---|
| 1.x | 1.0 | Full support — all features available |
| 1.0 | 1.3 | Older reader opens newer document — MUST work for known elements, preserve unknown elements per FC-1 through FC-4 |
| 1.0 | 2.0 | Major version mismatch — reader MUST display an upgrade prompt and render via `render.pdf` |
| 2.0 | 1.x | Newer reader opens older document — MUST support with deprecation warnings where applicable |

### 9.3 What Triggers a Major Version Bump

A MAJOR version increment is required for any of the following:

- Renaming or removing a required property
- Changing the semantics of an existing property
- Removing a required file from the container
- Changing the container format
- Changing the canonical form algorithm (this would invalidate all existing signatures)
- Removing or renaming an existing element type

When in doubt, bump the MAJOR version.

### 9.4 Backward Compatibility Guarantee (Minor Versions)

For MINOR version changes within the same MAJOR version series:

- New element types MAY be added. Existing types MUST NOT be removed
- New optional properties MAY be added to existing element types. Existing required properties MUST NOT be removed
- Required properties MUST NOT become required where they were previously optional
- New enum values MAY be added. Existing enum values MUST NOT be removed

---

## 10. Conformance Classes

### 10.1 Classes

SPDF defines four conformance classes. Each class is additive.

| Class | Identifier | Description |
|---|---|---|
| Minimal Reader | `SPDF-MR-1.0` | Opens any SPDF file and displays using `render.pdf` |
| Full Reader | `SPDF-R-1.0` | Full semantic parsing and screen rendering |
| Writer | `SPDF-W-1.0` | Can produce conformant SPDF documents |
| Signer | `SPDF-S-1.0` | Can sign and verify SPDF documents |

### 10.2 Minimal Reader Requirements (`SPDF-MR-1.0`)

- Parse `manifest.json` and validate all checksums
- Detect and report `F_` fatal errors
- Fall back to `render.pdf` for display

### 10.3 Full Reader Requirements (`SPDF-R-1.0`)

All Minimal Reader requirements, plus:

- Parse `semantic.json` into a typed in-memory DOM
- Parse `layout.json` and resolve layout for all elements
- Parse `styles.json` and resolve the full style cascade
- Decode all asset types (fonts, images, SVG)
- Render all element types defined in Section 6 to screen
- Apply the Forward Compatibility Protocol (Section 3.4) for unknown types and properties
- Enforce all allocation limits (Section 8.3)

### 10.4 Writer Requirements (`SPDF-W-1.0`)

All Full Reader requirements, plus:

- Produce all files required by the container specification (Section 4.3)
- Generate conformant element IDs (Section 5.3)
- Compute and embed correct SHA-256 checksums in `manifest.json`
- Embed or subset all referenced fonts
- Sanitise all SVG assets before embedding (Section 7.7)
- Apply strict validation before writing — MUST NOT write invalid documents
- Generate a conformant PDF 2.0 render layer
- Write an initial `CREATED` entry to `audit.json` with correct hash chain

### 10.5 Signer Requirements (`SPDF-S-1.0`)

All Writer requirements, plus:

- Compute document canonical form using RFC 8785 JCS (Section 7.4)
- Sign using RSA-PSS-SHA256 with X.509 certificate (Section 7.5)
- Write conformant signature JSON to `signatures/`
- Enforce the document state machine (Section 7.3)
- Manage the audit log on state transitions

---

## 11. Normative References

| Reference | Document | URL |
|---|---|---|
| ISO 32000-2 | PDF 2.0 | https://www.iso.org/standard/75839.html |
| RFC 8259 | JSON Data Interchange Format | https://tools.ietf.org/html/rfc8259 |
| RFC 8785 | JSON Canonicalization Scheme (JCS) | https://tools.ietf.org/html/rfc8785 |
| RFC 3629 | UTF-8 | https://tools.ietf.org/html/rfc3629 |
| RFC 2119 | Requirement Levels | https://tools.ietf.org/html/rfc2119 |
| RFC 3161 | Trusted Timestamping | https://tools.ietf.org/html/rfc3161 |
| RFC 3986 | URI Syntax | https://tools.ietf.org/html/rfc3986 |
| PKWARE AppNote | ZIP Format v6.3.10 | https://pkware.com/appnote |
| Unicode UAX #14 | Line Breaking Algorithm | https://unicode.org/reports/tr14/ |
| NIST SP 800-38D | AES-GCM | https://doi.org/10.6028/NIST.SP.800-38D |
| NIST FIPS 180-4 | SHA-256 | https://doi.org/10.6028/NIST.FIPS.180-4 |
| RFC 8017 | RSA-PSS | https://tools.ietf.org/html/rfc8017 |
| BCP 47 | Language Tags | https://tools.ietf.org/html/bcp47 |
| ISO 4217 | Currency Codes | https://www.iso.org/iso-4217-currency-codes.html |
| WCAG 2.1 | Accessibility Guidelines | https://www.w3.org/TR/WCAG21/ |

---

## 12. Change Log

| Version | Date | Changes |
|---|---|---|
| 1.0 | March 2025 | Initial public draft. Covers container architecture, DOM, element registry (structural, content, domain, trust, interactive), security model, error codes, versioning, and conformance classes |

---

## Contributing

SPDF is an open specification. Contributions, corrections, and proposals are welcome.

- **Bug reports and errata:** [Open an issue](https://github.com/spdf-foundation/spec/issues)
- **Proposals for new element types:** Open an issue with the `proposal` label and include a use case, schema definition, and example JSON
- **Security vulnerabilities:** Email security@spdf.dev. Please do not open public issues for security reports

Before contributing, read the [Contributing Guide](CONTRIBUTING.md).

## Reference Implementation

The reference implementation of the SPDF core engine (parser, writer, and renderer) is available at [github.com/spdf-foundation/spdf-core](https://github.com/spdf-foundation/spdf-core). It is written in Rust and published under the MIT licence.

Python and JavaScript/TypeScript SDKs are available at [github.com/spdf-foundation/spdf-python](https://github.com/spdf-foundation/spdf-python) and [github.com/spdf-foundation/spdf-js](https://github.com/spdf-foundation/spdf-js).

## Licence

This specification is published under the [Creative Commons Attribution 4.0 International License (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).

You are free to share and adapt this specification for any purpose, including commercial use, provided you give appropriate credit to the SPDF Foundation.

The reference implementation (spdf-core) is separately licenced under the MIT licence.

---

*Copyright © 2025 SPDF Foundation. All rights reserved under CC BY 4.0.*
