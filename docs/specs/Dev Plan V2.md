# SPDF Platform — 12-Week Development & GTM Plan
## SPDF-SPRINT-2025-001 · Version 2.0 · Cost-Optimised

---

## Cost Philosophy

Every service, feature, and tool in this plan earns its place by answering one question:
**"Does a developer evaluating the SDK today need this to exist?"**

If the answer is no, it is deferred until the revenue trigger that pays for it arrives.

| Period | Monthly Cost | What Changed |
|---|---|---|
| Weeks 1–10 | **$0** | Everything runs locally on TUF F15 |
| Week 11 | **$15–25** | Railway deploys after spec repo gets traction |
| Post-launch, 0 users | **$15–25** | Domain + Railway only |
| First Pro user ($29) | **~$0 net** | Revenue covers Railway. PDF conversion unlocks |
| 5 Pro users ($145) | **~$120 net** | Full platform viable |

---

## Deferred Feature Register

These features are **fully designed and documented** but **not built** until their revenue trigger fires. When the trigger fires, the feature slots into the next available sprint week.

| Feature | Trigger to Re-enable | Estimated Build Time |
|---|---|---|
| PDF → SPDF conversion (Claude API) | First Pro user pays ($29) | 2 days |
| Full X.509 certificate signing | First enterprise inquiry | 3 days |
| Natural language `/ask` endpoint | 10+ active users | 1 day |
| Webhook delivery + retry system | First GitHub issue requesting it | 2 days |
| Stripe live mode | Public launch day | 30 minutes (just a key swap) |
| Railway deployment | Spec repo getting traffic (Week 11) | 1 day |
| Supabase cloud PostgreSQL | Railway deployment | 30 minutes |
| Upstash cloud Redis | Railway deployment | 30 minutes |
| Cloudflare R2 cloud storage | Railway deployment | 30 minutes |
| Sentry error tracking | Railway deployment | 30 minutes |
| Doppler secrets management | Railway deployment | 1 hour |
| Resend transactional email | First real user signs up | 1 hour |

---

## The Two-Machine Workflow

```
ENTERPRISE DESKTOP                   GHE REPO                  PERSONAL LAPTOP (TUF F15)
──────────────────                (single source               ─────────────────────────
• Claude Code + Copilot             of truth)                  • All runtimes installed
• VS Code (code writing)                                       • Docker local services
• Architecture decisions         ◄── git pull ──               • cargo build / run
• AI-assisted code generation    ── git push ──►               • pytest / npm run dev
• Documentation + GTM writing                                  • Executes everything
• PR reviews on GHE                                            • GitHub Copilot

NEVER runs code                                                NEVER uses Claude Code
```

**Daily rhythm:**
1. Enterprise desktop: pull → write code with Claude Code → push
2. TUF F15: pull → build and test → push fixes
3. Never need both open simultaneously

---

## How to Read This Plan

- 🤖 **Claude Code** — exact prompt guidance (enterprise desktop)
- 🖐 **Manual Action** — numbered step-by-step guide
- 💻 **Run on TUF F15** — commands to execute on personal laptop
- ✅ **Verify** — how to confirm the day is complete
- ⚠️ **Risk** — common failure points
- ⏱ **Time** — realistic allocation per task

---

## Phase Overview

| Phase | Weeks | Theme | Cost | Key Deliverable |
|---|---|---|---|---|
| **1 — Foundation** | 1–2 | Environment + Rust Core Engine | $0 | Valid SPDF files generated, parsed, validated |
| **2 — Backend Core** | 3–4 | FastAPI + Auth + Billing stub | $0 | `/generate`, `/extract`, auth, rate limiting |
| **3 — Studio + SDKs** | 5–6 | React frontend + Python/TS SDKs | $0 | Studio live locally, both SDKs installable |
| **4 — Hardening** | 7–8 | Testing, security, performance | $0 | Production-grade reliability locally |
| **5 — GTM Content** | 9–10 | Docs, landing page, launch assets | $0 | All content written and ready |
| **6 — Launch** | 11–12 | Deploy + publish + distribute | $15–25 | Public launch, first users, spec repo live |

---

# WEEK 1 — Environment Setup + Project Scaffold

**Theme:** Every tool installed on the TUF F15. GHE repo created. Monorepo skeleton committed. Zero cost.

**Weekly Goal:** `just dev` starts Docker services on TUF F15. GHE repo syncs between both machines. `cargo check` passes.

**Accounts to create this week:** GitHub personal account (if needed), Cloudflare (free, for DNS — no R2 yet), Anthropic (set up now, use later).

---

## Day 1 — TUF F15: WSL2 + Core Tools + Rust

🎯 **Goal:** WSL2 running Ubuntu 22.04, Rust installed and verified, Python 3.12, Node 20, Docker, and `just` all working.

⏱ **4.5 hours — all on TUF F15**

---

### 🖐 Manual Action 1.1 — Enable WSL2

**Open PowerShell as Administrator on TUF F15:**

```powershell
wsl --install
```

Restart when prompted. Ubuntu 22.04 opens automatically after restart. Create username and password when asked.

```bash
# In the Ubuntu terminal:
sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential curl git wget unzip pkg-config \
  libssl-dev libffi-dev python3-dev ca-certificates gnupg lsb-release \
  software-properties-common apt-transport-https
```

⚠️ **If virtualisation error:** TUF F15 BIOS has virtualisation on by default. If you hit an error, press F2 on boot → Advanced → CPU Configuration → Intel Virtualization Technology → Enabled.

---

### 🖐 Manual Action 1.2 — Install Rust

```bash
# In WSL Ubuntu terminal:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Choose option 1 (default)

source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
pip3 install maturin

rustc --version    # rustc 1.75.0 or later
cargo --version
wasm-pack --version
```

---

### 🖐 Manual Action 1.3 — Install Python 3.12, Node 20, Docker, just

```bash
# Python 3.12
sudo add-apt-repository ppa:deadsnakes/ppa -y
sudo apt update
sudo apt install -y python3.12 python3.12-dev python3.12-venv python3.12-distutils
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.12
python3.12 --version    # Python 3.12.x

# Node.js 20
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
node --version          # v20.x.x

# Docker
sudo apt install -y docker.io docker-compose
sudo usermod -aG docker $USER
newgrp docker
docker --version        # Docker version 24.x or later

# just
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/.local/bin
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
just --version
```

---

### 🖐 Manual Action 1.4 — Configure VS Code for WSL

1. Install [VS Code for Windows](https://code.visualstudio.com/) on TUF F15 (Windows side)
2. Extensions → install **WSL** by Microsoft
3. `Ctrl+Shift+P` → "WSL: Connect to WSL"
4. Inside WSL-connected VS Code install: `rust-analyzer`, `Python`, `Pylance`, `ESLint`, `Prettier`, `GitHub Copilot`
5. Default terminal → WSL Bash: `Ctrl+Shift+P` → "Terminal: Select Default Profile" → "WSL Bash"

✅ **Day 1 Verification:** All version commands return numbers. Zero errors.

---

## Day 2 — GHE Repo + Git Configuration on Both Machines

🎯 **Goal:** GHE repository created, both machines clone and push/pull successfully.

⏱ **3 hours**

---

### 🖐 Manual Action 2.1 — Create GHE Repository

**On enterprise desktop browser:**

1. Open your company's GitHub Enterprise → **+** → **New repository**
2. Owner: your personal GHE account (not a work org)
3. Name: `spdf` · Visibility: **Private** · Add README, `.gitignore` (Rust), MIT license
4. Copy the HTTPS clone URL

---

### 🖐 Manual Action 2.2 — Configure Git + Clone on TUF F15

```bash
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
git config --global core.autocrlf false
git config --global init.defaultBranch main
git config --global credential.helper store

# Generate PAT on GHE: Settings → Developer Settings → Personal access tokens
# Scopes: repo, workflow — label it "tuf-f15"

mkdir -p ~/projects && cd ~/projects
git clone https://YOUR-GHE-HOST/YOUR-USERNAME/spdf.git
cd spdf
# Username = GHE username, Password = PAT token (stored after first use)
```

---

### 🖐 Manual Action 2.3 — Configure Git on Enterprise Desktop

```bash
# In Git Bash on enterprise desktop:
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
git config --global core.autocrlf false

# Generate a second PAT on GHE — label it "enterprise-desktop"
mkdir -p C:/Projects && cd C:/Projects
git clone https://YOUR-GHE-HOST/YOUR-USERNAME/spdf.git
cd spdf && code .
```

✅ **Day 2 Verification:** Test change on enterprise desktop → push → pull on TUF F15 → confirms sync works.

---

## Day 3 — Monorepo Scaffold

🎯 **Goal:** Complete monorepo structure committed. `cargo check` passes. Docker services start.

⏱ **4 hours (2h enterprise writing, 2h TUF F15 verifying)**

---

### 🤖 Claude Code Task 3.1 — Generate Monorepo Scaffold

**Enterprise desktop, in `C:/Projects/spdf`:**

```
Create a complete monorepo scaffold for the SPDF platform.

Directory structure:
spdf/
├── .github/workflows/         (empty — CI added in Week 7)
├── crates/
│   ├── spdf-core/             (Rust library crate — the core engine)
│   ├── spdf-wasm/             (Rust WASM bindings for browser)
│   └── spdf-python/           (Rust PyO3 bindings for Python SDK)
├── services/
│   ├── api/                   (Python FastAPI application)
│   └── worker/                (Python Celery — STUBBED, not built yet)
├── apps/
│   └── studio/                (React + Vite frontend)
├── packages/
│   ├── spdf-python/           (pip-publishable Python SDK)
│   └── spdf-js/               (npm-publishable TypeScript SDK)
├── spec/                      (SPDF format specification)
├── docs/                      (Developer documentation)
├── scripts/                   (Utility scripts)
├── Cargo.toml                 (Rust workspace manifest)
├── docker-compose.yml         (Local dev: postgres, redis, minio only)
├── justfile                   (Task runner)
└── .env.example

Create:
1. Root Cargo.toml workspace including all three crates

2. crates/spdf-core/Cargo.toml with dependencies:
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   zip = "0.6"
   sha2 = "0.10"
   uuid = { version = "1.0", features = ["v4", "serde"] }
   chrono = { version = "0.4", features = ["serde"] }
   thiserror = "1.0"
   lopdf = "0.28"
   fontdue = "0.7"
   base64 = "0.21"
   hex = "1.0"

3. crates/spdf-core/src/lib.rs with:
   - SpdfError enum using thiserror: InvalidContainer, InvalidJson,
     ChecksumMismatch, MissingRequiredFile, InvalidVersion, DuplicateEid,
     AssetMissing, SignatureInvalid, IoError(#[from] std::io::Error)
   - SpdfVersion struct with major: u32, minor: u32 and parse() method
   - pub fn validate_container(bytes: &[u8]) -> Result<(), SpdfError> stub

4. Minimal lib.rs in spdf-wasm and spdf-python with hello() stub
5. spdf-wasm and spdf-python Cargo.toml as cdylib with wasm-bindgen and pyo3 deps

6. docker-compose.yml with only three services (NO cloud services):
   - postgres:15 port 5432, db: spdf_dev, user: spdf, password: spdf_dev_password
   - redis:7-alpine port 6379
   - minio/minio ports 9000/9001 (local R2 replacement — S3-compatible)
   All with health checks and named volumes.
   NOTE: This is the only infrastructure we run. No cloud services during development.

7. justfile recipes:
   dev: docker-compose up -d
   stop: docker-compose down
   build-rust: cargo build --workspace
   build-wasm: wasm-pack build crates/spdf-wasm --target web
   build-pyo3: maturin develop --release in crates/spdf-python
   test-rust: cargo test --workspace
   test-python: pytest services/ packages/spdf-python/ -v
   test-frontend: cd apps/studio && npm test
   db-migrate: alembic -c services/api/alembic.ini upgrade head
   db-rollback: alembic -c services/api/alembic.ini downgrade -1
   lint: cargo clippy -- -D warnings
   clean: cargo clean

8. .env.example with local-only values (no cloud URLs yet):
   DATABASE_URL=postgresql+asyncpg://spdf:spdf_dev_password@localhost:5432/spdf_dev
   REDIS_URL=redis://localhost:6379/0
   R2_ENDPOINT=http://localhost:9000
   R2_ACCESS_KEY_ID=minioadmin
   R2_SECRET_ACCESS_KEY=minioadmin
   R2_BUCKET_DOCUMENTS=spdf-documents
   R2_BUCKET_UPLOADS=spdf-uploads
   ANTHROPIC_API_KEY=not-needed-yet
   CLERK_SECRET_KEY=not-needed-yet
   STRIPE_SECRET_KEY=not-needed-yet
   ENVIRONMENT=development
   LOG_LEVEL=debug

9. .gitignore additions: .env, .env.local, target/, __pycache__/, *.pyd, *.so,
   .venv/, node_modules/, dist/, pkg/

Generate all files completely. No placeholder comments.
```

```bash
# Enterprise desktop after generation:
git add -A
git commit -m "chore: initial monorepo scaffold"
git push
```

---

### 💻 Run on TUF F15

```bash
cd ~/projects/spdf && git pull
cargo check --workspace       # must pass with zero errors
just dev                       # starts postgres, redis, minio
docker ps                      # shows 3 containers running

# Create .env.local (copy from .env.example — already has correct local values)
cp .env.example .env.local
```

✅ **Day 3 Verification:** `cargo check` passes. `just dev` starts 3 containers. `.env.local` exists.

---

## Day 4 — Minimal Account Setup (30 Minutes Only)

🎯 **Goal:** Only the three accounts needed this week. Nothing else.

⏱ **1 hour — enterprise desktop browser**

**Why only three:** Every other account is created the day you build the feature that needs it. Creating accounts weeks early adds noise and wastes time.

---

### 🖐 Manual Action 4.1 — Create Three Accounts

**1. Anthropic (10 minutes)**
1. Go to [console.anthropic.com](https://console.anthropic.com) → create account → verify email
2. Add payment method → set spend limit: **$10/month** (enough for occasional testing; raise when conversion feature is re-enabled)
3. API Keys → Create Key → name: `spdf-dev` → copy key
4. Add to `.env.local`: `ANTHROPIC_API_KEY=sk-ant-...`

**2. Cloudflare (15 minutes)**
1. Go to [cloudflare.com](https://cloudflare.com) → create free account
2. **Do not create R2 buckets yet** — local MinIO handles storage until Week 11
3. Note your account login for Week 11 when you configure DNS and R2

**3. GitHub personal (5 minutes)**
Only needed if your personal GitHub account is separate from your GHE account. If your GHE account is personal, skip this.

**That is Day 4. Done in 30 minutes. Everything else waits.**

---

## Day 5 — FastAPI Skeleton + Health Endpoints

🎯 **Goal:** FastAPI app runs locally, `GET /v1/health` works, connects to local Postgres via Docker.

⏱ **4.5 hours**

---

### 🤖 Claude Code Task 5.1 — FastAPI Application Skeleton

**Enterprise desktop:**

```
Create a production-ready FastAPI application skeleton in services/api/.

IMPORTANT CONSTRAINTS:
- No Clerk auth yet (added Week 3)
- No Stripe yet (added Week 4)
- No Sentry yet (added Week 7)
- No Redis rate limiting yet (added Week 3)
- Authentication is STUBBED: a hardcoded test API key "sk_test_dev_key"
  is accepted on all endpoints for local development

Directory structure:
services/api/
├── main.py              (FastAPI app factory)
├── config.py            (Pydantic Settings from .env.local)
├── dependencies.py      (DI: db, current_user stub)
├── routers/
│   ├── __init__.py
│   ├── health.py        (GET /v1/health, GET /v1/health/ready)
│   └── documents.py     (stubs only — implemented Week 2)
├── models/
│   ├── __init__.py
│   ├── base.py          (DeclarativeBase, TimestampMixin, SoftDeleteMixin)
│   ├── user.py          (User model)
│   ├── organization.py  (Organization + OrgMember)
│   ├── document.py      (Document model)
│   └── job.py           (ConversionJob — stub, not used until conversion re-enabled)
├── schemas/
│   ├── __init__.py
│   ├── health.py        (HealthResponse, ReadinessResponse)
│   └── errors.py        (StandardErrorResponse matching API contract)
├── middleware/
│   ├── __init__.py
│   ├── request_id.py    (inject X-Request-Id UUID4)
│   └── logging.py       (structured JSON logging)
├── migrations/
│   └── env.py
├── alembic.ini
├── pyproject.toml
└── requirements.txt

Config reads from environment variables.
Use .env.local values (DATABASE_URL, REDIS_URL, R2_* — all point to local Docker).

Health endpoints:
GET /v1/health → {"status": "ok", "version": "1.0.0", "environment": "development", "timestamp": "..."}
GET /v1/health/ready → actually check postgres and redis connectivity, return {"status": "ready", "checks": {...}}

Stub auth dependency: get_current_user returns a hardcoded User object when
Authorization header equals "Bearer sk_test_dev_key". Returns 401 otherwise.
This stub is replaced with real Clerk auth in Week 3.

SQLAlchemy models use UUID primary keys, TIMESTAMPTZ columns, soft delete via deleted_at.

Create initial Alembic migration for tables: users, organizations, org_members, documents.

requirements.txt:
fastapi==0.110.0, uvicorn[standard]==0.27.0, sqlalchemy[asyncio]==2.0.25,
alembic==1.13.0, asyncpg==0.29.0, pydantic==2.6.0, pydantic-settings==2.1.0,
python-multipart==0.0.9, httpx==0.26.0, redis==5.0.0, structlog==24.1.0,
passlib[bcrypt]==1.7.4, python-jose[cryptography]==3.3.0, boto3==1.34.0
```

---

### 💻 Run on TUF F15

```bash
cd ~/projects/spdf && git pull
cd services/api
python3.12 -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt

# Load local env vars
set -a && source ../../.env.local && set +a

# Run migrations against local Docker postgres
alembic upgrade head

# Start API
uvicorn main:app --host 0.0.0.0 --port 8000 --reload

# Test in another terminal
curl http://localhost:8000/v1/health | python3.12 -m json.tool
curl http://localhost:8000/v1/health/ready | python3.12 -m json.tool
```

✅ **Week 1 Complete:** Both health endpoints return correct JSON. `cargo check` passes. Three Docker containers running.

---

# WEEK 2 — Rust Core Engine

**Theme:** The most technically complex week. Claude Code writes all Rust. Your job: review, compile, iterate.

**Weekly Goal:** `cargo test --workspace` passes. Integration test creates a valid SPDF invoice, round-trips it, validates it, renders it as PDF.

**Cost this week: $0**

---

## Day 6 — Container Layer: ZIP Structure + Manifest + Checksums

🎯 **Goal:** Core engine creates a valid SPDF ZIP container with correct checksums.

⏱ **4.5 hours**

---

### 🤖 Claude Code Task 6.1 — Container Builder

```
In crates/spdf-core/src/, implement the SPDF container layer.

Create:
- src/container/mod.rs
- src/container/manifest.rs
- src/container/reader.rs
- src/container/writer.rs
- src/container/checksums.rs

manifest.rs:
- SpdfManifest with: format ("SPDF"), version, profile, created_at, generator,
  layers (LayerManifests), assets (Vec<AssetManifestEntry>), extensions, document_id, manifest_hash
- All structs: Serialize, Deserialize, Debug, Clone
- version serializes as "1.0" not {"major":1,"minor":0}
- SpdfManifest::new() creates default v1.0 manifest
- SpdfManifest::compute_hash() → SHA-256 of canonical JSON

checksums.rs:
- fn sha256_hex(data: &[u8]) -> String  (lowercase hex)
- fn verify_checksum(data: &[u8], expected_hex: &str) -> bool

writer.rs — SpdfWriter struct:
- SpdfWriter::new()
- fn add_layer(&mut self, name: &str, content: &[u8])
- fn add_asset(&mut self, asset_id: &str, asset_type: &str, mime: &str, content: &[u8])
- fn build(self) -> Result<Vec<u8>, SpdfError>
  CRITICAL: manifest.json MUST be the first ZIP entry
  Computes SHA-256 for every file, embeds in manifest
  Returns raw ZIP bytes

reader.rs:
- fn read_container(bytes: &[u8]) -> Result<SpdfContainer, SpdfError>
- Validates ZIP magic bytes, verifies manifest.json is first entry
- Reads and parses manifest, verifies all checksums
- SpdfContainer exposes: manifest, semantic_bytes, layout_bytes,
  styles_bytes, render_bytes, metadata_bytes, audit_bytes

Write #[cfg(test)] inline unit tests.
Most important test: create minimal SPDF, write it, read it back,
verify all checksums match (round-trip test).
```

---

### 💻 Run on TUF F15

```bash
cd ~/projects/spdf && git pull
cargo test -p spdf-core container
```

⚠️ **Compile-fix loop:** Rust is strict. Expect 2–3 rounds of errors. Pattern:
1. Compile on TUF F15 → copy FULL error output
2. Enterprise desktop → Claude Code: "Fix these Rust errors: [paste full output]"
3. Push → pull on TUF F15 → recompile
Budget 30 minutes per round. Never manually edit Rust errors.

---

## Day 7 — DOM Type System

🎯 **Goal:** Full DOM in Rust — Document, Page, all 25 element types from the spec.

⏱ **4.5 hours**

---

### 🤖 Claude Code Task 7.1 — DOM Types

```
In crates/spdf-core/src/dom/, implement the SPDF Document Object Model.

Create:
- src/dom/mod.rs
- src/dom/document.rs  (Document, Page, DocumentState)
- src/dom/element_id.rs (ElementId generation and validation)
- src/dom/elements/mod.rs
- src/dom/elements/content.rs   (Heading, Paragraph, Table, TableRow, TableCell, Image, VectorImage, CodeBlock, HorizontalRule, PageBreak, Attachment)
- src/dom/elements/domain.rs    (InvoiceHeader, LineItem, LineItemTable, PaymentTerms)
- src/dom/elements/trust.rs     (SignatureBlock, Stamp, Annotation, Redaction)
- src/dom/elements/interactive.rs (FormField, VariablePlaceholder)

element_id.rs:
- ElementId is a newtype over String
- ElementId::new(prefix: &str, timestamp_ms: u64, sequence: u32) -> ElementId
- Format: "{prefix}-{timestamp_ms}-{sequence:05}-{checksum}"
- checksum = first 4 hex chars of SHA-256("{prefix}{timestamp_ms}{sequence}")
- ElementId::validate(s: &str) -> bool

document.rs:
- DocumentState enum: Draft, Review, Signed, Certified
- DocumentState::valid_transitions() -> &[DocumentState]
  (Draft→Review, Review→Draft, Review→Signed, Signed→Certified; never reverse from Signed)
- Document { eid, element_type: "Document", title, locale, direction, document_state, page_count, pages: Vec<Page> }
- Page { eid, element_type: "Page", elements: Vec<SpdfElement> }
- Document::new(title: &str, locale: &str) -> Document

For every element struct:
- All fields from SPDF Format Specification Section 6
- Universal properties via UniversalProps struct with #[serde(flatten)]:
  eid, element_type, version, created_at, modified_at, style_id,
  integrity_hash, locked, visible, accessible_label, custom_data
- ALL financial values (unit_price, total, tax_amount, etc.) MUST be String type — NEVER f64
- All timestamps: chrono::DateTime<chrono::Utc>

SpdfElement enum:
- Variant for every element type
- #[serde(tag = "element_type")] for automatic type-based serialization

Tests:
- Create Document → add Page → add Heading + Paragraph → serialize → deserialize → verify round-trip
- Verify DocumentState::Draft.valid_transitions() contains Review but NOT Signed
- Verify ElementId format matches spec pattern
```

---

## Day 8 — Validator

🎯 **Goal:** `validate_document()` catches all E_ and F_ error codes from the spec.

---

### 🤖 Claude Code Task 8.1 — Document Validator

```
In crates/spdf-core/src/validation/, implement the SPDF validator.

Create:
- src/validation/mod.rs
- src/validation/rules.rs
- src/validation/report.rs

report.rs:
- ValidationSeverity enum: Fatal, Error, Warning
- ValidationIssue { code, severity, message, element_eid, element_type,
  property_path, property_name, spec_reference, remediation }
- ValidationReport { errors, warnings, is_valid, validation_mode, validated_at, spdf_version }
- is_valid() → true only if zero Fatal and zero Error issues

rules.rs — implement these validation functions:
- validate_duplicate_eids(doc: &Document) -> Vec<ValidationIssue>
- validate_element_types(doc: &Document) -> Vec<ValidationIssue>
- validate_required_properties(element: &SpdfElement) -> Vec<ValidationIssue>
- validate_financial_values(doc: &Document) -> Vec<ValidationIssue>
  (every spdf:decimal field must be valid decimal string — no commas, no currency symbols)
- validate_timestamps(doc: &Document) -> Vec<ValidationIssue>
  (all timestamps must end with Z — UTC only)
- validate_document_state(doc: &Document) -> Vec<ValidationIssue>

pub fn validate_document(document: &Document, mode: ValidationMode) -> ValidationReport
  runs all rules and returns complete report
```

---

## Day 9 — PDF Render Layer Generator

🎯 **Goal:** Core engine generates an openable PDF from an SPDF document.

---

### 🤖 Claude Code Task 9.1 — PDF Renderer

```
In crates/spdf-core/src/render/, implement PDF render layer generation using lopdf.

Create:
- src/render/mod.rs
- src/render/pdf_writer.rs
- src/render/text.rs
- src/render/page.rs

pdf_writer.rs:
- fn render_to_pdf(document: &Document) -> Result<Vec<u8>, SpdfError>
- Produces a valid, openable PDF 2.0 file
- Page size: A4 (595.28 x 841.89 points)
- Default margins: 56.69pt (20mm) all sides
- Use Helvetica built-in PDF font (no custom embedding yet)
- Render these elements: Heading (bold text), Paragraph (normal text),
  HorizontalRule (horizontal line), InvoiceHeader (structured text block),
  LineItem (table row), LineItemTable (table with header + rows)
- Elements are laid out top-to-bottom within the content area
- Auto-paginate when content exceeds page height

Test: generate PDF from Document with one Heading + two Paragraphs.
Write to /tmp/test_render.pdf. Verify file starts with %PDF.
```

---

## Day 10 — Full Round-Trip Integration Test

🎯 **Goal:** Create invoice → write SPDF → read back → validate → render PDF. All assertions pass.

---

### 🤖 Claude Code Task 10.1 — Integration Test

```
Create crates/spdf-core/tests/integration_test.rs

The test must:
1. Build a complete invoice document:
   - Title: "Test Invoice #INV-2025-001", Locale: "en-IN"
   - One A4 Page
   - InvoiceHeader: vendor "ACME Corp", client "Test Client",
     invoice_number "INV-001", issue_date today, due_date +30 days
   - LineItemTable with 2 items:
     {"Backend Dev", qty: "80", unit: "hours", unit_price: "2500.00", total: "200000.00", currency: "INR"}
     {"DevOps Setup", qty: "40", unit: "hours", unit_price: "2000.00", total: "80000.00", currency: "INR"}
   - PaymentTerms: terms "Net 30"
   - SignatureBlock: required_from "CLIENT", lock_on_sign: true

2. Serialize document to semantic_json string
3. Create minimal layout_json and styles_json (empty objects)
4. Create metadata_json with document metadata
5. Create audit_json with CREATED entry (seq:1, prev_hash: 64 zeros)
6. Render to PDF bytes using render_to_pdf()
7. Build complete SPDF container using SpdfWriter
8. Write to /tmp/test_invoice_001.spdf
9. Read back using read_container()
10. Verify all checksums match
11. Parse semantic_json back to Document
12. Run validate_document() → assert is_valid: true
13. Assert invoice_number == "INV-001"
14. Assert both line items present
15. Assert document state is Draft
16. Verify .spdf file magic bytes are ZIP (PK\x03\x04)

Write to /tmp/test_invoice_001.pdf the rendered PDF.
Print: "✅ Integration test passed. SPDF: {X} bytes. PDF: {Y} bytes."
```

### 💻 Run on TUF F15

```bash
cd ~/projects/spdf && git pull
cargo test -p spdf-core --test integration_test -- --nocapture
# Copy PDF to Windows for visual inspection:
cp /tmp/test_invoice_001.pdf /mnt/c/Users/YOUR_USERNAME/Desktop/
# Open from Windows Desktop — must be a readable invoice PDF
```

✅ **Week 2 Complete:** `cargo test --workspace` passes. Integration test prints success. PDF opens in any viewer.

---

# WEEK 3 — Python API: Generation + Extraction Endpoints

**Theme:** The REST API delivers core developer value. No Claude API. No cloud services.

**Weekly Goal:** `POST /v1/documents/generate` returns a working SPDF. `POST /v1/documents/{id}/extract` returns structured invoice data. Stub auth replaced with real Clerk auth.

**Cost this week: $0**

**Accounts to create this week:** Clerk (free tier, needed for Day 11)

---

## Day 11 — PyO3 Bindings: Rust → Python

🎯 **Goal:** Python can call `validate_spdf()`, `generate_spdf()`, `render_to_pdf()`, and `extract_invoice_data()` from the Rust core.

---

### 🖐 Manual Action 11.1 — Create Clerk Account

**Enterprise desktop browser (15 minutes):**
1. Go to [clerk.com](https://clerk.com) → Sign up
2. Create application → name: `SPDF` → enable Email + Google sign-in
3. API Keys → copy `CLERK_PUBLISHABLE_KEY` and `CLERK_SECRET_KEY`
4. Webhooks → Add Endpoint → URL: `http://localhost:8000/v1/webhooks/clerk`
   Events: `user.created`, `user.updated`, `user.deleted`
5. Copy `CLERK_WEBHOOK_SECRET`
6. Add to `.env.local` on TUF F15:
   ```
   CLERK_SECRET_KEY=sk_test_...
   CLERK_WEBHOOK_SECRET=whsec_...
   ```

---

### 🤖 Claude Code Task 11.2 — PyO3 Bindings

```
In crates/spdf-python/src/lib.rs, expose these functions to Python via PyO3:

Module name: "spdf_native"

Functions:
1. validate_spdf(spdf_bytes: &[u8]) -> PyResult<String>
   Calls validation::validate_document(), returns ValidationReport as JSON string

2. generate_spdf(semantic_json: &str, layout_json: &str, styles_json: &str,
                 metadata_json: &str, audit_json: &str) -> PyResult<Vec<u8>>
   Builds complete SPDF container, returns raw bytes

3. render_to_pdf(spdf_bytes: &[u8]) -> PyResult<Vec<u8>>
   Reads SPDF, calls render_to_pdf(), returns PDF bytes

4. parse_semantic(semantic_json: &str) -> PyResult<String>
   Parses semantic.json, returns Document as JSON string

5. extract_invoice_data(spdf_bytes: &[u8]) -> PyResult<String>
   Finds InvoiceHeader, LineItemTable, PaymentTerms elements
   Returns structured dict as JSON string:
   { "invoice_number", "issue_date", "due_date", "vendor", "client",
     "line_items": [...], "subtotal", "tax_amount", "total", "currency" }

In services/api/core/spdf_engine.py create Python wrapper:
class SpdfEngine:
    @staticmethod
    def validate(spdf_bytes: bytes) -> dict: ...
    @staticmethod
    def generate(semantic: dict, layout: dict, styles: dict, metadata: dict) -> bytes: ...
    @staticmethod
    def render_pdf(spdf_bytes: bytes) -> bytes: ...
    @staticmethod
    def extract(spdf_bytes: bytes) -> dict: ...
```

### 💻 Run on TUF F15

```bash
cd ~/projects/spdf/crates/spdf-python
maturin develop --release
python3.12 -c "import spdf_native; print('PyO3 bindings working')"
```

---

## Days 12–13 — Document Generation + Extraction Endpoints

🎯 **Goal:** Both core API endpoints fully functional with local file storage (MinIO).

---

### 🤖 Claude Code Task 12.1 — Generation + Extraction Endpoints

```
Implement document endpoints in services/api/routers/documents.py.

Replace the stub auth with real Clerk JWT verification.

1. Clerk auth middleware in services/api/middleware/auth.py:
   - JWT path: "Bearer eyJ..." → verify with Clerk JWKS
     Fetch JWKS from https://YOUR_CLERK_DOMAIN/.well-known/jwks.json
     Cache in memory for 1 hour
     Extract user_id (sub claim), load User from database
   - API key path: "Bearer sk_test_..." → look up in users.api_key_hash via bcrypt verify
   - Fallback: return 401 with AUTH_REQUIRED error code
   - Replace the hardcoded stub from Week 1 Day 5

2. POST /v1/documents/generate (exact contract from SPDF-API-2025-001 Section 4.2):
   - Validate request body: GenerateDocumentRequest
   - Look up template from templates table (or use a hardcoded default template for now)
   - Build semantic_json, layout_json, styles_json, metadata_json, audit_json from request data
   - Call SpdfEngine.generate() → returns .spdf bytes
   - Upload to MinIO (local R2): key = users/{user_id}/documents/{doc_id}/{doc_id}.spdf
   - INSERT into documents table
   - Generate presigned MinIO URL (1 hour expiry)
   - Return GenerateDocumentResponse matching API contract

3. POST /v1/documents/{document_id}/extract (API contract Section 4.6):
   - Fetch .spdf bytes from MinIO using r2_key
   - Call SpdfEngine.extract()
   - Return structured invoice data with confidence scores

4. GET /v1/documents/{document_id} — metadata + fresh presigned URL
5. GET /v1/documents — paginated list, cursor-based
6. DELETE /v1/documents/{document_id} — soft delete, 409 if SIGNED

7. services/api/services/storage_service.py:
   - Uses boto3 configured for MinIO (same S3 API as R2)
   - R2_ENDPOINT from env points to http://localhost:9000 during development
   - Same code works for real R2 in production — only the endpoint URL changes

8. services/api/services/document_service.py:
   - generate(), get_by_id(), list_for_user(), soft_delete()

IMPORTANT: No conversion worker, no Celery, no async job queue.
The generate endpoint is fully synchronous — call Rust → store file → return response.
The upload endpoint (PDF→SPDF conversion) is NOT built yet — deferred.
```

---

### 🤖 Claude Code Task 13.1 — Clerk Webhook Handler

```
Create services/api/routers/webhooks.py:

POST /v1/webhooks/clerk:
- Verify Clerk webhook signature using svix library
- user.created → INSERT into users table (clerk_user_id, email, display_name, tier='FREE')
- user.updated → UPDATE users table
- user.deleted → soft delete users table (set deleted_at = NOW())
- Return 200 always (Clerk retries on non-200)

Add a default invoice template to the database seed:
In scripts/seed_db.py insert template:
  id: "tmpl_invoice_gst_india"
  name: "GST Invoice (India)"
  category: "Invoice"
  is_public: true
  variable_schema: { required: [INVOICE_NUMBER, ISSUE_DATE, DUE_DATE,
    VENDOR_NAME, VENDOR_ADDRESS, VENDOR_GSTIN, CLIENT_NAME, CLIENT_ADDRESS,
    CLIENT_GSTIN, LINE_ITEMS, CURRENCY], optional: [TAX_SCHEME, PAYMENT_TERMS] }
```

---

## Days 14–15 — Account Endpoints + Rate Limiting

🎯 **Goal:** API keys work. Rate limits enforced. `/account/api-key` and `/account/usage` live.

---

### 🤖 Claude Code Task 14.1 — API Keys + Rate Limiting

```
1. API key generation in services/api/routers/account.py:

   GET /v1/account/api-key
   Returns: { key_prefix: "sk_live_aBcDeFgH", created_at: "...", last_used_at: "..." }
   The full key is NEVER returned after creation.

   POST /v1/account/api-key/rotate:
   - Generate: "sk_live_" + 26 random base62 chars using secrets.token_urlsafe
   - bcrypt hash it (cost=12) using passlib
   - UPDATE users.api_key_hash and users.api_key_prefix
   - INSERT into api_keys table (mark old key revoked)
   - Return the FULL key ONCE with warning: "Store this key securely. Not shown again."

   GET /v1/account/usage:
   Returns today's usage from usage_events table grouped by event_type.

2. Redis rate limiting in services/api/middleware/rate_limit.py:

   Limits per user_id per endpoint family per calendar day UTC:
   FREE:  convert=10, generate=50, extract=100, sign=50, other=500
   PRO:   convert=1000, generate=5000, extract=10000, sign=2000, other=50000
   TEAM:  convert=10000, generate=50000, extract=100000, sign=20000, other=500000
   ENTERPRISE: skip check

   Redis key: "ratelimit:{user_id}:{endpoint_family}:{YYYY-MM-DD-UTC}"
   TTL: 86400 seconds
   On exceeded: return 429 with RATE_LIMIT_EXCEEDED error + X-RateLimit-* headers
   Use Redis INCR + EXPIRE pipeline (atomic)

   Endpoint family mapping:
   /v1/documents/upload → "convert" (stub, not built yet)
   /v1/documents/generate → "generate"
   /v1/documents/*/extract → "extract"
   /v1/documents/*/sign → "sign"
   everything else → "other"

   Add X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset
   to EVERY response, not just 429s.

3. usage_events recording:
   After every successful billable operation, INSERT into usage_events:
   (user_id, event_type, document_id, occurred_at, billable=true, billed=false)
```

✅ **Week 3 Complete:**
```bash
curl -X POST http://localhost:8000/v1/documents/generate \
  -H "Authorization: Bearer sk_test_dev_key" \
  -H "Content-Type: application/json" \
  -d '{"template_id":"tmpl_invoice_gst_india","title":"Test","locale":"en-IN",
       "data":{"INVOICE_NUMBER":"INV-001","VENDOR_NAME":"ACME","CLIENT_NAME":"Client",
       "LINE_ITEMS":[{"description":"Dev","qty":"10","unit_price":"5000.00"}],
       "CURRENCY":"INR"},"options":{"include_render_layer":true}}' \
  | python3.12 -m json.tool
# Must return 201 with download_url
```

---

# WEEK 4 — Billing Stub + Simplified Signing

**Theme:** Commercial structure in place. Signing works (simplified). Billing reads revenue. No Stripe live mode yet.

**Cost this week: $0**

**Accounts to create this week:** Stripe test mode (Day 16)

---

## Day 16 — Stripe Test Mode Setup + Billing Endpoints

🎯 **Goal:** Stripe webhook updates subscription tier. Pro tier gets higher rate limits.

---

### 🖐 Manual Action 16.1 — Create Stripe Account (Test Mode Only)

**Enterprise desktop browser (20 minutes):**
1. Go to [stripe.com](https://stripe.com) → create account
2. Ensure **Test Mode** toggle is ON (top left — stays in test mode until launch day)
3. Developers → API Keys → copy `sk_test_...` key
4. Create 3 products in test mode:
   - Pro: $29/month → copy Price ID (`price_xxx`)
   - Team: $99/month → copy Price ID
5. Webhooks → Add endpoint → URL: `http://localhost:8000/v1/webhooks/stripe`
   Events: `customer.subscription.created`, `customer.subscription.updated`,
   `customer.subscription.deleted`, `invoice.paid`, `invoice.payment_failed`
6. Copy webhook signing secret
7. Add to `.env.local`:
   ```
   STRIPE_SECRET_KEY=sk_test_...
   STRIPE_WEBHOOK_SECRET=whsec_...
   STRIPE_PRICE_PRO=price_...
   STRIPE_PRICE_TEAM=price_...
   ```

---

### 🤖 Claude Code Task 16.2 — Billing Endpoints

```
Implement Stripe billing integration in services/api/.

Add stripe==7.0.0 to requirements.txt.

1. services/api/routers/webhooks.py — add Stripe handler:
   POST /v1/webhooks/stripe:
   - Verify with stripe.Webhook.construct_event() + STRIPE_WEBHOOK_SECRET
   - customer.subscription.created/updated:
     UPDATE subscriptions table, UPDATE users.tier accordingly
     (price_pro → PRO, price_team → TEAM, else FREE)
   - customer.subscription.deleted: set status=CANCELLED, revert users.tier to FREE
   - invoice.paid: set subscription status=ACTIVE
   - invoice.payment_failed: set subscription status=PAST_DUE
   - Return 200 always

2. services/api/services/billing_service.py:
   async def get_or_create_stripe_customer(user) -> str
   async def create_checkout_session(user, plan) -> str  (returns Stripe checkout URL)
   async def get_billing_portal_url(user) -> str

3. services/api/routers/billing.py:
   POST /v1/billing/checkout → { checkout_url: str }
   POST /v1/billing/portal → { portal_url: str }
   GET /v1/billing/subscription → current subscription status

NOTE: Stripe stays in TEST MODE. Live mode is enabled on launch day
by swapping one env var: sk_test_... → sk_live_... That is a 30-minute task.
```

---

## Days 17–18 — Simplified Document Signing

🎯 **Goal:** Documents transition DRAFT → REVIEW → SIGNED. Signed documents are locked. Signature is SHA-256 hash (not X.509 — that is deferred to first enterprise inquiry).

---

### 🤖 Claude Code Task 17.1 — Simplified Signing

```
Implement document signing using SHA-256 hash (NOT X.509 PKI — that is deferred).

In crates/spdf-core/src/security/signing.rs:

fn sign_document_simple(
  spdf_bytes: &[u8],
  signer_name: &str,
  signer_email: Option<&str>
) -> Result<Vec<u8>, SpdfError>

Steps:
1. Parse SPDF container
2. Compute document_hash = SHA-256(semantic_json + layout_json + styles_json + metadata_json)
   (concatenate canonical forms, then hash)
3. Create signature JSON:
   {
     "signature_id": "sig-{uuid}",
     "document_id": "...",
     "signer": { "name": signer_name, "email": signer_email },
     "signed_at": "ISO8601 UTC",
     "document_hash": "sha256:...",
     "signature_algorithm": "SHA256_SIMPLE",
     "signature_value": "sha256:{document_hash}",
     "signature_type": "HASH_ONLY",
     "note": "X.509 PKI signing available in enterprise tier"
   }
4. Add signatures/signature_001.json to container
5. Transition document_state from Review to Signed in semantic.json
6. Lock all elements (set locked: true on all)
7. Append STATE_CHANGED entry to audit.json
8. Recompute all manifest checksums
9. Return updated SPDF bytes

fn verify_document_simple(spdf_bytes: &[u8]) -> Result<VerificationReport, SpdfError>
Recomputes document_hash and compares to stored signature_value.
Returns { is_valid, tamper_detected, signed_at, signer_name }

Expose via PyO3 as sign_document and verify_document.

In services/api/routers/documents.py add:
POST /v1/documents/{document_id}/sign
- Document must be in REVIEW state (409 otherwise)
- Call SpdfEngine.sign_document()
- Store updated SPDF bytes back to MinIO
- Update documents.state = SIGNED in PostgreSQL
- Return 200 with signature details and new download_url

POST /v1/documents/{document_id}/verify
- Fetch SPDF from MinIO
- Call SpdfEngine.verify_document()
- Return VerificationReport

POST /v1/documents/{document_id}/transition
- DRAFT → REVIEW and REVIEW → DRAFT transitions only
- REVIEW → SIGNED handled by /sign endpoint only
- 409 if document is SIGNED or CERTIFIED
```

---

## Days 19–20 — Semantic Diff + Validation Endpoint

🎯 **Goal:** Compare two SPDF documents at element level. Validate any SPDF file against the spec.

---

### 🤖 Claude Code Task 19.1 — Diff Engine + Validate Endpoint

```
1. In crates/spdf-core/src/diff/, implement semantic diffing:

fn diff_documents(doc_a: &Document, doc_b: &Document) -> DiffReport

DiffReport: { document_a_id, document_b_id, summary: DiffSummary, changes: Vec<DiffChange> }
DiffSummary: { elements_added, elements_removed, elements_modified, elements_unchanged }
DiffChange: { change_type, eid, element_type, path, field, before, after, semantic_impact }
SemanticImpact enum: FinancialValueChanged, LegalClauseChanged, SignatureBlockChanged,
  MetadataChanged, StructuralChange, ContentChange

Algorithm:
- Build EID→element map for each document
- For each EID in doc_a: if not in doc_b → Removed; if in doc_b → compare all fields → Modified
- For each EID in doc_b not in doc_a → Added

Expose via PyO3: diff_documents(spdf_a: &[u8], spdf_b: &[u8]) -> String (JSON)

2. Add to services/api/routers/documents.py:

GET /v1/documents/{document_id}/diff/{document_id_2}
- Fetch both SPDF files from MinIO
- Parse both to Document structs
- Call diff_documents()
- Return DiffReport as JSON

POST /v1/validate (public — no auth required, 10 calls/IP/day)
- Accept multipart: file (.spdf)
- Run validate_container() + validate_document()
- Return ValidationReport
- This is the developer trust-building endpoint — they can test SDK output without signing up
```

✅ **Week 4 Complete:**
- Signing transitions document to SIGNED state, locks all elements
- Verifying a tampered document returns `tamper_detected: true`
- Diff between two invoice versions returns field-level changes
- Stripe webhook updates user.tier in the database

---

# WEEK 5 — React Studio Frontend

**Theme:** The visual product. Developers and business users can upload, view, and export.

**Cost this week: $0**

---

## Day 21 — React App Scaffold + Clerk Auth

🎯 **Goal:** Studio at `localhost:5173`. Clerk sign-in working. Dashboard shows document list.

---

### 🤖 Claude Code Task 21.1 — Studio Scaffold

```
Create React + Vite + TypeScript application in apps/studio/.

Dependencies: React 18, Vite 5, TypeScript, Tailwind CSS, shadcn/ui,
React Router v6, Zustand, SWR, @clerk/clerk-react, lucide-react, jszip, react-pdf

Directory structure:
apps/studio/src/
├── main.tsx           (entry — ClerkProvider wraps everything)
├── App.tsx            (router)
├── routes/
│   ├── index.tsx      (dashboard — document list)
│   ├── upload.tsx     (generate invoice form + future PDF upload stub)
│   ├── document.tsx   (viewer: PDF left pane + element tree right pane)
│   └── settings.tsx   (API key + billing + usage)
├── components/
│   ├── layout/
│   │   ├── Shell.tsx      (sidebar + main content area)
│   │   └── Sidebar.tsx    (nav links: Dashboard, New Invoice, Settings)
│   ├── documents/
│   │   ├── DocumentCard.tsx
│   │   └── GenerateForm.tsx   (form to fill invoice data + call /generate)
│   ├── viewer/
│   │   ├── SPDFViewer.tsx     (split-pane viewer)
│   │   ├── ElementTree.tsx    (collapsible element tree)
│   │   └── PropertyPanel.tsx  (selected element properties)
│   └── ui/                    (shadcn/ui re-exports)
├── lib/
│   ├── api-client.ts   (typed fetch — uses Clerk JWT for auth)
│   └── auth.ts         (Clerk hooks)
└── stores/
    ├── documentStore.ts
    └── uiStore.ts

Key requirements:
1. All /dashboard/* routes require authentication (<SignedIn> wrapper)
2. Unauthenticated → redirect to /sign-in
3. Dashboard: fetch GET /v1/documents → display as card grid
4. "New Invoice" button → GenerateForm → fills template variables → POST /v1/documents/generate
5. Card click → document viewer route
6. GenerateForm collects: invoice_number, vendor_name, vendor_address, vendor_gstin,
   client_name, client_address, client_gstin, line_items (dynamic add/remove rows),
   currency, issue_date, due_date

vite.config.ts:
- Proxy /api → http://localhost:8000

.env.example:
VITE_CLERK_PUBLISHABLE_KEY=pk_test_...
VITE_API_URL=http://localhost:8000

NOTE: The PDF upload feature (for PDF→SPDF conversion) shows a
"Coming Soon" banner in the UI. The actual endpoint is deferred.
```

---

## Days 22–23 — Document Viewer

🎯 **Goal:** Click a document → see PDF preview on left, clickable element tree on right.

---

### 🤖 Claude Code Task 22.1 — Document Viewer

```
Build the SPDF document viewer in apps/studio/src/routes/document.tsx.

Split-pane layout:
LEFT (60%): PDF preview using react-pdf
RIGHT (40%): Element tree inspector

Data loading:
1. GET /v1/documents/{id} → metadata + download_url
2. POST /v1/documents/{id}/extract?include_element_tree=true → structured data + DOM
3. Fetch download_url → raw .spdf bytes
4. Use JSZip to extract render.pdf from the ZIP bytes → pass to react-pdf

LEFT PANE — react-pdf:
- Renders the PDF layer page by page
- Page navigation: ← 1/2 → controls

RIGHT PANE — element tree:
- Collapsible tree built from extract response element_tree
- Each node: type icon + eid (truncated to 12 chars) + key property preview
- Clicking a node: opens PropertyPanel showing all properties as key-value table
- Financial values: formatted with currency (₹, $, €)
- Timestamps: local timezone
- confidence_score badge: green ≥0.9, yellow ≥0.7, red <0.7
- Low-confidence elements highlighted in yellow in the tree

Element type → icon mapping (use lucide-react):
  Document → FileText, Heading → Heading1/2/3 by level, Paragraph → AlignLeft,
  Table → Table2, InvoiceHeader → Receipt, LineItem → List, SignatureBlock → PenLine,
  Image → ImageIcon

Action toolbar above split pane:
- "Download SPDF" → downloads the .spdf file
- "Download PDF" → calls POST /v1/documents/{id}/render → downloads PDF
- "Sign Document" → transitions to REVIEW then opens sign dialog
- Document state badge: DRAFT/REVIEW/SIGNED/CERTIFIED with colour coding
```

---

## Days 24–25 — WASM Integration + Export Polish

🎯 **Goal:** SPDF validation runs in browser via WASM. Export flow complete.

---

### 🤖 Claude Code Task 24.1 — WASM Integration

```
1. In crates/spdf-wasm/src/lib.rs expose via wasm-bindgen:

   #[wasm_bindgen]
   pub fn validate_spdf_wasm(spdf_bytes: &[u8]) -> String
     Returns ValidationReport as JSON string

   #[wasm_bindgen]
   pub fn get_document_info(spdf_bytes: &[u8]) -> String
     Returns { document_id, title, page_count, state, element_count } as JSON

2. In apps/studio/src/lib/spdf-wasm.ts:
   lazy-load WASM module (import() on first use)
   export async function validateSpdf(bytes: Uint8Array): Promise<ValidationReport>
   export async function getDocumentInfo(bytes: Uint8Array): Promise<DocumentInfo>

3. In the document viewer, after loading SPDF bytes:
   - Run validateSpdf(bytes) in the browser (client-side, no server call)
   - Show a green "Valid SPDF ✓" badge or red "Validation Errors" badge in toolbar

4. In justfile, add:
   build-wasm:
     wasm-pack build crates/spdf-wasm --target web --out-dir apps/studio/src/wasm

5. Settings page (apps/studio/src/routes/settings.tsx):
   - Current API key prefix + "Rotate" button (POST /v1/account/api-key/rotate)
   - Show new key in a one-time reveal dialog with copy button
   - Current plan + usage bars (GET /v1/account/usage)
   - "Upgrade to Pro" button → POST /v1/billing/checkout → redirect to Stripe checkout
```

✅ **Week 5 Complete:**
- Studio loads at localhost:5173, Clerk auth works
- Generate invoice form produces SPDF → appears in dashboard
- Document viewer shows PDF + element tree
- WASM validation badge appears in toolbar
- Export to PDF works

---

# WEEK 6 — Python SDK + TypeScript SDK

**Theme:** The developer-facing products. What gets GitHub stars and PyPI downloads.

**Cost this week: $0**

---

## Days 26–27 — Python SDK

🎯 **Goal:** `pip install spdf-sdk && python getting_started.py` generates a valid invoice in under 25 lines.

---

### 🤖 Claude Code Task 26.1 — Python SDK

```
Create the Python SDK in packages/spdf-python/.

This is a developer-facing pip package. It wraps PyO3 bindings with clean Python API.

packages/spdf-python/
├── spdf/
│   ├── __init__.py         (exports: Document, Page, elements, signing)
│   ├── document.py         (Document class)
│   ├── page.py             (Page class)
│   ├── template.py         (Template loading)
│   ├── signing.py          (sign and verify)
│   ├── extraction.py       (extract structured data)
│   ├── exceptions.py       (SpdfError hierarchy)
│   └── elements/
│       ├── __init__.py
│       ├── content.py      (Heading, Paragraph, Table, Image)
│       ├── domain.py       (InvoiceHeader, LineItem, LineItemTable, PaymentTerms)
│       └── trust.py        (SignatureBlock, Stamp)
├── tests/
│   ├── test_generation.py
│   └── test_extraction.py
├── pyproject.toml
└── README.md

The API must work exactly like this (matching the PRD):

  from spdf import Document, Page
  from spdf.elements.domain import InvoiceHeader, LineItemTable, PaymentTerms

  doc = Document(
      title="Invoice #INV-2025-001",
      locale="en-IN",
      document_type="Invoice"
  )
  page = Page(size="A4")
  page.add(InvoiceHeader(
      invoice_number="INV-2025-001",
      issue_date="2025-03-15",
      due_date="2025-04-14",
      vendor={"name": "ACME Corp", "address": "Pune", "gstin": "27AABCA1234F1Z5"},
      client={"name": "GlobalTech", "address": "Bengaluru", "gstin": "29AACG5678H1ZQ"}
  ))
  page.add(LineItemTable(
      currency="INR",
      tax_scheme="GST_18",
      items=[
          {"description": "Backend Dev", "qty": 80, "unit": "hours", "unit_price": "2500.00"},
          {"description": "DevOps Setup", "qty": 40, "unit": "hours", "unit_price": "2000.00"},
      ]
  ))
  page.add(PaymentTerms(terms="Net 30", due_date="2025-04-14"))
  doc.add_page(page)
  doc.export.spdf("invoice.spdf")
  doc.export.pdf("invoice.pdf")
  doc.export.json("invoice.json")
  print("Done!")

Financial values accept: str ("2500.00"), int (2500), or float (2500.0)
Internally always stored as decimal strings.

README.md must show this 20-line example as the first code block.
Write examples/getting_started.py — a runnable demo that produces invoice.spdf and invoice.pdf.
Write tests that verify the round-trip: generate → read back → assert field values.
```

---

## Days 28–29 — TypeScript SDK

🎯 **Goal:** `npm install @spdf/sdk && node getting_started.js` generates a valid invoice.

---

### 🤖 Claude Code Task 28.1 — TypeScript SDK

```
Create the TypeScript SDK in packages/spdf-js/.

Package name: @spdf/sdk

packages/spdf-js/
├── src/
│   ├── index.ts            (public exports)
│   ├── document.ts         (Document class — all methods return Promise)
│   ├── page.ts             (Page class)
│   ├── elements/
│   │   ├── index.ts
│   │   ├── content.ts      (Heading, Paragraph, Table, Image)
│   │   ├── domain.ts       (InvoiceHeader, LineItem, LineItemTable, PaymentTerms)
│   │   └── trust.ts        (SignatureBlock)
│   ├── wasm-loader.ts      (lazy WASM init)
│   ├── node-adapter.ts     (Node.js: fs, Buffer)
│   └── browser-adapter.ts  (Browser: Uint8Array, Blob, download)
├── tests/
│   ├── generation.test.ts
│   └── extraction.test.ts
├── package.json
├── tsconfig.json
└── README.md

Usage must match exactly:

  import { Document, Page, InvoiceHeader, LineItemTable } from "@spdf/sdk";

  const doc = await Document.create({ title: "Invoice #001", locale: "en-IN" });
  const page = new Page({ size: "A4" });
  page.add(new InvoiceHeader({
      invoiceNumber: "INV-001",
      issueDate: "2025-03-15",
      dueDate: "2025-04-14",
      vendor: { name: "ACME Corp", gstin: "27AABCA1234F1Z5" },
      client: { name: "GlobalTech", gstin: "29AACG5678H1ZQ" }
  }));
  page.add(new LineItemTable({
      currency: "INR",
      items: [
          { description: "Backend Dev", qty: 80, unitPrice: "2500.00" },
          { description: "DevOps", qty: 40, unitPrice: "2000.00" }
      ]
  }));
  doc.addPage(page);

  // Node.js
  const bytes = await doc.export.spdf();
  require("fs").writeFileSync("invoice.spdf", bytes);

  // Browser
  const blob = await doc.export.blob("spdf");
  const url = URL.createObjectURL(blob);

100% TypeScript types — no `any` in the public API.
Write examples/getting_started.js as a runnable Node.js script.
```

---

## Day 30 — SDK Cross-Compatibility Test

🎯 **Goal:** Python SDK and TypeScript SDK produce byte-compatible SPDF files.

### 💻 Run on TUF F15

```bash
# Python SDK
cd packages/spdf-python && pip install -e .
python examples/getting_started.py
# Must produce: invoice.spdf, invoice.pdf

# TypeScript SDK
cd packages/spdf-js && npm install && npm run build
node examples/getting_started.js
# Must produce: invoice.spdf

# Cross-SDK validation
curl -X POST http://localhost:8000/v1/validate \
  -F "file=@packages/spdf-python/invoice.spdf"
# Must return is_valid: true

curl -X POST http://localhost:8000/v1/validate \
  -F "file=@packages/spdf-js/invoice.spdf"
# Must return is_valid: true
```

✅ **Week 6 Complete:** Both SDKs installed from source. Both generate valid SPDF. Cross-validated against the API.

---

# WEEK 7 — Testing + Security Hardening

**Theme:** Nothing ships without tests. Security audit before public code.

**Cost this week: $0**

---

## Days 31–33 — Comprehensive Test Suite

### 🤖 Claude Code Task 31.1 — Full Test Suite

```
Create comprehensive tests across the entire platform.

1. crates/spdf-core/tests/integration_test.rs (expand existing):
   - test_complete_invoice_round_trip (already done — verify still passes)
   - test_signing_and_verification: sign doc → verify → tamper 1 byte → verify → assert tamper_detected
   - test_cryptographic_diff: create two invoice versions, diff, assert exact field changes detected
   - test_validation_error_codes: trigger every E_ error code, assert correct code in report
   - test_forward_compatibility: parse SPDF with unknown element type "x-test:FutureElement",
     assert it's preserved as opaque node, round-trip produces identical bytes
   - test_audit_chain: verify chain → modify entry_hash → verify → assert chain broken detected
   - test_financial_values_never_float: all decimal fields in generated SPDF are JSON strings, never numbers
   - test_document_state_machine: assert invalid transitions (Signed→Draft) return error

2. services/api/tests/test_documents.py:
   - test_generate_invoice_success: POST /generate → 201, valid SPDF in response
   - test_generate_missing_variable: omit INVOICE_NUMBER → 422 TEMPLATE_VARIABLE_MISSING
   - test_generate_download: generate → download from URL → validate as SPDF
   - test_extract_invoice_data: generate → extract → assert invoice_number, vendor.name, line_items
   - test_delete_draft: generate → delete → 204
   - test_delete_signed_fails: generate → sign → delete → 409

3. services/api/tests/test_auth.py:
   - test_no_auth_header: GET /documents → 401 AUTH_REQUIRED
   - test_invalid_key: GET /documents with "Bearer wrong_key" → 401 INVALID_API_KEY
   - test_valid_key: rotate API key → use new key → 200
   - test_old_key_fails: after rotation → old key → 401

4. services/api/tests/test_rate_limiting.py:
   - test_limit_enforced: set user tier=FREE, make 51 generate calls → 52nd returns 429
   - test_headers_present: every response has X-RateLimit-Limit, Remaining, Reset
   - test_pro_higher_limit: set tier=PRO → make 100 calls → all succeed

5. apps/studio/tests/:
   - DocumentCard.test.tsx: renders with correct state badge colour
   - GenerateForm.test.tsx: form submission calls POST /generate with correct body
   - ElementTree.test.tsx: tree renders, click selects node, property panel shows fields
```

---

## Days 34–35 — Security Hardening

### 🤖 Claude Code Task 34.1 — Security Hardening

```
Apply security hardening to services/api/.

1. Input validation middleware:
   - Max file upload: 50MB (check Content-Length before reading)
   - Max JSON body: 1MB
   - PDF magic bytes validation: first 4 bytes must be %PDF (25 50 44 46)
   - Template variable sanitisation: strip HTML tags, limit strings to 10,000 chars

2. SSRF prevention for future webhook_url field:
   Create validate_webhook_url(url: str) -> bool
   Block: 127.0.0.0/8, 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 169.254.0.0/16
   Only allow https:// scheme — block http://
   Raise 400 INVALID_FIELD_VALUE if URL fails validation

3. Security response headers middleware:
   X-Content-Type-Options: nosniff
   X-Frame-Options: DENY
   X-XSS-Protection: 1; mode=block
   Referrer-Policy: strict-origin-when-cross-origin

4. SQL injection audit:
   Review ALL database queries — 100% must use SQLAlchemy ORM or parameterised queries
   Zero raw SQL strings containing user input

5. Sensitive field redaction in logs:
   In logging middleware, auto-redact any log field whose key contains:
   "key", "secret", "token", "password", "hash", "authorization"
   Replace value with "[REDACTED]"

6. API key rotation rate limit:
   POST /v1/account/api-key/rotate: max 5 per hour per user
   Enforced via Redis: key "ratelimit:{user_id}:key_rotation:{YYYY-MM-DD-HH-UTC}"

7. Dependency security:
   Create scripts/security_check.sh:
     pip-audit --requirement services/api/requirements.txt
     cargo audit
     npm audit --prefix apps/studio
   Run it and fix any HIGH or CRITICAL findings.
```

✅ **Week 7 Complete:**
- `cargo test --workspace` — all tests pass
- `pytest services/api/tests/ -v` — all tests pass
- `npm test` in studio — all tests pass
- `bash scripts/security_check.sh` — zero HIGH/CRITICAL findings

---

# WEEK 8 — CI/CD Pipeline + Templates Library

**Theme:** Automate the build. Expand the template library so launch has more than one template.

**Cost this week: $0**

---

## Days 36–37 — GitHub Actions CI Pipeline

### 🤖 Claude Code Task 36.1 — CI Pipeline

```
Create GitHub Actions workflows in .github/workflows/.

1. .github/workflows/ci.yml — runs on every PR and push to main:

   Jobs (run in parallel where possible):

   rust-core:
   - ubuntu-latest
   - Install Rust stable + rust-cache (Swatinem/rust-cache@v2)
   - cargo test --workspace
   - cargo clippy -- -D warnings
   - cargo audit
   - wasm-pack build crates/spdf-wasm --target web
   - maturin build --release in crates/spdf-python
   - Upload artifacts: WASM package + PyO3 wheel

   python-backend:
   - ubuntu-latest, needs: rust-core
   - services: postgres:15 (env: POSTGRES_PASSWORD=test), redis:7
   - Download rust artifacts
   - Python 3.12, pip install requirements.txt
   - ruff check services/ packages/spdf-python/
   - pytest services/api/tests/ --cov=services/api --cov-report=xml
   - Upload coverage to Codecov

   frontend:
   - ubuntu-latest, needs: rust-core
   - Download WASM artifact → apps/studio/src/wasm/
   - Node 20, npm ci in apps/studio
   - npm run lint, npm run type-check, npm test, npm run build

   security:
   - ubuntu-latest (runs separately, not blocking deploy)
   - pip-audit, cargo audit, npm audit

2. .github/workflows/release.yml — runs on tag push v*:
   - Build Python SDK wheel (maturin build --release)
   - Publish to PyPI using PYPI_TOKEN secret
   - Build TypeScript SDK (npm run build)
   - Publish to npm using NPM_TOKEN secret
   - Create GitHub Release with auto-generated changelog

NOTE: No deploy job yet. Railway deploy is added in Week 11.
Add secrets to GHE: PYPI_TOKEN (create at pypi.org), NPM_TOKEN (create at npmjs.com)
```

---

## Days 38–40 — Template Library Expansion

🎯 **Goal:** 8 templates ready for launch. More templates = more developer appeal on launch day.

---

### 🤖 Claude Code Task 38.1 — Extended Template Library

```
Expand scripts/seed_db.py to insert 8 default templates.

For each template, define a complete variable_schema with required and optional fields.

Templates to create:

1. tmpl_invoice_gst_india — "GST Invoice (India)" (already exists, keep)
   Required: INVOICE_NUMBER, ISSUE_DATE, DUE_DATE, VENDOR_*, CLIENT_*, LINE_ITEMS, CURRENCY
   
2. tmpl_invoice_simple — "Simple Invoice"
   Same as India but without GST fields. Universal format.

3. tmpl_invoice_us — "US Invoice"
   Adds: STATE, ZIP_CODE, SALES_TAX_RATE. Removes: GSTIN fields.

4. tmpl_invoice_eu — "EU Invoice (VAT)"
   Adds: VAT_NUMBER, VAT_RATE, IBAN, BIC/SWIFT. Removes: GSTIN.

5. tmpl_purchase_order — "Purchase Order"
   Required: PO_NUMBER, PO_DATE, DELIVERY_DATE, BUYER_*, SUPPLIER_*, LINE_ITEMS
   Different from invoice: has DELIVERY_ADDRESS, TERMS_AND_CONDITIONS

6. tmpl_credit_note — "Credit Note"
   Required: CREDIT_NOTE_NUMBER, ORIGINAL_INVOICE_NUMBER, ISSUE_DATE, REASON, AMOUNT

7. tmpl_proforma_invoice — "Proforma Invoice"
   Same as invoice but with VALIDITY_DATE and "NOT A TAX INVOICE" watermark note

8. tmpl_statement_of_account — "Statement of Account"
   Required: STATEMENT_DATE, ACCOUNT_NUMBER, PERIOD_FROM, PERIOD_TO,
   TRANSACTIONS (array: {date, description, debit, credit, balance}), CLOSING_BALANCE

Also expand the GenerateForm in Studio to show a template selector
dropdown at the top, dynamically rendering the correct fields for each template.
```

✅ **Week 8 Complete:**
- CI pipeline runs on every push to main
- All jobs pass on first run
- 8 templates in the database
- Template selector works in Studio

---

# WEEKS 9–10 — GTM Content Creation

**Theme:** The freed-up weeks from deferred features go entirely into GTM content. Content is written before a single line of launch code runs. This is the most important investment before going public.

**Cost these weeks: $0**

**Why content first:** The spec repo goes public in Week 11. When it does, every piece of content needs to already exist — the Hacker News post, the technical article, the documentation, the landing page. Writing them under pressure the day of launch produces mediocre content. Writing them over two weeks produces content that earns links and users.

---

## Week 9 — Documentation + Spec Publication Prep

---

## Days 41–42 — Documentation Site

### 🤖 Claude Code Task 41.1 — Mintlify Documentation

```
Set up SPDF documentation in docs/ using Mintlify.

docs/
├── mint.json
├── introduction.mdx
├── quickstart.mdx
├── concepts/
│   ├── format.mdx        (what SPDF is, the dual-layer architecture)
│   ├── dom.mdx           (Document Object Model, element types)
│   └── security.mdx      (simplified signing, deferred X.509 note)
├── sdk/
│   ├── python.mdx        (Python SDK full reference)
│   └── javascript.mdx    (TypeScript SDK full reference)
├── api-reference/        (auto-generated from OpenAPI)
└── guides/
    ├── first-invoice.mdx       (Generate your first invoice — 5 minutes)
    ├── extract-data.mdx        (Extract structured data from SPDF)
    ├── validate-spdf.mdx       (Validate any SPDF file)
    ├── compare-documents.mdx   (Semantic diff between two documents)
    └── studio-walkthrough.mdx  (Studio UI walkthrough)

quickstart.mdx must show the complete 20-line Python example as the FIRST code block.
Time to first working invoice: under 5 minutes from opening the docs.

Every guide must have:
1. A one-sentence goal statement
2. A working code example that runs without modification
3. Expected output shown below the code block
4. A "What's next" section linking to related guides

mint.json: name "SPDF Docs", dark theme, blue accent, all sections in navigation.
```

---

## Days 43–44 — Spec Repository Preparation

### 🤖 Claude Code Task 43.1 — Spec Repo README

```
Prepare the public SPDF format specification repository.

Create spec/README.md — this becomes the GitHub landing page for the spec repo.
It must be excellent because it is the first thing a developer sees.

Structure:
1. Badge row: License CC BY 4.0, Version 1.0, Status: Draft for Review
2. One-line description: "An open, structured document format designed to replace PDF."
3. The problem in 3 sentences (PDF is a photograph, not data)
4. The solution in 1 sentence (dual-layer: render for viewers, semantic for machines)
5. Core properties table: Visual fidelity, Semantic structure, Backward compat, AI-native, Cryptographic integrity
6. Quick example — 8-line Python invoice generation (no imports, just the core code)
7. "Why now" — AI makes unreadable documents the bottleneck (2-3 sentences)
8. What's in this repo: SPEC.md (the full specification), CONTRIBUTING.md
9. Implementations: link to spdf-core (Rust), spdf-python SDK, spdf-js SDK
10. License: CC BY 4.0 for spec, MIT for implementations
11. Contributing section

Also create spec/CONTRIBUTING.md:
- How to report errata (open an issue with label "errata")
- How to propose new element types (issue template with required fields)
- How to propose extensions (namespace registration process)
- Security vulnerability reporting: email, not public issues
- Code of conduct reference

The spec/SPEC.md file already exists (copy from SPDF-FORMAT-2025-001).
The README wraps and introduces it.
```

---

## Day 45 — Landing Page

### 🤖 Claude Code Task 45.1 — spdf.dev Landing Page

```
Create apps/landing/ — a standalone HTML/CSS landing page.

Pure HTML, CSS, minimal vanilla JS. No framework. Target: < 50KB total, < 1s load time.

Sections:
1. NAV: Logo "SPDF" + links: Spec, Docs, Studio, GitHub

2. HERO:
   Headline (large): "The document format for the next 30 years"
   Subheadline: "PDF was designed in 1993 for a world that no longer exists. SPDF keeps every promise PDF made — and adds everything AI, automation, and developers need today."
   Two CTAs: [Read the Spec →] and [Try the Studio →]
   Below CTAs: a minimal code block showing the 8-line Python invoice example

3. PROBLEM (dark background):
   Title: "PDF is a photograph. Not data."
   Three columns:
   - "AI can't read it" — enterprise paying $2–5B/year on PDF parsing pipelines
   - "Developers hate it" — 200+ lines of painful reportlab code for a mediocre invoice
   - "Enterprises can't extract it" — 57% of invoices still manually re-entered

4. SOLUTION:
   Title: "Same face. Completely new inside."
   Two-column diagram (HTML/CSS, not image):
   Left: "What PDF viewers see" → render.pdf → "Opens identically everywhere"
   Right: "What systems see" → semantic.json → "Every element typed and queryable"

5. FEATURES grid (6 cards, 2x3):
   Visual fidelity, Semantic structure, AI-native, Backward compatible,
   Developer SDK, Cryptographic integrity

6. CODE TABS:
   Three tabs: Python SDK | TypeScript SDK | REST API
   Each showing a complete invoice generation example

7. USE CASES (3 cards):
   B2B Invoices, Legal Contracts, Government Certificates

8. OPEN SOURCE section:
   "The spec is free. Always. The engine is open source. Always."
   Link to spec repo and spdf-core repo

9. FOOTER: Spec | Docs | Studio | GitHub | License (CC BY 4.0)

Design: dark (#0D1117 background), white text, blue accent (#2563EB).
Inter font from Google Fonts. Mobile responsive. No animations needed.
```

---

## Days 46–47 — Launch Content Writing

### 🤖 Claude Code Task 46.1 — All Launch Content

```
Write all launch content pieces. These are used on launch day in Week 12.

1. LAUNCH/hacker_news_post.md — Show HN post:
   Title: "Show HN: SPDF – An open structured replacement for PDF"
   Body (400–500 words):
   - Open with the specific problem you experienced that led to building this
   - The technical insight (dual-layer: ZIP containing render.pdf + semantic.json)
   - Why Rust for the core engine (parser security, WASM compilation, PyO3 bindings)
   - What is open (spec CC BY 4.0, core engine MIT) vs what is the business (the platform)
   - Current state: what works today, what is deferred and why (be honest about X.509 and PDF conversion)
   - What you are asking for: try the Python SDK, open issues, star the spec repo
   Tone: technical, direct, honest. No hype. Hackers respect candour.

2. LAUNCH/product_hunt.md:
   Tagline (60 chars max): "PDF was designed in 1993. SPDF is designed for today."
   Description (260 chars): focus on the developer pain and the SDK solution
   Maker comment (400 words): personal story + technical choices + what you are most proud of

3. LAUNCH/twitter_thread.md — 8 tweets:
   Tweet 1: hook — the 1993 problem
   Tweets 2–3: the cost (invoice processing, AI parsing spend)
   Tweet 4: the architectural insight in plain English
   Tweet 5: the code (Python SDK 8 lines)
   Tweet 6: what is open vs commercial
   Tweet 7: deferred features and why (shows thoughtful engineering)
   Tweet 8: CTA — spec repo link + Studio link

4. LAUNCH/devto_article.md — full technical article (1,500 words):
   Title: "I got tired of PDF so I built a structured replacement"
   Sections: The Problem, The Insight, The Architecture, The Rust Core,
   The ZIP Container Trick, Why JSON for the Semantic Layer, What Works Today,
   What Is Coming, How to Try It
   Include: the 20-line Python invoice example with output

5. LAUNCH/linkedin_post.md — professional audience version (300 words):
   Focus on the enterprise cost angle ($2–5B PDF parsing infrastructure)
   and the business case for structured documents

6. LAUNCH/readme_update.md — updated GHE private repo README:
   Full architecture overview, local setup instructions, contributing guide
```

---

## Days 48–50 — Smoke Test Script + Pre-Launch Checklist

### 🤖 Claude Code Task 48.1 — Smoke Test

```
Create scripts/smoke_test.py — production smoke test run after every deployment.

Tests the LIVE production API (URL from --api-url argument):

1. Health check: GET /v1/health → status: "ok"
2. Auth rejected: GET /v1/documents without auth → 401 AUTH_REQUIRED
3. Auth accepted: GET /v1/documents with valid key → 200
4. Rate limit headers: verify X-RateLimit-* on every response
5. Generate invoice: POST /v1/documents/generate with test data → 201
6. Download SPDF: fetch download_url → verify ZIP magic bytes (PK\x03\x04)
7. Extract data: POST /v1/documents/{id}/extract → verify invoice_number field
8. Render PDF: POST /v1/documents/{id}/render → verify %PDF magic bytes
9. Validate SPDF: POST /v1/validate with downloaded file → is_valid: true
10. Diff same document with itself: GET /v1/documents/{id}/diff/{id} → 0 changes
11. Delete: DELETE /v1/documents/{id} → 204

Usage: python scripts/smoke_test.py --api-url https://api.spdf.dev --api-key sk_live_...
Exit code 0 if all pass, 1 if any fail.
Print: PASS/FAIL for each test + total runtime.
Target: all 11 tests in under 30 seconds.
```

### 🤖 Claude Code Task 49.1 — Performance Benchmarks

```
Create performance benchmarks in crates/spdf-core/benches/benchmarks.rs using criterion.

bench_parse_spdf: parse 50-page SPDF 100 times → target p95 < 200ms
bench_generate_invoice: generate 2-page invoice 1000 times → target p95 < 500ms
bench_render_pdf: render 2-page doc to PDF 100 times → target p95 < 1000ms
bench_validate_document: validate complex document 1000 times → target p95 < 50ms
bench_diff_documents: diff two 5-page invoices 100 times → target p95 < 500ms

Run and record results in BENCHMARKS.md.
If any benchmark misses its target by more than 2x, that is a P1 bug.
```

✅ **Weeks 9–10 Complete:**
- Docs site builds and renders all guides
- All 6 launch content pieces written and reviewed
- Spec README is compelling and complete
- Landing page loads in < 1 second
- Smoke test script passes against local API (localhost:8000)
- All benchmarks within target

---

# WEEK 11 — Deploy + Spec Goes Public

**Theme:** First real money spent. Spec repo goes public. Railway deploys. Vercel deploys. Everything pointed at real cloud infrastructure.

**Cost this week: $15–25 (Railway + domain)**

**Trigger check before starting Week 11:** Has the spec repo gotten any traffic or GitHub stars? If yes, proceed. If the spec is not yet public, publish it on Day 51 first, then deploy after 24 hours of confirming it resolves correctly.

---

## Day 51 — Create All Remaining Cloud Accounts

🎯 **Goal:** All cloud services configured in one focused session.

⏱ **3 hours — enterprise desktop browser**

---

### 🖐 Manual Action 51.1 — Create All Remaining Accounts

Work through each service in order. Collect all credentials in a temporary private document. Move to `.env.production` file at end of session. Delete temporary document immediately.

**1. Supabase (15 min)**
1. [supabase.com](https://supabase.com) → Sign up with GitHub
2. New project → name: `spdf-production` → strong DB password → region: Asia (Mumbai) or nearest
3. Wait ~2 min → Settings → API → copy Project URL, anon key, service_role key
4. Settings → Database → copy Connection string (URI format, pooler preferred)

**2. Upstash Redis (10 min)**
1. [upstash.com](https://upstash.com) → Sign up
2. Create database → name: `spdf-production` → region: nearest → Regional type
3. Copy full `REDIS_URL` (starts with `rediss://`)

**3. Cloudflare R2 (15 min)**
1. Already have Cloudflare account (created Day 4)
2. R2 Object Storage → Create bucket: `spdf-documents`
3. Create bucket: `spdf-uploads`
4. R2 → Manage R2 API Tokens → Create:
   - Permissions: Object Read & Write on both buckets
   - Copy `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY`, `R2_ACCOUNT_ID`
5. R2_ENDPOINT = `https://YOUR_ACCOUNT_ID.r2.cloudflarestorage.com`

**4. Railway (10 min)**
1. [railway.app](https://railway.app) → Sign up with GitHub
2. New Project → name: `spdf-platform`
3. Settings → Tokens → Create → name: `spdf-deploy` → copy `RAILWAY_TOKEN`
4. Add to GHE secrets: `RAILWAY_TOKEN`

**5. Vercel (5 min)**
1. [vercel.com](https://vercel.com) → Sign up with GitHub
2. No setup needed yet — just create account

**6. Sentry (5 min)**
1. [sentry.io](https://sentry.io) → Sign up (free tier)
2. Create project → Platform: FastAPI → name: `spdf-api`
3. Copy DSN URL

**7. Resend (5 min)**
1. [resend.com](https://resend.com) → Sign up
2. API Keys → Create → name: `spdf-production` → copy key

**8. Doppler (15 min)**
1. [doppler.com](https://doppler.com) → Sign up (free for individuals)
2. Create project → name: `spdf`
3. Click `prd` environment → add ALL secrets:
   ```
   DATABASE_URL          = (Supabase connection string)
   REDIS_URL             = (Upstash Redis URL)
   R2_ENDPOINT           = https://ACCOUNT_ID.r2.cloudflarestorage.com
   R2_ACCESS_KEY_ID      = (R2 access key)
   R2_SECRET_ACCESS_KEY  = (R2 secret key)
   R2_BUCKET_DOCUMENTS   = spdf-documents
   R2_BUCKET_UPLOADS     = spdf-uploads
   ANTHROPIC_API_KEY     = (from Day 4 — not used yet, but stored)
   CLERK_SECRET_KEY      = (from Week 3)
   CLERK_WEBHOOK_SECRET  = (from Week 3)
   STRIPE_SECRET_KEY     = sk_test_... (NOT live yet — switched on launch day)
   STRIPE_WEBHOOK_SECRET = (from Week 4)
   STRIPE_PRICE_PRO      = (from Week 4)
   STRIPE_PRICE_TEAM     = (from Week 4)
   RESEND_API_KEY        = (from today)
   SENTRY_DSN            = (from today)
   ENVIRONMENT           = production
   LOG_LEVEL             = info
   ```
4. Install Doppler CLI on TUF F15 if not already: `sudo apt install doppler`
5. `doppler login && doppler setup` (select project: spdf, config: prd)

⚠️ **Delete your temporary credentials document now.**

---

## Day 52 — Run Production Migrations + Deploy to Railway

### 🖐 Manual Action 52.1 — Deploy API and Worker to Railway

**On TUF F15 (WSL):**

```bash
# Install Railway CLI
npm install -g @railway/cli
railway login

# Link to Railway project
cd ~/projects/spdf
railway link   # select spdf-platform project
```

**Configure Railway services via the Railway dashboard (browser):**

**API Service:**
1. Railway project → New Service → GitHub Repo → select your `spdf` repo
2. Root directory: `services/api`
3. Build command: `pip install -r requirements.txt`
4. Start command: `sh -c 'alembic upgrade head && uvicorn main:app --host 0.0.0.0 --port $PORT'`
5. Health check path: `/v1/health`
6. Variables: click "Connect Doppler" → select project: spdf, config: prd
   (This syncs all Doppler secrets to Railway automatically)
7. Domains → Add custom domain: `api.spdf.dev`

**Worker Service:**
1. New Service → same repo
2. Root directory: `services/worker`
3. Start command: `celery -A celery_app worker --loglevel=info --concurrency=2`
4. Same Doppler sync
5. No custom domain needed

**Deploy:**
```bash
railway up --service spdf-api
railway up --service spdf-worker

# Verify
curl https://api.spdf.dev/v1/health
# Must return: {"status": "ok", ...}
```

**Update Clerk webhook URL:**
1. Clerk dashboard → Webhooks → edit your webhook endpoint
2. Change URL from `http://localhost:8000/...` to `https://api.spdf.dev/v1/webhooks/clerk`

**Update Stripe webhook URL:**
1. Stripe dashboard → Developers → Webhooks → edit endpoint
2. Change URL to `https://api.spdf.dev/v1/webhooks/stripe`

---

## Day 53 — Deploy Studio to Vercel + DNS Configuration

### 🖐 Manual Action 53.1 — Vercel Deployment

1. [vercel.com](https://vercel.com) → New Project → Import from GitHub
2. Select `spdf` repo → Root directory: `apps/studio`
3. Framework: Vite · Build: `npm run build` · Output: `dist`
4. Environment variables:
   - `VITE_CLERK_PUBLISHABLE_KEY` = your Clerk publishable key
   - `VITE_API_URL` = `https://api.spdf.dev`
5. Add custom domain: `studio.spdf.dev`
6. Deploy → wait ~2 minutes → visit `https://studio.spdf.dev`

### 🖐 Manual Action 53.2 — DNS Configuration

In Cloudflare DNS (your registered domain `spdf.dev`):

| Type | Name | Value | Proxy |
|---|---|---|---|
| CNAME | api | your-railway-domain.railway.app | ✅ Proxied |
| CNAME | studio | cname.vercel-dns.com | ✅ Proxied |
| CNAME | docs | YOUR_MINTLIFY_DOMAIN | ✅ Proxied |
| A | @ | 76.76.21.21 (Vercel landing page) | ✅ Proxied |

Deploy docs to Mintlify:
1. [mintlify.com](https://mintlify.com) → connect your GHE repo → select `docs/` directory
2. Custom domain: `docs.spdf.dev`

---

## Day 54 — Publish Spec Repo + Add Sentry

### 🖐 Manual Action 54.1 — Make Spec Repo Public

1. On GHE → create **new public repository**: `spdf-spec`
2. Copy `spec/SPEC.md` → rename to `SPEC.md` in the new repo
3. Copy `spec/README.md` as the repo README
4. Copy `spec/CONTRIBUTING.md`
5. Add `LICENSE` file with CC BY 4.0 text
6. Make repo public: Settings → Danger Zone → Change visibility → Public
7. Add repo description: "The open specification for SPDF — a structured replacement for PDF"
8. Add topics: `pdf`, `document-format`, `open-standard`, `spdf`, `rust`, `api`

### 🤖 Claude Code Task 54.2 — Add Sentry to Production

```
Add Sentry error tracking to services/api/main.py and services/worker/celery_app.py.

In main.py startup:
  import sentry_sdk
  from sentry_sdk.integrations.fastapi import FastApiIntegration
  from sentry_sdk.integrations.sqlalchemy import SqlalchemyIntegration

  if config.sentry_dsn:
      sentry_sdk.init(
          dsn=config.sentry_dsn,
          integrations=[FastApiIntegration(), SqlalchemyIntegration()],
          traces_sample_rate=0.1,
          environment=config.environment
      )

In celery_app.py:
  if SENTRY_DSN:
      import sentry_sdk
      sentry_sdk.init(dsn=SENTRY_DSN, environment=ENVIRONMENT)

Add to requirements.txt: sentry-sdk[fastapi]==1.40.0
```

---

## Day 55 — Production Smoke Test + Stripe Live Mode Switch

### 💻 Run on TUF F15

```bash
# Run full smoke test against production
cd ~/projects/spdf
python scripts/smoke_test.py \
  --api-url https://api.spdf.dev \
  --api-key YOUR_PRODUCTION_API_KEY

# All 11 tests must pass before proceeding
```

### 🖐 Manual Action 55.1 — Switch Stripe to Live Mode

**This is the moment SPDF becomes a commercial product.**

1. Stripe dashboard → switch from Test Mode to **Live Mode** (top left toggle)
2. In Live Mode: Developers → API Keys → copy `sk_live_...` key
3. Create the same products and pricing in Live Mode:
   - Pro: $29/month → copy live Price ID
   - Team: $99/month → copy live Price ID
4. Webhooks → Add endpoint → URL: `https://api.spdf.dev/v1/webhooks/stripe`
   Copy live webhook signing secret
5. Update Doppler `prd` environment:
   ```
   STRIPE_SECRET_KEY = sk_live_...    (was sk_test_...)
   STRIPE_WEBHOOK_SECRET = whsec_live_...
   STRIPE_PRICE_PRO = price_live_...
   STRIPE_PRICE_TEAM = price_live_...
   ```
6. Railway automatically picks up the Doppler change (no redeploy needed if using Doppler sync)
   Otherwise: `railway up --service spdf-api`

✅ **Week 11 Complete:**
- `https://api.spdf.dev/v1/health` → `{"status": "ok"}`
- `https://studio.spdf.dev` → loads, Clerk auth works
- `https://docs.spdf.dev` → documentation site live
- `https://spdf.dev` → landing page loads < 1s
- Spec repo is public on GHE
- `python scripts/smoke_test.py` → all 11 tests pass
- Stripe is in LIVE mode

---

# WEEK 12 — Launch Week

**Theme:** Distribution. The code is done. This week is entirely about getting the product in front of developers.

**Cost this week: $15–25 (same as Week 11 — no new costs)**

---

## Day 56 — Final Pre-Launch Checks

### 🖐 Manual Action 56.1 — Complete Pre-Launch Checklist

Work through every item. Do not skip any. Mark each one explicitly.

**Production:**
- [ ] Smoke test: all 11 tests pass (`python scripts/smoke_test.py`)
- [ ] Studio: sign up with a fresh email, generate an invoice, download SPDF and PDF
- [ ] Stripe: complete a test purchase with card `4242 4242 4242 4242` → tier upgrades to Pro
- [ ] API key rotation: rotate → use new key → old key rejected
- [ ] Rate limiting: exceed free tier → get 429 with correct headers
- [ ] Sentry: trigger a test error → verify it appears in Sentry dashboard
- [ ] Railway: both API and Worker services show green health status
- [ ] Uptime: verify `api.spdf.dev`, `studio.spdf.dev`, `docs.spdf.dev` all load

**SDKs:**
- [ ] `pip install spdf-sdk` from PyPI → works (published via CI on last tag)
- [ ] `npm install @spdf/sdk` from npm → works
- [ ] Both generate valid SPDF per getting_started scripts

**Content:**
- [ ] HN post text is written and reviewed (in `LAUNCH/hacker_news_post.md`)
- [ ] Product Hunt listing is drafted and submitted for review (submit 3 days before)
- [ ] Dev.to article is saved as draft, ready to publish
- [ ] Twitter thread is written and ready to post
- [ ] LinkedIn post is written and ready

**Spec repo:**
- [ ] `github.com/YOUR_USERNAME/spdf-spec` is public
- [ ] README renders correctly on GitHub
- [ ] Issues are enabled
- [ ] CONTRIBUTING.md is clear

---

## Day 57 — Launch Day

### 🖐 Manual Action 57.1 — Launch Submissions (Time-Sequenced)

**8:00 AM: Submit to Hacker News**
1. Go to [news.ycombinator.com/submit](https://news.ycombinator.com/submit)
2. Title: `Show HN: SPDF – An open structured replacement for PDF`
3. URL: your public spec repo URL on GHE
4. Submit
5. **Immediately** post the content from `LAUNCH/hacker_news_post.md` as a comment on your own post
6. Keep the HN tab open all day — respond to every comment within 30 minutes

**8:30 AM: Publish Dev.to article**
1. Go to dev.to → your draft → Publish
2. The article links to the spec repo and the Studio

**9:00 AM: Post Twitter/X thread**
1. Post the thread from `LAUNCH/twitter_thread.md`
2. First tweet links to the HN post (cross-pollination)

**10:00 AM: Post on LinkedIn**
1. Post `LAUNCH/linkedin_post.md` content

**12:00 PM: Product Hunt goes live**
1. Product Hunt launches reset at 12:01 AM PST — if you submitted 3 days ago, it is live
2. Post the maker comment immediately
3. Share the PH link in the Twitter thread as a reply

**Throughout the day:**
- Monitor: HN score, GHE spec repo stars, Studio signups, API keys created
- Respond to every comment, issue, and question within 30 minutes
- Do not push code today — today is for community

---

## Days 58–60 — Community Response + Iteration

🎯 **Goal:** Every comment answered. Top issues triaged. First improvements shipped.

---

**Daily pattern for Days 58–60:**

**Morning (enterprise desktop — 2 hours):**
- Read all new HN comments, GitHub issues, Product Hunt comments
- Identify the 1–2 most-requested items
- Write fixes or improvements using Claude Code → push to GHE

**Afternoon (TUF F15 — 2 hours):**
- Pull → build → test → push verified changes
- If changes are non-breaking: `railway up` to deploy immediately

**What to watch for:**
Issues that commonly appear on day 2–3 of a developer tool launch:
1. "The Python SDK doesn't work on Windows" — path separator or CRLF issue
2. "The SPDF file I generate doesn't open in [X PDF viewer]" — render layer edge case
3. "The docs example has a typo/wrong output" — fix within the hour, high visibility
4. "Is there a Go/Java/.NET SDK?" — standard response: "not yet, PRs welcome"
5. "How do I convert existing PDFs?" — standard response: "coming as a paid feature, subscribe for updates"

**The conversion feature re-enable decision (Days 58–60):**
If by Day 60 you have one paying Pro subscriber ($29), immediately begin the 2-day PDF conversion build. It is already fully designed — the API contract, the Claude prompt, the Celery task, the confidence scoring. You are only re-enabling something that was intentionally parked.

---

## Week 12 KPIs

| Metric | Target | What it means |
|---|---|---|
| Spec repo stars | 200 | Developers found it interesting |
| Studio registered users | 100 | People tried it |
| API keys created | 50 | Developers are integrating |
| PyPI downloads | 500 | SDK is being used |
| HN score | 100+ | Community validated |
| First paying user | 1 | Product is commercial |
| Enterprise inbound conversations | 1 | Pipeline started |

---

# Appendix A — Feature Re-Enable Guides

Each deferred feature has a pre-written re-enable guide. When the trigger fires, execute the guide.

---

## RE-ENABLE 1: PDF → SPDF Conversion
**Trigger:** First Pro user pays ($29 received in Stripe)
**Time to re-enable:** 2 days
**Cost impact:** Anthropic API ~$0.05–0.30/conversion (covered by Pro revenue)

**Day 1:**
```
Build services/worker/ — the Celery conversion worker.

Create services/worker/celery_app.py and services/worker/tasks/conversion.py.

The conversion pipeline (pdf_to_spdf_task):
Step 1: Download PDF from R2 using input_r2_key, update job status PROCESSING
Step 2: Extract text blocks with pdfplumber: [{text, x, y, width, height, fontsize}]
Step 3: Complexity detection → model selection:
  pages ≤ 5 AND tables ≤ 3 AND no multi-currency → claude-haiku-4-5
  otherwise → claude-sonnet-4-6
Step 4: Claude semantic extraction:
  Prompt: classify text blocks into SPDF element types, return JSON array with
  element_type, properties, confidence_score per element.
  Retry 3x with backoff (1s, 2s, 4s) on failure.
Step 5b fallback: heuristic extraction if Claude fails
Step 5: DOM assembly, validation
Step 6: Container assembly (use original PDF as render layer)
Step 7: Upload to R2, update job COMPLETED

Also re-enable POST /v1/documents/upload in services/api/routers/upload.py:
- Validate %PDF magic bytes, enforce 50MB limit
- Upload to spdf-uploads bucket
- CREATE conversion_jobs record, enqueue pdf_to_spdf_task.delay(job_id)
- Return 202 with job_id

Remove the "Coming Soon" banner from Studio upload page.

requirements: celery==5.3.6, pdfplumber==0.10.3, anthropic==0.18.0
```

**Day 2:** Test the pipeline, update Anthropic spend limit to $100/month, update Studio.

---

## RE-ENABLE 2: Full X.509 Certificate Signing
**Trigger:** First enterprise inquiry
**Time to re-enable:** 3 days

```
In crates/spdf-core/src/security/signing.rs, replace sign_document_simple()
with sign_document_x509() using the openssl crate.

The X.509 signing path accepts a PKCS#12 (.p12) file.
Signs document_hash using RSA-PSS-SHA256.
Stores full X.509 certificate chain in signatures/certificate_001.pem.
Sets signature_type: "X509" in the signature JSON.

Keep sign_document_simple() as the free tier path.
Add to config: SIGNING_MODE = "simple" | "x509"
Enterprise tier uses x509, Free/Pro tier uses simple.

Update POST /v1/documents/{id}/sign to accept optional certificate upload.
If certificate provided and tier is ENTERPRISE: use X.509 path.
Otherwise: use simple path.
```

---

## RE-ENABLE 3: Natural Language `/ask` Endpoint
**Trigger:** 10+ active users (defined as: 10 users with at least 1 API call in last 7 days)
**Time to re-enable:** 1 day

```
Add POST /v1/documents/{document_id}/ask to services/api/routers/documents.py.

Request: { "question": str, "return_structured": bool }

Implementation:
1. Fetch SPDF from R2
2. Call SpdfEngine.extract() to get structured invoice data
3. Build Claude prompt: "Given this invoice data: {structured_data}. Answer: {question}. 
   Return JSON: { answer: str, structured_answer: dict | null, confidence: float }"
4. Call Anthropic API (claude-haiku-4-5 for most questions)
5. Return response matching API contract Section 7.1

Count as event_type: "AI_QUERY" in usage_events. Rate limit: 100/day on FREE tier.
```

---

## RE-ENABLE 4: Webhook Delivery
**Trigger:** First GitHub issue requesting webhooks
**Time to re-enable:** 2 days

```
Add webhook delivery to services/worker/tasks/webhooks.py:

async def deliver_webhook(job_id: str):
  Load job record, get webhook_url and webhook_secret
  Build payload: { event, job_id, status, output_document_id, completed_at }
  Compute HMAC-SHA256 signature using webhook_secret
  POST to webhook_url with header X-SPDF-Signature: sha256={hmac_hex}
  On non-2xx: retry with backoff (30s, 5m, 30m, 2h, 12h)
  After 5 failures: mark webhook_delivered=FALSE, log failure

Call deliver_webhook.delay(job_id) at end of pdf_to_spdf_task if webhook_url is set.
```

---

## RE-ENABLE 5: Stripe Live Mode
**Trigger:** Public launch day (Week 12 Day 55)
**Time to re-enable:** 30 minutes
**Already done in Week 11 Day 55 above.**

---

# Appendix B — Daily Git Workflow

**On enterprise desktop (start of session):**
```bash
cd C:/Projects/spdf
git pull origin main
git checkout -b feature/day-XX-description
# Write code with Claude Code
git add -A
git commit -m "feat: description"
git push origin feature/day-XX-description
# Create PR on GHE → merge to main
```

**On TUF F15 (verification session):**
```bash
cd ~/projects/spdf && git pull origin main
# Build and test
# If fixes needed: commit and push
```

**Commit format:** `feat:` · `fix:` · `chore:` · `test:` · `docs:` · `refactor:`

---

# Appendix C — Environment Variables

| Variable | Local Value (`.env.local`) | Production (Doppler `prd`) | When Needed |
|---|---|---|---|
| `DATABASE_URL` | `postgresql+asyncpg://spdf:spdf_dev_password@localhost:5432/spdf_dev` | Supabase URI | Week 1 |
| `REDIS_URL` | `redis://localhost:6379/0` | Upstash `rediss://` URL | Week 1 |
| `R2_ENDPOINT` | `http://localhost:9000` | `https://ACCOUNT_ID.r2.cloudflarestorage.com` | Week 1 |
| `R2_ACCESS_KEY_ID` | `minioadmin` | Cloudflare R2 key | Week 1 |
| `R2_SECRET_ACCESS_KEY` | `minioadmin` | Cloudflare R2 secret | Week 1 |
| `R2_BUCKET_DOCUMENTS` | `spdf-documents` | `spdf-documents` | Week 1 |
| `R2_BUCKET_UPLOADS` | `spdf-uploads` | `spdf-uploads` | Week 1 |
| `ANTHROPIC_API_KEY` | `not-needed-yet` | Real key (stored, unused until RE-ENABLE 1) | RE-ENABLE 1 |
| `CLERK_SECRET_KEY` | Clerk test key | Clerk live key | Week 3 |
| `CLERK_WEBHOOK_SECRET` | Clerk webhook secret | Same | Week 3 |
| `STRIPE_SECRET_KEY` | `sk_test_...` | `sk_live_...` (switched Week 11 Day 55) | Week 4 |
| `STRIPE_WEBHOOK_SECRET` | Stripe test secret | Stripe live secret | Week 4 |
| `STRIPE_PRICE_PRO` | Test price ID | Live price ID | Week 4 |
| `STRIPE_PRICE_TEAM` | Test price ID | Live price ID | Week 4 |
| `RESEND_API_KEY` | `not-needed-yet` | Real key | Week 11 |
| `SENTRY_DSN` | (empty) | Real DSN | Week 11 |
| `ENVIRONMENT` | `development` | `production` | Always |
| `LOG_LEVEL` | `debug` | `info` | Always |

---

# Appendix D — Cost Timeline (Final)

| Week | What Happens | Running Monthly Cost |
|---|---|---|
| 1–10 | Everything local on TUF F15 | **$0** |
| 11 | Railway + Vercel + domain deployed | **$15–25** |
| 12 (launch) | Stripe live mode (costs only on revenue) | **$15–25** |
| Post-launch, 0 paying users | Status quo | **$15–25** |
| First Pro user ($29/mo) | Revenue > cost. Re-enable PDF conversion | **~$0 net** |
| 5 Pro users ($145/mo) | Anthropic API costs covered. Full platform | **~$120 net** |
| 10 Pro users ($290/mo) | First enterprise outreach begins | **~$265 net** |

---

*— End of SPDF 12-Week Development & GTM Plan v2.0 —*
*SPDF-SPRINT-2025-001 · Cost-Optimised · March 2025*
