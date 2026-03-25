# SPDF Platform — 12-Week Development & GTM Plan
## SPDF-SPRINT-2025-001 · Version 1.0

---

## Executive Summary

| Parameter | Value |
|---|---|
| **Capacity** | 4–5 hours/day · 5 days/week · ~22 hours/week · 264 hours total |
| **Primary dev machine** | ASUS TUF F15 (i7-12700H, 16GB DDR5, RTX 4060) — personal laptop |
| **Secondary machine** | Enterprise Win11 desktop — code writing + AI assistance only |
| **AI coding tools** | Claude Code on enterprise desktop · GitHub Copilot on personal laptop |
| **Repository** | Company GitHub Enterprise (personal repo, accessible from both machines) |
| **Execution model** | Solo founder · Claude Code/Copilot writes ~90% of code |

---

## The Two-Machine Workflow

This is the core operating model for all 12 weeks. Every day follows this pattern.

```
┌─────────────────────────────────┐     GHE Repo      ┌──────────────────────────────────┐
│   ENTERPRISE DESKTOP            │  (single source   │   PERSONAL LAPTOP (TUF F15)      │
│                                 │   of truth)       │                                  │
│  • Claude Code + Copilot        │                   │  • All runtimes installed         │
│  • VS Code (code writing)       │  ◄── git pull ──  │  • Docker + local services       │
│  • Design & architecture        │  ── git push ──►  │  • cargo build / run             │
│  • AI-assisted code generation  │                   │  • pytest / npm run dev          │
│  • Documentation writing        │                   │  • Actually executes everything  │
│  • PR reviews on GHE            │                   │  • GitHub Copilot for local edits│
│                                 │                   │                                  │
│  NEVER runs code                │                   │  NEVER uses Claude Code          │
└─────────────────────────────────┘                   └──────────────────────────────────┘
```

**The daily rhythm:**
1. Morning (enterprise desktop): Pull latest, review previous day's work, write new code with Claude Code, push
2. Lunch/evening (personal laptop): Pull, run builds and tests, fix anything broken, push fixes
3. Async: Both machines stay in sync via GHE. You never need both open simultaneously

---

## How to Read This Plan

Each day entry contains:
- 🎯 **Goal** — what "done" looks like at end of day
- 🤖 **Claude Code** — exact tasks and prompt guidance for AI-assisted coding (enterprise desktop)
- 🖐 **Manual Actions** — numbered step-by-step guides for human actions
- 💻 **Run on TUF** — commands to execute on the personal laptop
- ✅ **Verify** — how to confirm the day's work is complete
- ⚠️ **Risk** — common failure points and how to avoid them
- ⏱ **Time budget** — realistic allocation

**Symbols:** 🤖 Claude Code · 🖐 Manual step · 💻 Personal laptop · ✅ Checkpoint · ⚠️ Risk · 📌 Decision

---

## Phase Overview

| Phase | Weeks | Theme | Primary Machine | Key Deliverable |
|---|---|---|---|---|
| **1 — Foundation** | 1–2 | Environment + Rust Core Engine | TUF F15 (setup) + Enterprise (code) | Valid SPDF files generated and parsed |
| **2 — Backend API** | 3–5 | FastAPI + Celery + Claude conversion | Both | REST API live, PDF→SPDF working |
| **3 — Studio** | 6–7 | React frontend + WASM viewer | Both | Upload, view, export in browser |
| **4 — Billing & Auth** | 8 | Clerk + Stripe + API keys | Both | Paid tiers enforced end-to-end |
| **5 — Hardening** | 9–10 | Testing, security, observability | Both | Production-grade reliability |
| **6 — GTM** | 11–12 | Launch preparation and execution | Enterprise (writing) | Public launch, first users |

---

# WEEK 1 — Environment Setup (Personal Laptop) + Project Scaffold

**Theme:** Every tool installed on the TUF F15, GHE repo created, monorepo skeleton committed. No feature code this week.

**Weekly Goal:** `just dev` starts a working local environment on the TUF F15. GHE repo exists and both machines can push/pull.

**Primary machine this week:** TUF F15 (all installs) + Enterprise desktop (repo creation, scaffold generation)

---

## Day 1 — TUF F15: WSL2, Core Tools, Rust

🎯 **Goal:** WSL2 running Ubuntu 22.04 on the TUF F15, Rust installed and verified.

⏱ **Time Budget:** 4.5 hours (all on personal laptop)

---

### 🖐 Manual Action 1.1 — Enable WSL2 on TUF F15
**Machine:** Personal laptop (TUF F15) · **Time:** 30 minutes

The TUF F15 is a personal machine with no IT restrictions. WSL2 will work.

1. Open **PowerShell as Administrator** (right-click Start → "Windows PowerShell (Admin)")
2. Enable WSL and Virtual Machine Platform:
   ```powershell
   wsl --install
   ```
   This installs WSL2 with Ubuntu 22.04 by default.
3. **Restart the laptop** when prompted
4. After restart, Ubuntu 22.04 opens automatically. Create a Linux username (e.g. `dev`) and a strong password
5. Once at the Ubuntu prompt, run:
   ```bash
   sudo apt update && sudo apt upgrade -y
   ```
6. Install build essentials:
   ```bash
   sudo apt install -y build-essential curl git wget unzip pkg-config \
     libssl-dev libffi-dev python3-dev ca-certificates gnupg lsb-release \
     software-properties-common apt-transport-https
   ```
7. Verify WSL2 (not WSL1). Open a new PowerShell window:
   ```powershell
   wsl -l -v
   ```
   VERSION column must show `2`.

⚠️ **If virtualisation error appears:** The TUF F15 BIOS has virtualisation enabled by default for gaming. If you hit an error, go to BIOS (F2 on boot) → Advanced → CPU Configuration → Intel Virtualization Technology → Enabled.

---

### 🖐 Manual Action 1.2 — Install Rust on TUF F15
**Machine:** Personal laptop · **Time:** 15 minutes

All following commands run inside the **WSL2 Ubuntu terminal** on the TUF F15.

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Choose option 1 (default)

# Load into current shell
source "$HOME/.cargo/env"

# Add WASM compilation target (needed for browser builds in Week 6)
rustup target add wasm32-unknown-unknown

# Install wasm-pack (WASM bundler)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install maturin (PyO3 build tool — Rust→Python bindings)
pip3 install maturin

# Verify
rustc --version   # rustc 1.75.0 or later
cargo --version   # cargo 1.75.0 or later
```

---

### 🖐 Manual Action 1.3 — Install Python 3.12, Node 20, Docker, just
**Machine:** Personal laptop · **Time:** 25 minutes

```bash
# Python 3.12
sudo add-apt-repository ppa:deadsnakes/ppa -y
sudo apt update
sudo apt install -y python3.12 python3.12-dev python3.12-venv python3.12-distutils
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.12
python3.12 --version   # Python 3.12.x

# Node.js 20
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
node --version    # v20.x.x
npm --version     # 10.x.x

# Docker
sudo apt install -y docker.io docker-compose
sudo usermod -aG docker $USER
newgrp docker
docker --version  # Docker version 24.x or later

# just (task runner — replaces make)
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/.local/bin
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
just --version    # just 1.x.x
```

---

### 🖐 Manual Action 1.4 — Configure VS Code on TUF F15 for WSL
**Machine:** Personal laptop · **Time:** 20 minutes

1. Download and install [VS Code for Windows](https://code.visualstudio.com/) on the TUF F15 (Windows side)
2. Open VS Code → Extensions (Ctrl+Shift+X) → install **WSL** by Microsoft
3. Press `Ctrl+Shift+P` → "WSL: Connect to WSL" → VS Code reopens in WSL mode
4. Inside WSL-connected VS Code, install these extensions:
   - `rust-analyzer` — Rust language server (autocomplete, error highlighting)
   - `Python` by Microsoft
   - `Pylance`
   - `ES Lint`
   - `Prettier`
   - `GitHub Copilot` (sign in with your GitHub account)
5. Set default terminal to WSL: `Ctrl+Shift+P` → "Terminal: Select Default Profile" → "WSL Bash"

---

✅ **Day 1 Verification:**
```bash
# Run all of these in WSL terminal on TUF F15
rustc --version && cargo --version
python3.12 --version
node --version
docker --version
just --version
wasm-pack --version
```
All must return version numbers, zero errors.

---

## Day 2 — GHE Repo + Git Configuration on Both Machines

🎯 **Goal:** GHE repository created, both machines cloned and pushing/pulling successfully.

⏱ **Time Budget:** 3 hours

---

### 🖐 Manual Action 2.1 — Create GHE Repository
**Machine:** Enterprise desktop · **Time:** 20 minutes

1. Open your company's GitHub Enterprise in the browser (your company's GHE URL)
2. Click **+** (top right) → **New repository**
3. Set:
   - **Owner:** Your personal GHE account (not a work org)
   - **Repository name:** `spdf`
   - **Visibility:** Private
   - **Initialize:** Add README, add `.gitignore` (select Rust), add license (MIT)
4. Click **Create repository**
5. Copy the repository HTTPS clone URL (format: `https://YOUR-GHE-HOST/YOUR-USERNAME/spdf.git`)

---

### 🖐 Manual Action 2.2 — Configure Git on TUF F15 (WSL)
**Machine:** Personal laptop · **Time:** 15 minutes

```bash
# Basic git config
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
git config --global core.autocrlf false    # Critical: prevents CRLF issues on Windows
git config --global init.defaultBranch main

# Configure GHE credentials
# Method: Personal Access Token (PAT)
# On GHE: Settings → Developer Settings → Personal access tokens → Generate new token
# Scopes needed: repo (full), workflow
# Copy the token — it shows once only

# Store credentials so you don't type the token every push
git config --global credential.helper store

# Clone the repo
mkdir -p ~/projects && cd ~/projects
git clone https://YOUR-GHE-HOST/YOUR-USERNAME/spdf.git
cd spdf
# When prompted: username = your GHE username, password = your PAT token
# This gets stored by credential.helper store

ls   # Should show: LICENSE, README.md, .gitignore
```

---

### 🖐 Manual Action 2.3 — Configure Git on Enterprise Desktop
**Machine:** Enterprise desktop · **Time:** 15 minutes

Git is already installed on the enterprise desktop.

1. Open **Git Bash** (from Start menu — installed with Git for Windows)
2. Configure identity:
   ```bash
   git config --global user.name "Your Name"
   git config --global user.email "your@email.com"
   git config --global core.autocrlf false
   ```
3. Create a second PAT on GHE (separate from the TUF F15 one — label it "enterprise-desktop"):
   - GHE → Settings → Developer Settings → Personal access tokens → Generate
   - Scopes: `repo`, `workflow`
4. Clone the repo:
   ```bash
   mkdir -p C:/Projects
   cd C:/Projects
   git clone https://YOUR-GHE-HOST/YOUR-USERNAME/spdf.git
   cd spdf
   ```
5. Open this folder in VS Code: `code .`

---

✅ **Day 2 Verification:**
- On TUF F15: `cd ~/projects/spdf && git status` shows clean working tree
- On enterprise desktop: `cd C:/Projects/spdf && git log --oneline` shows initial commit
- Make a test change on enterprise desktop, push, pull on TUF F15 → confirms sync works

---

## Day 3 — Monorepo Scaffold (Enterprise Desktop → TUF F15)

🎯 **Goal:** Complete monorepo structure committed, `cargo check` passes on TUF F15.

⏱ **Time Budget:** 4 hours (2h enterprise desktop writing, 2h TUF F15 verifying)

---

### 🤖 Claude Code Task 3.1 — Generate Complete Monorepo Scaffold
**Machine:** Enterprise desktop · **Time:** 90 minutes

Open Claude Code in `C:/Projects/spdf` and use this prompt:

```
I am building SPDF — a next-generation document format platform with a Rust core 
engine, Python FastAPI backend, Celery worker, and React frontend.

Create a complete production-ready monorepo scaffold with this exact directory structure:

spdf/
├── .github/
│   └── workflows/         (empty directory — we'll add CI later)
├── crates/
│   ├── spdf-core/         (Rust library crate — the core engine)
│   ├── spdf-wasm/         (Rust WASM bindings for browser)
│   └── spdf-python/       (Rust PyO3 bindings for Python SDK)
├── services/
│   ├── api/               (Python FastAPI application)
│   └── worker/            (Python Celery worker)
├── apps/
│   └── studio/            (React + Vite frontend)
├── packages/
│   ├── spdf-python/       (pip-publishable Python SDK)
│   └── spdf-js/           (npm-publishable TypeScript SDK)
├── spec/                  (SPDF format specification)
├── docs/                  (Developer documentation)
├── scripts/               (Utility scripts)
├── infra/                 (IaC — empty for now)
├── Cargo.toml             (Rust workspace manifest)
├── docker-compose.yml     (Local dev services)
├── justfile               (Task runner)
└── .env.example           (Environment variable template)

Please create:

1. Root Cargo.toml as a workspace including all three crates

2. crates/spdf-core/Cargo.toml with these exact dependencies:
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
   - A public SpdfError enum using thiserror with variants for: 
     InvalidContainer, InvalidJson, ChecksumMismatch, MissingRequiredFile,
     InvalidVersion, DuplicateEid, AssetMissing, SignatureInvalid, IoError(#[from] std::io::Error)
   - A public SpdfVersion struct with major: u32, minor: u32 and a parse() method
   - A stub pub fn validate_container(bytes: &[u8]) -> Result<(), SpdfError> that returns Ok(())
   - All types must derive Debug, Clone where appropriate

4. crates/spdf-wasm/Cargo.toml as cdylib with wasm-bindgen dependency
5. crates/spdf-python/Cargo.toml as cdylib with pyo3 dependency (feature = "extension-module")
6. Minimal src/lib.rs in spdf-wasm and spdf-python with a hello() function

7. docker-compose.yml with:
   - postgres:15 on port 5432, database: spdf_dev, user: spdf, password: spdf_dev_password
   - redis:7-alpine on port 6379
   - minio/minio on ports 9000/9001 as S3-compatible R2 replacement for local dev
   - All services with health checks
   - A named volume for each service

8. justfile with these recipes:
   dev: starts docker-compose in background
   build-rust: cargo build --workspace
   build-wasm: wasm-pack build crates/spdf-wasm --target web
   build-pyo3: maturin develop --release in crates/spdf-python
   test-rust: cargo test --workspace
   test-python: pytest services/ packages/spdf-python/ -v  
   test-frontend: cd apps/studio && npm test
   db-migrate: alembic -c services/api/alembic.ini upgrade head
   db-rollback: alembic -c services/api/alembic.ini downgrade -1
   lint: cargo clippy -- -D warnings && ruff check .
   clean: cargo clean && find . -name __pycache__ -exec rm -rf {} +

9. .env.example with all required environment variables (empty values):
   DATABASE_URL=
   REDIS_URL=
   R2_ENDPOINT=
   R2_ACCESS_KEY_ID=
   R2_SECRET_ACCESS_KEY=
   R2_BUCKET_DOCUMENTS=
   R2_BUCKET_UPLOADS=
   ANTHROPIC_API_KEY=
   CLERK_SECRET_KEY=
   CLERK_WEBHOOK_SECRET=
   STRIPE_SECRET_KEY=
   STRIPE_WEBHOOK_SECRET=
   RESEND_API_KEY=
   SENTRY_DSN=
   ENVIRONMENT=development
   LOG_LEVEL=debug

10. Update .gitignore to add:
    .env
    .env.local
    target/
    __pycache__/
    *.pyd
    *.so
    .venv/
    node_modules/
    dist/
    pkg/

Generate all files completely. Do not use placeholder comments.
```

After generation, commit and push from enterprise desktop:
```bash
cd C:/Projects/spdf
git add -A
git commit -m "chore: initial monorepo scaffold with Rust workspace, Docker, and justfile"
git push
```

---

### 💻 Run on TUF F15 — Verify Scaffold
**Time:** 45 minutes

```bash
cd ~/projects/spdf
git pull

# Test Rust workspace compiles
cargo check --workspace
# Expected: Compiling spdf-core, spdf-wasm, spdf-python — no errors

# Start local services
just dev
# Expected: postgres, redis, minio containers start

# Verify services are running
docker ps
# Should show 3 containers running

# Test database connection
docker exec -it spdf_postgres_1 psql -U spdf -d spdf_dev -c "SELECT version();"
# Should return PostgreSQL version
```

✅ **Day 3 Verification:** `cargo check` passes. `just dev` starts 3 containers. No errors.

---

## Day 4 — External Accounts Setup

🎯 **Goal:** All 9 external service accounts created and API keys collected. Doppler set up for secret management.

⏱ **Time Budget:** 4 hours (all on enterprise desktop browser + TUF F15 for Doppler CLI)

⚠️ **Critical:** Keep a temporary private text file open to collect keys as you create them. Move everything to Doppler at the end of this day. Delete the text file immediately after.

---

### 🖐 Manual Action 4.1 — Create All Service Accounts
**Machine:** Enterprise desktop (browser) · **Time:** 2.5 hours

Work through each service sequentially:

**1. Anthropic (Claude API) — 20 min**
1. Go to [console.anthropic.com](https://console.anthropic.com) → Create account
2. Verify email → Add payment method
3. API Keys → Create Key → name: `spdf-dev`
4. Set monthly spend limit: $50 (prevents runaway costs during development)
5. Save: `ANTHROPIC_API_KEY=sk-ant-...`

**2. Supabase (PostgreSQL) — 15 min**
1. Go to [supabase.com](https://supabase.com) → Sign up with GitHub (your personal GitHub, linked to GHE)
2. New project → name: `spdf-dev` → generate strong database password → region: Asia (Mumbai) or nearest
3. Wait for provisioning (~2 minutes)
4. Settings → API → copy:
   - Project URL → `SUPABASE_URL=https://xxxx.supabase.co`
   - `anon` public key → `SUPABASE_ANON_KEY=...`
   - `service_role` secret key → `SUPABASE_SERVICE_KEY=...`
5. Settings → Database → Connection string (URI) → `DATABASE_URL=postgresql://...`

**3. Upstash (Redis) — 10 min**
1. Go to [upstash.com](https://upstash.com) → Sign up
2. Create Redis database → name: `spdf-dev` → region: nearest → type: Regional
3. Copy: `REDIS_URL=rediss://...` (the full Redis connection string with password)

**4. Cloudflare R2 (Object Storage) — 20 min**
1. Go to [cloudflare.com](https://cloudflare.com) → Create account (free plan)
2. Dashboard → R2 Object Storage → Create bucket
   - Bucket 1: `spdf-documents` (permanent document storage)
   - Bucket 2: `spdf-uploads` (temporary upload staging, 48h lifecycle)
3. R2 → Manage R2 API Tokens → Create API Token
   - Permissions: Object Read & Write on both buckets
4. Copy: `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY`
5. Copy account ID from URL bar or dashboard: `R2_ACCOUNT_ID`
6. R2 endpoint format: `https://ACCOUNT_ID.r2.cloudflarestorage.com`

**5. Clerk (Authentication) — 15 min**
1. Go to [clerk.com](https://clerk.com) → Sign up
2. Create application → name: `SPDF` → enable: Email, Google sign-in
3. API Keys page → copy `CLERK_PUBLISHABLE_KEY` and `CLERK_SECRET_KEY`
4. Webhooks → Add Endpoint:
   - URL: `https://placeholder.example.com/v1/webhooks/clerk` (update in Week 3)
   - Events: `user.created`, `user.updated`, `user.deleted`, `organization.created`, `organizationMembership.created`
5. Copy `CLERK_WEBHOOK_SECRET`

**6. Stripe (Payments — Test Mode) — 15 min**
1. Go to [stripe.com](https://stripe.com) → Create account
2. Ensure you are in **Test Mode** (toggle in top-left dashboard)
3. Developers → API Keys → copy `STRIPE_PUBLISHABLE_KEY` (pk_test_...) and `STRIPE_SECRET_KEY` (sk_test_...)
4. Developers → Webhooks → Add endpoint:
   - URL: `https://placeholder.example.com/v1/webhooks/stripe`
   - Events: `customer.subscription.created`, `customer.subscription.updated`, `customer.subscription.deleted`, `invoice.paid`, `invoice.payment_failed`
5. Copy `STRIPE_WEBHOOK_SECRET`
6. Products → Create 3 products:
   - Pro: $29/month recurring → copy Price ID (`price_xxx`)
   - Team: $99/month recurring → copy Price ID
   - Enterprise: custom → skip for now

**7. Resend (Email) — 5 min**
1. Go to [resend.com](https://resend.com) → Sign up
2. API Keys → Create API Key → name: `spdf-dev`
3. Copy `RESEND_API_KEY=re_...`

**8. Sentry (Error Tracking) — 5 min**
1. Go to [sentry.io](https://sentry.io) → Sign up (free tier)
2. Create project → Platform: FastAPI → name: `spdf-api`
3. Copy the DSN URL → `SENTRY_DSN=https://xxx@xxx.ingest.sentry.io/xxx`

**9. Railway (API Hosting) — 5 min**
1. Go to [railway.app](https://railway.app) → Sign up with GitHub
2. No setup needed yet — just create the account
3. Dashboard → Account Settings → Tokens → Create token → name: `spdf-deploy`
4. Copy `RAILWAY_TOKEN=...`

---

### 🖐 Manual Action 4.2 — Set Up Doppler Secret Management
**Machine:** TUF F15 (WSL) · **Time:** 30 minutes

Doppler replaces `.env` files. All secrets live in Doppler. Never committed to git.

```bash
# Install Doppler CLI in WSL
(curl -Ls --tlsv1.2 --proto "=https" --retry 3 https://cli.doppler.com/install.sh || wget -t 3 -qO- https://cli.doppler.com/install.sh) | sudo sh

# Verify
doppler --version

# Login (opens browser)
doppler login
```

Now create the project structure on [doppler.com](https://doppler.com):
1. Go to doppler.com → Create account (or login)
2. Create project → name: `spdf`
3. The project auto-creates three environments: `dev`, `stg`, `prd`
4. Click `dev` environment → Add all secrets from your temporary notes file:
   ```
   DATABASE_URL          = (your Supabase connection string)
   REDIS_URL             = (your Upstash Redis URL)
   R2_ENDPOINT           = https://YOUR_ACCOUNT_ID.r2.cloudflarestorage.com
   R2_ACCESS_KEY_ID      = (Cloudflare R2 access key)
   R2_SECRET_ACCESS_KEY  = (Cloudflare R2 secret key)
   R2_BUCKET_DOCUMENTS   = spdf-documents
   R2_BUCKET_UPLOADS     = spdf-uploads
   ANTHROPIC_API_KEY     = (your Anthropic key)
   CLERK_SECRET_KEY      = (your Clerk secret key)
   CLERK_WEBHOOK_SECRET  = (your Clerk webhook secret)
   STRIPE_SECRET_KEY     = (your Stripe test secret key)
   STRIPE_WEBHOOK_SECRET = (your Stripe webhook secret)
   RESEND_API_KEY        = (your Resend API key)
   SENTRY_DSN            = (your Sentry DSN)
   ENVIRONMENT           = development
   LOG_LEVEL             = debug
   R2_ACCOUNT_ID         = (your Cloudflare account ID)
   ```
5. Back in WSL terminal, link the project:
   ```bash
   cd ~/projects/spdf
   doppler setup
   # Select project: spdf
   # Select config: dev
   
   # Test it works
   doppler run -- env | grep ANTHROPIC
   # Should print your API key
   ```

⚠️ **Delete your temporary notes file now.** The keys are in Doppler.

✅ **Day 4 Verification:** `doppler run -- env | grep ANTHROPIC_API_KEY` returns your key. All 9 services have accounts.

---

## Day 5 — Python API Skeleton + FastAPI Foundation

🎯 **Goal:** FastAPI application runs locally, serves `GET /v1/health`, connects to local Postgres via Docker.

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 5.1 — Generate FastAPI Application Skeleton
**Machine:** Enterprise desktop · **Time:** 2 hours

```
Create a complete production-ready FastAPI application skeleton in services/api/.

Requirements:
1. Python 3.12, FastAPI 0.110+, SQLAlchemy 2.0 (async), Alembic, Pydantic v2

2. Directory structure:
   services/api/
   ├── main.py              (FastAPI app factory)
   ├── config.py            (Pydantic Settings — all config from env)
   ├── dependencies.py      (FastAPI DI: db, redis, storage, current_user)
   ├── routers/
   │   ├── __init__.py
   │   ├── health.py        (GET /v1/health, GET /v1/health/ready)
   │   └── documents.py     (stub — just the router, no handlers yet)
   ├── models/
   │   ├── __init__.py
   │   ├── base.py          (DeclarativeBase, TimestampMixin, SoftDeleteMixin)
   │   ├── user.py          (User model — exact schema from SPDF-DB-2025-001)
   │   ├── organization.py  (Organization + OrgMember models)
   │   ├── document.py      (Document model)
   │   └── job.py           (ConversionJob model)
   ├── schemas/
   │   ├── __init__.py
   │   ├── health.py        (HealthResponse, ReadinessResponse)
   │   └── errors.py        (StandardErrorResponse, ErrorDetail)
   ├── middleware/
   │   ├── __init__.py
   │   ├── request_id.py    (inject X-Request-Id UUID4 on every request)
   │   └── logging.py       (structured JSON logging per request)
   ├── migrations/
   │   └── env.py           (Alembic environment)
   ├── alembic.ini
   ├── pyproject.toml
   └── requirements.txt

3. config.py must use Pydantic BaseSettings reading from environment variables:
   - database_url, redis_url, r2_endpoint, r2_access_key_id, r2_secret_access_key
   - r2_bucket_documents, r2_bucket_uploads, anthropic_api_key
   - clerk_secret_key, clerk_webhook_secret
   - stripe_secret_key, stripe_webhook_secret
   - resend_api_key, sentry_dsn
   - environment (default: "development"), log_level (default: "debug"), version (default: "1.0.0")

4. main.py must:
   - Create FastAPI app with title="SPDF API", version from config
   - Add CORSMiddleware (origins from config, default allow all in dev)
   - Add the request_id and logging middleware
   - Include health router at prefix=""
   - Include documents router at prefix="/v1"
   - Add a startup event that logs "SPDF API starting" with version
   - Return proper JSON 404 and 422 error responses matching the API contract error format

5. Health endpoints must match the API contract exactly:
   GET /v1/health → { status: "ok", version: "1.0.0", environment: "development", timestamp: "..." }
   GET /v1/health/ready → { status: "ready", checks: { database: {...}, redis: {...} } }
   health/ready must actually check database and redis connectivity

6. SQLAlchemy models must exactly match the database schema from SPDF-DB-2025-001:
   - Use UUID primary keys with uuid_generate_v4() server default
   - All timestamps as DateTime(timezone=True)
   - Soft delete via deleted_at column
   - Use Mapped[] type annotations throughout

7. Create an initial Alembic migration that creates all 4 tables:
   users, organizations, org_members, documents
   (conversion_jobs, templates, assets, billing tables will be added in Week 3)

8. requirements.txt must include:
   fastapi==0.110.0
   uvicorn[standard]==0.27.0
   sqlalchemy[asyncio]==2.0.25
   alembic==1.13.0
   asyncpg==0.29.0
   pydantic==2.6.0
   pydantic-settings==2.1.0
   python-multipart==0.0.9
   httpx==0.26.0
   redis==5.0.0
   python-jose[cryptography]==3.3.0
   sentry-sdk[fastapi]==1.40.0
   structlog==24.1.0
```

After generation, commit and push:
```bash
git add -A
git commit -m "feat: FastAPI application skeleton with health endpoints and SQLAlchemy models"
git push
```

---

### 💻 Run on TUF F15 — Install and Test API
**Time:** 1.5 hours

```bash
cd ~/projects/spdf
git pull

# Create Python virtual environment for the API
cd services/api
python3.12 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# Create .env.local for local overrides (Doppler handles everything else)
# For local Docker services, we override the cloud URLs:
cat > .env.local << 'EOF'
DATABASE_URL=postgresql+asyncpg://spdf:spdf_dev_password@localhost:5432/spdf_dev
REDIS_URL=redis://localhost:6379/0
R2_ENDPOINT=http://localhost:9000
R2_ACCESS_KEY_ID=minioadmin
R2_SECRET_ACCESS_KEY=minioadmin
ENVIRONMENT=development
EOF

# Run database migrations
just dev   # Ensure Docker services are running
doppler run --config dev -- alembic upgrade head
# OR with .env.local:
source .env.local && alembic upgrade head

# Start the API server
doppler run --config dev -- uvicorn main:app --host 0.0.0.0 --port 8000 --reload
```

In another terminal:
```bash
# Test health endpoints
curl http://localhost:8000/v1/health | python3.12 -m json.tool
curl http://localhost:8000/v1/health/ready | python3.12 -m json.tool
```

✅ **Week 1 Complete Verification:**
- `cargo check --workspace` — passes on TUF F15
- `curl http://localhost:8000/v1/health` — returns `{"status": "ok", ...}`
- `curl http://localhost:8000/v1/health/ready` — returns `{"status": "ready", ...}`
- `docker ps` — shows postgres, redis, minio running
- GHE repo has commits from both machines
- Doppler has all secrets configured

---

# WEEK 2 — Rust Core Engine: Parser, Writer, and Validator

**Theme:** The most technically challenging week. Claude Code writes all Rust. Your job is to review, test, and iterate.

**Weekly Goal:** `cargo test` passes on all core engine tests. Can create, write, and parse a valid SPDF container.

**Primary machine this week:** Enterprise desktop (Claude Code writing Rust) → TUF F15 (compiling and testing)

---

## Day 6 — SPDF Container: ZIP Structure and Manifest

🎯 **Goal:** Core engine can create a valid SPDF ZIP container with all required files and correct checksums.

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 6.1 — Container Builder
**Machine:** Enterprise desktop · **Time:** 2.5 hours

```
In crates/spdf-core/src/, implement the SPDF container layer.

Create these files:

1. src/container/mod.rs — public module declarations
2. src/container/manifest.rs — Manifest struct with serialization
3. src/container/reader.rs — Read and validate an SPDF ZIP container
4. src/container/writer.rs — Write an SPDF ZIP container
5. src/container/checksums.rs — SHA-256 checksum computation

Requirements for manifest.rs:
- SpdfManifest struct with fields: format (always "SPDF"), version (SpdfVersion), profile, created_at, generator (GeneratorInfo), layers (LayerManifests), assets (Vec<AssetManifestEntry>), extensions (Vec<ExtensionDeclaration>), document_id, manifest_hash
- All structs derive Serialize, Deserialize, Debug, Clone
- version must serialize as "1.0" not {"major":1,"minor":0}
- SpdfManifest::new() creates a default v1.0 manifest
- SpdfManifest::compute_hash() computes SHA-256 of the canonical JSON form

Requirements for writer.rs:
- SpdfWriter struct
- SpdfWriter::new() creates empty writer
- fn add_layer(&mut self, name: &str, content: &[u8]) adds a container layer
- fn add_asset(&mut self, asset_id: &str, asset_type: &str, mime: &str, content: &[u8]) adds an asset file
- fn build(self) -> Result<Vec<u8>, SpdfError> assembles the ZIP container:
  - manifest.json MUST be the first ZIP entry (critical per spec)
  - All layers added in order: semantic, layout, styles, render, metadata, audit
  - Assets in assets/{type}/ subdirectories
  - Computes SHA-256 for every file and embeds in manifest
  - Returns raw bytes of the complete ZIP file

Requirements for reader.rs:
- fn read_container(bytes: &[u8]) -> Result<SpdfContainer, SpdfError>
- Validates ZIP format (magic bytes check)
- Verifies manifest.json is first entry
- Reads and parses manifest
- Verifies SHA-256 checksums of all declared files
- Returns SpdfContainer with all raw layer bytes accessible
- SpdfContainer struct exposes: manifest, semantic_bytes, layout_bytes, styles_bytes, render_bytes, metadata_bytes, audit_bytes

Requirements for checksums.rs:
- fn sha256_hex(data: &[u8]) -> String — returns lowercase hex string
- fn verify_checksum(data: &[u8], expected_hex: &str) -> bool

Write comprehensive unit tests for each function using #[cfg(test)] inline test modules.
The test for the writer must: create a minimal SPDF, write it, read it back, verify checksums match.
This round-trip test is the most important test in the codebase.
```

---

### 💻 Run on TUF F15 — Compile and Test Container Layer
**Time:** 1 hour

```bash
cd ~/projects/spdf
git pull
cargo test -p spdf-core container
# Expected: all container tests pass

# If compilation errors, read the error carefully and note them
# Go back to enterprise desktop, open Claude Code:
# "cargo compilation failed with these errors: [paste errors]. Fix them."
# Push fix, pull on TUF F15, retest
```

⚠️ **The compile-fix loop:** Rust is strict. Expect 2–3 rounds of compilation errors even with Claude Code writing the code. This is normal. The pattern is: compile → copy errors → fix in Claude Code → push → pull → recompile. Budget 30 minutes per error round.

---

## Day 7 — SPDF Document Object Model

🎯 **Goal:** Full DOM type system in Rust — Document, Page, all element types from Section 6 of the spec.

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 7.1 — DOM Type System
**Machine:** Enterprise desktop · **Time:** 2.5 hours

```
In crates/spdf-core/src/dom/, implement the complete SPDF Document Object Model.

Create:
1. src/dom/mod.rs
2. src/dom/document.rs — Document root, Page, document state machine
3. src/dom/elements/mod.rs
4. src/dom/elements/content.rs — Heading, Paragraph, Table, TableRow, TableCell, Image, VectorImage, CodeBlock, HorizontalRule, PageBreak, Attachment
5. src/dom/elements/domain.rs — InvoiceHeader, LineItem, LineItemTable, PaymentTerms
6. src/dom/elements/trust.rs — SignatureBlock, Stamp, Annotation, Redaction
7. src/dom/elements/interactive.rs — FormField, VariablePlaceholder
8. src/dom/element_id.rs — EID generation and validation

Requirements:

For element_id.rs:
- ElementId is a newtype wrapper around String
- ElementId::new(prefix: &str, timestamp_ms: u64, sequence: u32) -> ElementId
- The format must be: "{prefix}-{timestamp_ms}-{sequence:05}-{checksum}"
- checksum = first 4 hex chars of SHA-256("{prefix}{timestamp_ms}{sequence}")
- ElementId::validate(s: &str) -> bool

For document.rs:
- DocumentState enum: Draft, Review, Signed, Certified
- DocumentState must implement valid_transitions() -> &[DocumentState]
- Document struct with: eid, element_type (always "Document"), title, locale, direction, document_state, page_count, pages: Vec<Page>
- Page struct with: eid, element_type (always "Page"), elements: Vec<SpdfElement>
- Document::new(title: &str, locale: &str) -> Document

For each element type, create a Rust struct with:
- All fields from the SPDF specification (Section 6 of the spec document)
- Universal properties as a separate UniversalProps struct embedded via #[serde(flatten)]
- UniversalProps: eid, element_type, version, created_at, modified_at, style_id, integrity_hash, locked, visible, accessible_label, custom_data
- All financial values (unit_price, total, tax_amount, etc.) must be String type, never f64
- All timestamps must be chrono::DateTime<chrono::Utc>

For the SpdfElement enum:
- Must be an enum with a variant for every element type
- Must serialize/deserialize using serde's "element_type" field as the tag
- Use #[serde(tag = "element_type")] on the enum

Write tests that:
- Create a Document, add a Page, add a Heading and a Paragraph, serialize to JSON, deserialize back, verify round-trip
- Verify DocumentState::Draft.valid_transitions() contains Review but not Signed
- Verify ElementId generation produces correctly formatted IDs
```

---

## Day 8 — Document Validator and Semantic Parser

🎯 **Goal:** Can parse `semantic.json` into a typed DOM and validate it against all CONT- and E_ rules from the spec.

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 8.1 — Validator
**Machine:** Enterprise desktop · **Time:** 2 hours

```
In crates/spdf-core/src/validation/, implement the SPDF document validator.

Create:
1. src/validation/mod.rs
2. src/validation/rules.rs — all validation rules as individual functions
3. src/validation/report.rs — ValidationReport, ValidationError, ValidationWarning structs

Requirements for report.rs:
- ValidationSeverity enum: Fatal, Error, Warning
- ValidationIssue struct: code (String), severity, message, element_eid (Option<String>), element_type (Option<String>), property_path (Option<String>), property_name (Option<String>), spec_reference (String), remediation (String)
- ValidationReport struct: errors: Vec<ValidationIssue>, warnings: Vec<ValidationIssue>, is_valid: bool, validation_mode: ValidationMode, validated_at: DateTime<Utc>, spdf_version: String
- ValidationReport::is_valid() returns true only if zero Fatal and zero Error severity issues

Requirements for rules.rs — implement these validation functions:
- validate_duplicate_eids(document: &Document) -> Vec<ValidationIssue>
- validate_element_types(document: &Document) -> Vec<ValidationIssue>
- validate_required_properties(element: &SpdfElement) -> Vec<ValidationIssue>
- validate_financial_values(document: &Document) -> Vec<ValidationIssue>
  (checks all spdf:decimal fields are valid decimal strings, not floats)
- validate_timestamps(document: &Document) -> Vec<ValidationIssue>
  (checks all timestamps are ISO 8601 UTC — must end with Z)
- validate_document_state(document: &Document) -> Vec<ValidationIssue>
- validate_locked_element_modification(document: &Document, previous: Option<&Document>) -> Vec<ValidationIssue>

pub fn validate_document(document: &Document, mode: ValidationMode) -> ValidationReport
  runs all rules and returns a complete report
```

---

## Day 9 — PDF Render Layer Generation

🎯 **Goal:** Core engine can generate a basic PDF from an SPDF document (the render layer).

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 9.1 — PDF Renderer
**Machine:** Enterprise desktop · **Time:** 2.5 hours

```
In crates/spdf-core/src/render/, implement the PDF render layer generator using lopdf.

Create:
1. src/render/mod.rs
2. src/render/pdf_writer.rs — converts SPDF DOM to PDF 2.0 bytes
3. src/render/text.rs — text rendering helpers
4. src/render/page.rs — page layout helpers

Requirements for pdf_writer.rs:
- SpdfPdfRenderer struct
- fn render_to_pdf(document: &Document, layout: &LayoutData) -> Result<Vec<u8>, SpdfError>
- Must produce a valid, openable PDF file
- For MVP, render only these elements: Heading (as bold text), Paragraph (as normal text), HorizontalRule (as a line), Image (as embedded image placeholder), InvoiceHeader (as structured text block), LineItem (as a table row)
- Page size defaults to A4 (595.28 x 841.89 points)
- Default margins: 56.69pt (20mm) on all sides
- Use the Helvetica built-in PDF font for MVP (no custom font embedding yet — that's Week 2 Day 10)
- Each page in the SPDF document maps to one page in the PDF
- Text is positioned using the y-coordinate from layout.json

The goal is a working, openable PDF. Pixel-perfect rendering comes later.
Write a test that generates a PDF from a simple Document with one Heading and two Paragraphs,
writes it to a temp file, and verifies the file is non-empty and starts with %PDF.
```

---

## Day 10 — Integration Test: Full Round Trip

🎯 **Goal:** Create an SPDF invoice document in Rust, write it to a `.spdf` file, read it back, validate it, and render it as a PDF. End-to-end test passes.

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 10.1 — Integration Test Suite
**Machine:** Enterprise desktop · **Time:** 2 hours

```
Create an integration test in crates/spdf-core/tests/integration_test.rs that performs a complete SPDF round-trip.

The test must:
1. Build a complete SPDF invoice document in Rust:
   - Document title: "Test Invoice #001"
   - Locale: "en-IN"
   - One Page (A4)
   - InvoiceHeader with vendor (ACME Corp), client (Test Client), invoice_number: "INV-001", issue_date, due_date
   - LineItemTable with 2 LineItems:
     Item 1: "Software Development", qty: "10", unit: "hours", unit_price: "5000.00", total: "50000.00", currency: "INR"
     Item 2: "Code Review", qty: "5", unit: "hours", unit_price: "3000.00", total: "15000.00", currency: "INR"
   - PaymentTerms with terms: "Net 30", due_date matching invoice due_date
   - SignatureBlock with required_from role: "CLIENT", lock_on_sign: true

2. Serialize the document to JSON (semantic.json)
3. Create a minimal layout.json and styles.json (can be empty objects for now)
4. Render the document to PDF bytes using render_to_pdf()
5. Create audit.json with a CREATED entry
6. Build a complete SPDF container using SpdfWriter
7. Write the container bytes to a temp file: /tmp/test_invoice_001.spdf
8. Read the container back using read_container()
9. Verify all checksums match
10. Parse the semantic.json back into a Document struct
11. Validate the document — must return is_valid: true
12. Assert: invoice_number == "INV-001"
13. Assert: line items total == "65000.00" (sum of both items)
14. Assert: document state is Draft
15. Assert: the .spdf file exists and is a valid ZIP (check magic bytes)

Also write to /tmp/test_invoice_001.pdf the rendered PDF.
Print: "Integration test passed. SPDF file: X bytes. PDF file: Y bytes."
```

### 💻 Run on TUF F15 — Run Integration Test
```bash
cd ~/projects/spdf
git pull
cargo test -p spdf-core --test integration_test -- --nocapture
# Must print: "Integration test passed."

# Open the generated PDF to verify it opens correctly
# Copy to Windows accessible location:
cp /tmp/test_invoice_001.pdf /mnt/c/Users/YOUR_WINDOWS_USERNAME/Desktop/
# Open from Windows Desktop and verify it's a readable PDF
```

✅ **Week 2 Complete Verification:**
- `cargo test --workspace` — all tests pass
- `/tmp/test_invoice_001.spdf` exists and is a valid ZIP
- `/tmp/test_invoice_001.pdf` opens correctly in any PDF viewer
- `cargo clippy -- -D warnings` — zero warnings

---

# WEEK 3 — Python API: Core Endpoints + Conversion Pipeline

**Theme:** The REST API takes shape. Document generation and extraction endpoints go live.

**Weekly Goal:** `POST /v1/documents/generate` returns a working SPDF file. PDF can be uploaded and queued for conversion.

---

## Day 11 — PyO3 Bindings: Rust Core → Python

🎯 **Goal:** Python can call `validate_document()`, `generate_spdf()`, and `render_to_pdf()` from the Rust core engine.

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 11.1 — PyO3 Python Bindings
**Machine:** Enterprise desktop · **Time:** 2 hours

```
In crates/spdf-python/src/lib.rs, create PyO3 Python bindings for the spdf-core crate.

Expose these functions to Python:

1. validate_spdf(spdf_bytes: bytes) -> PyResult<dict>
   - Calls spdf_core::validation::validate_document()
   - Returns a Python dict matching the ValidationReport structure:
     { "is_valid": bool, "errors": [...], "warnings": [...], "validated_at": str }

2. generate_spdf(semantic_json: str, layout_json: str, styles_json: str, 
                 metadata_json: str, audit_json: str) -> PyResult<bytes>
   - Takes the five JSON layer strings
   - Builds a complete SPDF container using SpdfWriter
   - Returns raw bytes of the .spdf file

3. render_to_pdf(spdf_bytes: bytes) -> PyResult<bytes>
   - Reads the SPDF container
   - Calls render_to_pdf() on the DOM
   - Returns raw PDF bytes

4. parse_semantic(semantic_json: str) -> PyResult<dict>
   - Parses semantic.json string into Document struct
   - Returns it as a Python dict (via serde_json)

5. extract_invoice_data(spdf_bytes: bytes) -> PyResult<dict>
   - Reads SPDF container
   - Finds InvoiceHeader, LineItemTable, PaymentTerms elements
   - Returns structured dict with invoice data

The PyO3 module name must be "spdf_native".
Register all functions on a module named "spdf_native".

In services/api/core/spdf_engine.py, create a Python wrapper:
from spdf_native import validate_spdf, generate_spdf, render_to_pdf, parse_semantic, extract_invoice_data

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

### 💻 Run on TUF F15 — Build and Test Bindings
```bash
cd ~/projects/spdf/crates/spdf-python
maturin develop --release
python3.12 -c "import spdf_native; print(spdf_native.validate_spdf(b'invalid'))"
# Should return validation error dict, not crash
```

---

## Days 12–13 — Document Generation and Extraction Endpoints

🎯 **Goal:** `POST /v1/documents/generate` and `POST /v1/documents/{id}/extract` are fully functional.

⏱ **Time Budget:** 4.5 hours each day

---

### 🤖 Claude Code Task 12.1 — Document Generation Endpoint
**Machine:** Enterprise desktop · **Time:** 3 hours (across Days 12–13)

```
Implement the complete document generation endpoint in services/api/.

1. Create services/api/routers/documents.py with:

   POST /v1/documents/generate
   - Request body: GenerateDocumentRequest (from API contract SPDF-API-2025-001 Section 4.2)
   - Validates: template_id exists and is accessible to user, all required variables present
   - Calls SpdfEngine.generate() with the assembled JSON layers
   - Stores the .spdf file bytes in R2 (bucket: spdf-documents, key: users/{user_id}/documents/{doc_id}/{doc_id}.spdf)
   - Creates a documents row in PostgreSQL
   - Returns GenerateDocumentResponse with a signed download URL (15 min expiry)
   - Response must exactly match API contract Section 4.2

   GET /v1/documents/{document_id}
   - Fetches document metadata from PostgreSQL
   - Generates a fresh signed R2 download URL (1 hour expiry)
   - Returns DocumentDetailResponse

   GET /v1/documents
   - Returns paginated list of user's documents
   - Supports state, document_type, q (search) query params
   - Cursor-based pagination

   DELETE /v1/documents/{document_id}
   - Soft-delete only (sets deleted_at = NOW())
   - Returns 409 if document state is SIGNED or CERTIFIED

2. Create services/api/services/document_service.py:
   - async def generate(db, storage, user, request) -> Document
   - async def get_by_id(db, document_id, user_id) -> Document | None
   - async def list_for_user(db, user_id, filters, cursor, limit) -> tuple[list[Document], str | None]
   - async def soft_delete(db, document_id, user_id) -> None

3. Create services/api/services/storage_service.py:
   - Wraps boto3 S3 client configured for Cloudflare R2
   - async def upload(key: str, data: bytes, content_type: str) -> str
   - async def download(key: str) -> bytes
   - async def generate_signed_url(key: str, expires_in: int) -> str

4. Create services/api/routers/documents_extract.py:
   POST /v1/documents/{document_id}/extract
   - Fetches SPDF from R2
   - Calls SpdfEngine.extract()
   - Returns structured JSON matching API contract Section 4.6

All endpoints must:
- Use the get_current_user dependency for authentication
- Use get_db dependency for database access  
- Return errors matching the standard error format (Section 2 of API contract)
- Log every request/response with structlog
```

### 💻 Run on TUF F15 — Test Generation Endpoint
```bash
cd ~/projects/spdf
git pull
cd services/api
source .venv/bin/activate
doppler run -- uvicorn main:app --reload --port 8000

# In another terminal, test the endpoint:
curl -X POST http://localhost:8000/v1/documents/generate \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk_test_placeholder" \
  -d '{
    "template_id": "tmpl_invoice_standard_v1",
    "title": "Test Invoice #001",
    "locale": "en-IN",
    "data": {
      "INVOICE_NUMBER": "INV-2025-001",
      "VENDOR_NAME": "ACME Corp",
      "CLIENT_NAME": "Test Client",
      "LINE_ITEMS": [{"description": "Dev work", "qty": 10, "unit_price": "5000.00"}],
      "CURRENCY": "INR"
    },
    "options": { "include_render_layer": true }
  }' | python3.12 -m json.tool
```

---

## Days 14–15 — Celery Worker + PDF Upload + Conversion Queue

🎯 **Goal:** Upload a PDF → job created → Celery worker picks it up → calls Claude API → returns SPDF.

⏱ **Time Budget:** 4.5 hours each day

---

### 🤖 Claude Code Task 14.1 — Celery Worker and Conversion Pipeline
**Machine:** Enterprise desktop · **Time:** 3 hours

```
Implement the async PDF conversion pipeline.

1. Create services/worker/celery_app.py:
   - Celery app configured with Redis broker (REDIS_URL from env)
   - Result backend: Redis
   - Task serializer: json
   - Timezone: UTC

2. Create services/worker/tasks/conversion.py with task: pdf_to_spdf_task(job_id: str):
   
   The task must implement this exact pipeline:
   
   Step 1 - RETRIEVE: 
     Download PDF from R2 using input_r2_key from the job record
     Update job status to PROCESSING, started_at = NOW()
   
   Step 2 - TEXT EXTRACTION:
     Use pdfplumber to extract text blocks with positions: [{text, x, y, width, height, fontsize}]
     Extract images as PNG bytes
     Detect page count and dimensions
   
   Step 3 - COMPLEXITY DETECTION (for model selection per OQ-06):
     Count pages, tables, text density
     If pages <= 5 AND tables <= 3 AND no multi-currency: use claude-haiku-4-5
     Otherwise: use claude-sonnet-4-6
     Log which model was selected and why
   
   Step 4 - CLAUDE SEMANTIC EXTRACTION:
     Build this exact prompt:
     "You are an SPDF document converter. Analyze this document content and classify each text block into SPDF element types.
     
     Document text blocks (JSON): {text_blocks}
     
     Return ONLY a JSON array of elements. Each element must have:
     - element_type: one of [Heading, Paragraph, InvoiceHeader, LineItem, LineItemTable, PaymentTerms, Table, TableRow, TableCell, Image, SignatureBlock]
     - All properties matching the SPDF specification for that element type
     - A confidence score 0.0-1.0 for your classification
     
     For financial values: always use string decimal notation e.g. '1234.56' never floats.
     For dates: always use ISO 8601 UTC format.
     Return ONLY the JSON array, no other text."
     
     Call the Anthropic API using the selected model
     Parse the JSON response
     On failure: retry up to 3 times with exponential backoff (1s, 2s, 4s)
     On persistent failure: activate HEURISTIC FALLBACK (Step 4b)
   
   Step 4b - HEURISTIC FALLBACK:
     Font size > 14pt → Heading
     Position-aligned text in columns → Table detection
     Currency regex ($, £, ₹, €) + numeric → financial field
     Set confidence scores: 0.5-0.7 (lower than Claude baseline)
     Set conversion_method = "HEURISTIC"
   
   Step 5 - DOM ASSEMBLY:
     Assign ElementIds to all extracted elements
     Build Document → Page → elements hierarchy
     Serialize to semantic.json
     Create minimal layout.json from PDF coordinates
     Create default styles.json
   
   Step 6 - VALIDATION:
     Call SpdfEngine.validate() on assembled document
     Log any E_ errors
   
   Step 7 - CONTAINER ASSEMBLY:
     Use original PDF bytes as render.pdf (preserves visual fidelity)
     Call SpdfEngine.generate() to create the .spdf container
     Compute confidence report
   
   Step 8 - UPLOAD AND COMPLETE:
     Upload .spdf to R2
     Update documents table (INSERT new document record)
     Update conversion_jobs: status=COMPLETED, output_doc_id, result_data (confidence report), completed_at
     If webhook_url set: POST completion notification

3. In services/api/routers/upload.py, implement:
   POST /v1/documents/upload (multipart form)
   - Validate file is PDF (check magic bytes: %PDF)
   - Enforce 50MB size limit
   - Upload to R2 bucket: spdf-uploads/{job_id}/{filename}.pdf
   - Create conversion_jobs record with status=QUEUED
   - Enqueue pdf_to_spdf_task.delay(job_id)
   - Return 202 Accepted with job_id

4. In services/api/routers/jobs.py, implement:
   GET /v1/jobs/{job_id} — poll job status
   GET /v1/jobs — list user jobs (paginated)
   DELETE /v1/jobs/{job_id} — cancel QUEUED job only

5. requirements for worker (services/worker/requirements.txt):
   celery==5.3.6
   pdfplumber==0.10.3
   anthropic==0.18.0
   boto3==1.34.0
   structlog==24.1.0
   redis==5.0.0
   sqlalchemy[asyncio]==2.0.25
   asyncpg==0.29.0
```

### 💻 Run on TUF F15 — Test Full Conversion Pipeline
```bash
# Terminal 1: Start API
cd ~/projects/spdf/services/api
doppler run -- uvicorn main:app --reload --port 8000

# Terminal 2: Start Celery worker
cd ~/projects/spdf/services/worker
doppler run -- celery -A celery_app worker --loglevel=info --concurrency=2

# Terminal 3: Test upload
curl -X POST http://localhost:8000/v1/documents/upload \
  -H "Authorization: Bearer sk_test_placeholder" \
  -F "file=@/path/to/any/invoice.pdf" \
  -F "title=Test Conversion" | python3.12 -m json.tool

# Get the job_id from the response, then poll:
JOB_ID="job_f3e2d1c0-..."
curl http://localhost:8000/v1/jobs/$JOB_ID | python3.12 -m json.tool
# Poll every 2 seconds until status = COMPLETED
```

✅ **Week 3 Complete Verification:**
- PDF upload returns `202` with `job_id`
- After ~15 seconds, job status is `COMPLETED` with `confidence_report`
- `GET /v1/documents/{id}` returns document with `download_url`
- Download URL returns a valid `.spdf` file (ZIP with correct structure)

---

# WEEK 4 — Authentication, Rate Limiting, and Billing

**Theme:** The API becomes secure and commercial. Clerk JWT + API key auth, rate limiting via Redis, Stripe subscriptions.

**Weekly Goal:** Real users can sign up, get an API key, make authenticated calls, and have rate limits enforced.

---

## Day 16 — Clerk Authentication Integration

🎯 **Goal:** All API endpoints require valid auth. Clerk JWT and API key both work. Webhook creates user records.

⏱ **Time Budget:** 4.5 hours

---

### 🤖 Claude Code Task 16.1 — Authentication Middleware
**Machine:** Enterprise desktop · **Time:** 2.5 hours

```
Implement the complete authentication system in services/api/.

1. services/api/middleware/auth.py — Authentication middleware:

   Supports two auth paths simultaneously:
   
   Path 1 — API Key (Bearer sk_live_... or sk_test_...):
   - Extract key from Authorization: Bearer header
   - Look up key prefix (first 8 chars) in api_keys table
   - Verify full key against bcrypt hash (key_hash column)
   - If valid: load user record, attach to request.state.user
   - Update api_keys.last_used_at and use_count (async, non-blocking)
   
   Path 2 — Clerk JWT (Bearer eyJ...):
   - Detect: JWT starts with "eyJ" (base64 encoded JSON header)
   - Verify using Clerk's public key (fetch from https://YOUR_CLERK_DOMAIN/.well-known/jwks.json)
   - Cache JWKS for 1 hour in Redis
   - Extract user_id (sub claim) and load user record from database
   - Attach to request.state.user
   
   If neither path succeeds: return 401 with AUTH_REQUIRED or INVALID_TOKEN error code

2. services/api/dependencies.py — FastAPI dependencies:
   
   async def get_current_user(request: Request, db: AsyncSession = Depends(get_db)) -> User:
     Uses request.state.user set by middleware
     Raises 401 if not authenticated
   
   async def get_optional_user(request: Request) -> User | None:
     Returns user if authenticated, None if not (for public endpoints)

3. services/api/routers/webhooks.py — Clerk webhook handler:
   
   POST /v1/webhooks/clerk:
   - Verify Clerk webhook signature using CLERK_WEBHOOK_SECRET
   - On user.created: INSERT into users table (clerk_user_id, email, display_name, tier='FREE')
   - On user.updated: UPDATE users table
   - On user.deleted: soft-delete users table (set deleted_at)
   - Return 200

4. services/api/routers/account.py:
   
   GET /v1/account/api-key → returns prefix and created_at (never the full key)
   POST /v1/account/api-key/rotate:
   - Generate new API key: "sk_live_" + 26 random base62 chars using secrets.token_urlsafe
   - Bcrypt hash it (cost=12) using passlib
   - Update users.api_key_hash and api_key_prefix
   - Mark old key as revoked in api_keys table
   - Return the FULL key ONCE in the response with a warning
   - After this call, the key cannot be retrieved again
   
   GET /v1/account/usage → current day and month usage stats from usage_events

5. Apply get_current_user dependency to ALL /v1/ endpoints except:
   - GET /v1/health and GET /v1/health/ready (no auth)
   - POST /v1/validate (optional auth — 10 free calls per IP per day)
   - POST /v1/webhooks/* (signature-based auth, not user auth)
```

---

## Day 17 — Redis Rate Limiting

🎯 **Goal:** Rate limits enforced per user per endpoint family. 429 responses correct with headers.

---

### 🤖 Claude Code Task 17.1 — Rate Limiting
**Machine:** Enterprise desktop · **Time:** 2 hours

```
Implement Redis-backed rate limiting in services/api/middleware/rate_limit.py.

Requirements:
- Rate limits are per user_id per endpoint family per calendar day UTC
- Endpoint families and limits:
  FREE:       convert=10, generate=50, extract=100, sign=50, other=500 per day
  PRO:        convert=1000, generate=5000, extract=10000, sign=2000, other=50000 per day
  TEAM:       convert=10000, generate=50000, extract=100000, sign=20000, other=500000 per day
  ENTERPRISE: unlimited (skip rate limit check)

- Redis key format: "ratelimit:{user_id}:{endpoint_family}:{YYYY-MM-DD-UTC}"
- TTL: 86400 seconds (auto-expires at end of day)
- On rate limit exceeded: return HTTP 429 with:
  - Error body matching API contract Section 2.3 RATE_LIMIT_EXCEEDED code
  - Response headers: X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset, Retry-After

- Endpoint family mapping:
  /v1/documents/upload → "convert"
  /v1/documents/generate → "generate"
  /v1/documents/*/extract → "extract"
  /v1/documents/*/sign → "sign"
  everything else → "other"

- Implement as FastAPI dependency: check_rate_limit(endpoint_family: str)
- Apply to each router using Depends(check_rate_limit("generate")) etc.
- Rate limit check must be atomic (use Redis INCR + EXPIRE pipeline)
- Include X-RateLimit-* headers on EVERY response (not just 429s)
```

---

## Day 18 — Stripe Billing Integration

🎯 **Goal:** Stripe webhooks update subscription status. Tier is enforced on API calls. Pro upgrade flow works.

---

### 🤖 Claude Code Task 18.1 — Stripe Integration
**Machine:** Enterprise desktop · **Time:** 2.5 hours

```
Implement Stripe billing integration in services/api/.

1. services/api/routers/webhooks.py — add Stripe webhook handler:
   
   POST /v1/webhooks/stripe:
   - Verify Stripe webhook signature using stripe-python SDK and STRIPE_WEBHOOK_SECRET
   - Handle these events:
     customer.subscription.created → INSERT or UPDATE subscriptions table, UPDATE users.tier
     customer.subscription.updated → UPDATE subscriptions table, UPDATE users.tier  
     customer.subscription.deleted → UPDATE subscriptions status='CANCELLED', revert users.tier to 'FREE'
     invoice.paid → mark subscription active, UPDATE users.tier
     invoice.payment_failed → set subscription status='PAST_DUE'
   - Always return 200 (Stripe retries on non-200)

2. services/api/services/billing_service.py:
   
   async def get_or_create_stripe_customer(user: User) -> str:
     If user.stripe_customer_id is set: return it
     Else: call stripe.Customer.create(email=user.email, name=user.display_name)
     Store stripe_customer_id in users table and return it
   
   async def create_checkout_session(user: User, plan: str) -> str:
     Maps plan to Stripe Price ID (from env: STRIPE_PRICE_PRO, STRIPE_PRICE_TEAM)
     Calls stripe.checkout.Session.create()
     Returns checkout URL
   
   async def get_billing_portal_url(user: User) -> str:
     Returns Stripe billing portal URL for the user

3. services/api/routers/billing.py:
   
   POST /v1/billing/checkout → returns {checkout_url: str} for upgrading plan
   POST /v1/billing/portal → returns {portal_url: str} for managing subscription
   GET /v1/billing/subscription → returns current subscription status

4. usage_events recording:
   In every billable endpoint handler, after successful processing, call:
   await usage_service.record(db, user, event_type, document_id, job_id)
   This inserts a row into usage_events for billing reconciliation
```

---

## Days 19–20 — Templates System + End-to-End Integration Test

🎯 **Goal:** Default invoice template works. Full flow from signup → API key → generate invoice → download PDF works end to end.

---

### 🤖 Claude Code Task 19.1 — Templates System
**Machine:** Enterprise desktop · **Time:** 2 hours

```
Implement the templates system.

1. Add templates table migration (Alembic)
2. Create services/api/routers/templates.py:
   GET /v1/templates — list available templates (public + user's own)
   GET /v1/templates/{template_id} — get template with full variable_schema

3. Create a seed script scripts/seed_templates.py that inserts 3 default templates:
   
   Template 1 - tmpl_invoice_gst_india:
     name: "GST Invoice (India)"
     category: "Invoice"
     is_public: true
     variable_schema with required vars: INVOICE_NUMBER, ISSUE_DATE, DUE_DATE, 
       VENDOR_NAME, VENDOR_ADDRESS, VENDOR_GSTIN, CLIENT_NAME, CLIENT_ADDRESS, 
       CLIENT_GSTIN, LINE_ITEMS (array), CURRENCY (default: INR), TAX_SCHEME (default: GST_18)
   
   Template 2 - tmpl_invoice_simple:
     name: "Simple Invoice"
     category: "Invoice" 
     is_public: true
     Simpler variable schema without GST fields
   
   Template 3 - tmpl_invoice_us:
     name: "US Invoice"
     category: "Invoice"
     is_public: true
     variable_schema with US-specific fields

4. Update generate endpoint to look up template from database and use its variable_schema
   for validation before generating
```

✅ **Week 4 Complete Verification:**
```bash
# Full authenticated flow test:
# 1. Get API key (via account endpoint)
# 2. Generate invoice with API key
# 3. Verify rate limit headers on response
# 4. Exceed rate limit (make 51 requests as FREE tier) → get 429
# 5. Check usage_events table has records
```

---

# WEEK 5 — Document Signing, Validation, and Diff

**Theme:** The trust layer. Sign documents with X.509 certificates, verify integrity, semantic diff.

**Weekly Goal:** `POST /v1/documents/{id}/sign` produces a cryptographically signed SPDF with valid X.509 signature.

---

## Days 21–22 — Signing Engine in Rust + API Endpoint

### 🤖 Claude Code Task 21.1 — Rust Signing Engine

```
In crates/spdf-core/src/security/, implement the document signing engine.

1. src/security/mod.rs
2. src/security/signing.rs:
   
   fn compute_document_hash(
     semantic_json: &str, layout_json: &str, 
     styles_json: &str, metadata_json: &str
   ) -> Result<String, SpdfError>
   
   Steps:
   - Apply RFC 8785 JSON Canonicalization to each JSON string
     (sort keys lexicographically, remove insignificant whitespace)
   - Compute SHA-256 of each canonical form
   - Concatenate the four hashes in order: semantic + layout + styles + metadata
   - Return SHA-256 of the concatenated string as lowercase hex
   
   fn sign_document(
     spdf_bytes: &[u8],
     p12_bytes: &[u8],
     p12_password: &str,
     signer_name: &str,
     signer_email: Option<&str>,
     signer_title: Option<&str>
   ) -> Result<Vec<u8>, SpdfError>
   
   Steps:
   - Read SPDF container
   - Compute document_hash using compute_document_hash()
   - Load PKCS#12 certificate (use openssl crate)
   - Sign document_hash using RSA-PSS-SHA256
   - Create signature JSON matching the schema in spec Section 7.5
   - Add signature to container in signatures/signature_001.json
   - Add certificate PEM to signatures/certificate_001.pem
   - Transition document state from REVIEW to SIGNED
   - Append STATE_CHANGED entry to audit.json
   - Recompute all manifest checksums
   - Return updated SPDF bytes
   
   fn verify_document(spdf_bytes: &[u8]) -> Result<VerificationReport, SpdfError>
   
   VerificationReport: is_valid, signatures (Vec<SignatureVerification>), tamper_detected, audit_chain_valid

Add openssl = { version = "0.10", features = ["v102", "vendored"] } to Cargo.toml
```

---

## Days 23–24 — Semantic Diff Engine

### 🤖 Claude Code Task 23.1 — Diff Engine

```
In crates/spdf-core/src/diff/, implement semantic document diffing.

fn diff_documents(doc_a: &Document, doc_b: &Document) -> DiffReport

DiffReport struct:
- document_a_id: String
- document_b_id: String  
- summary: DiffSummary { elements_added, elements_removed, elements_modified, elements_unchanged }
- changes: Vec<DiffChange>

DiffChange struct:
- change_type: ChangeType (Added, Removed, Modified, Unchanged)
- eid: String
- element_type: String
- path: String (e.g. "pages[0].elements[2].rows[1].cells[3]")
- field: Option<String> (for Modified: which field changed)
- before: Option<serde_json::Value>
- after: Option<serde_json::Value>
- semantic_impact: SemanticImpact enum

SemanticImpact enum: FinancialValueChanged, LegalClauseChanged, SignatureBlockChanged, MetadataChanged, StructuralChange, ContentChange

Algorithm:
- Build EID → element map for each document
- For each EID in doc_a: check if exists in doc_b
  - If not: Removed
  - If yes: compare all properties, report Modified with before/after for changed fields
- For each EID in doc_b not in doc_a: Added

Expose via PyO3 as: diff_documents(spdf_a: bytes, spdf_b: bytes) -> dict
```

---

## Day 25 — Redaction Endpoint

### 🤖 Claude Code Task 25.1 — Cryptographic Redaction

```
Implement cryptographic erasure in Rust and expose via API.

In crates/spdf-core/src/security/redaction.rs:

fn redact_elements(
  spdf_bytes: &[u8],
  eids_to_erase: &[&str],
  reason: &str,
  erased_by_name: &str,
  replacement_text: Option<&str>,
  regenerate_render: bool
) -> Result<Vec<u8>, SpdfError>

Steps:
1. Parse SPDF container
2. For each EID: compute proof_hash = SHA-256(canonical_json(element))
3. Remove elements from DOM
4. If regenerate_render: regenerate render.pdf without those elements
5. Insert Redaction element at each erased position
6. Append ELEMENT_ERASED to audit.json with proof_hashes
7. Recompute manifest checksums
8. Return updated bytes

In services/api/routers/documents.py add:
POST /v1/documents/{document_id}/redact
```

✅ **Week 5 Complete Verification:**
- Sign a document → verify it → tamper with the SPDF file → verify again → `tamper_detected: true`
- Diff two versions of an invoice → changes list shows exact field-level changes
- Redact an element → download the SPDF → verify the element is gone from semantic.json

---

# WEEK 6 — React Studio Frontend

**Theme:** The web application. Upload PDFs, view their structure, export SPDF files.

**Weekly Goal:** Studio runs in browser. PDF can be uploaded, converted, viewed as element tree, downloaded.

---

## Day 26 — React App Scaffold + Clerk Auth

🎯 **Goal:** React app running at localhost:5173. Clerk sign-in/sign-up working.

---

### 🤖 Claude Code Task 26.1 — Studio Scaffold
**Machine:** Enterprise desktop · **Time:** 2 hours

```
Create a complete React + Vite + TypeScript application in apps/studio/.

Initialize with:
- React 18
- Vite 5
- TypeScript
- Tailwind CSS
- shadcn/ui component library
- React Router v6
- Zustand (state management)
- SWR (data fetching and polling)
- Clerk React SDK (@clerk/clerk-react)

Directory structure:
apps/studio/src/
├── main.tsx           (entry point with ClerkProvider)
├── App.tsx            (router setup)
├── routes/
│   ├── index.tsx      (dashboard — list of documents)
│   ├── upload.tsx     (PDF upload flow)
│   ├── document.tsx   (document viewer — split view: visual + element tree)
│   ├── settings.tsx   (account, API key, billing)
│   └── auth/
│       └── index.tsx  (Clerk sign-in/sign-up pages)
├── components/
│   ├── layout/
│   │   ├── Shell.tsx       (sidebar + main area layout)
│   │   └── Sidebar.tsx     (navigation links)
│   ├── documents/
│   │   ├── DocumentCard.tsx   (document list item)
│   │   └── UploadZone.tsx     (drag-drop PDF upload)
│   └── ui/                    (shadcn/ui re-exports)
├── lib/
│   ├── api-client.ts   (typed fetch client for SPDF API)
│   └── auth.ts         (Clerk hooks)
└── stores/
    ├── documentStore.ts
    └── uiStore.ts

Requirements:
1. ClerkProvider wraps the entire app (VITE_CLERK_PUBLISHABLE_KEY from env)
2. All /dashboard/* routes require authentication (<SignedIn> wrapper)
3. Unauthenticated users are redirected to /sign-in
4. api-client.ts uses Clerk's useAuth() to get JWT for API calls
5. The dashboard shows a list of documents from GET /v1/documents
6. The upload page has a drag-drop zone that calls POST /v1/documents/upload
7. After upload, poll GET /v1/jobs/{id} with SWR every 2 seconds until COMPLETED
8. Create vite.config.ts with:
   - Proxy /api → http://localhost:8000 (avoids CORS in dev)
   - Environment variable: VITE_API_URL, VITE_CLERK_PUBLISHABLE_KEY

Create apps/studio/.env.example:
VITE_CLERK_PUBLISHABLE_KEY=pk_test_...
VITE_API_URL=http://localhost:8000
```

### 💻 Run on TUF F15
```bash
cd ~/projects/spdf/apps/studio
npm install
npm run dev
# Open http://localhost:5173
```

---

## Days 27–28 — Document Viewer with Element Tree

🎯 **Goal:** Converted SPDF document shows as PDF on left, clickable element tree on right.

---

### 🤖 Claude Code Task 27.1 — Document Viewer
**Machine:** Enterprise desktop · **Time:** 3 hours

```
Build the SPDF document viewer in apps/studio/src/routes/document.tsx.

The viewer is a split-pane layout:
- LEFT PANE: PDF preview using react-pdf (renders render.pdf layer from the SPDF file)
- RIGHT PANE: Element tree inspector

For the LEFT PANE:
- Use react-pdf library to render the PDF layer
- The PDF bytes come from: download the .spdf file, extract render.pdf from the ZIP
- Show page navigation controls
- Highlight elements on hover when corresponding tree node is hovered

For the RIGHT PANE — Element Tree Inspector:
- Show the document tree as a collapsible tree component
- Each node shows: element_type icon, eid (truncated), key property value
- Clicking a node shows a PROPERTIES PANEL below the tree:
  All properties of that element displayed as key-value pairs
  Financial values shown with currency formatting
  Timestamps shown in local timezone
  confidence_score shown as a colored badge (green ≥0.9, yellow ≥0.7, red <0.7)
- Low-confidence elements are highlighted in yellow in the tree

For loading the document data:
- Fetch document metadata from GET /v1/documents/{id}
- Fetch structured data from POST /v1/documents/{id}/extract (with include_element_tree: true)
- Download the SPDF file bytes from the signed download_url
- Use JSZip to extract and parse semantic.json from the ZIP bytes

The element type → icon mapping:
  Document → FileText icon
  Heading → Heading1 icon (or H1/H2/H3 based on level)
  Paragraph → AlignLeft icon
  Table → Table icon
  InvoiceHeader → Receipt icon
  LineItem → ListItem icon
  SignatureBlock → PenLine icon
  Image → Image icon

Install required packages: react-pdf, jszip, lucide-react
```

---

## Day 29 — WASM Integration (Render Layer in Browser)

🎯 **Goal:** SPDF validation runs in the browser using the Rust WASM module. No server round-trip for local validation.

---

### 🤖 Claude Code Task 29.1 — WASM Browser Integration

```
Add WASM module loading to the Studio frontend.

1. In crates/spdf-wasm/src/lib.rs, expose these functions via wasm-bindgen:

   #[wasm_bindgen]
   pub fn validate_spdf_wasm(spdf_bytes: &[u8]) -> JsValue
     Calls the core validator
     Returns ValidationReport as JSON string, converted to JsValue via serde-wasm-bindgen

   #[wasm_bindgen]  
   pub fn get_element_tree(spdf_bytes: &[u8]) -> JsValue
     Parses semantic.json from the SPDF bytes
     Returns the full element tree as JSON

   #[wasm_bindgen]
   pub fn get_document_info(spdf_bytes: &[u8]) -> JsValue
     Returns: {document_id, title, page_count, state, element_count}

2. In apps/studio/src/lib/spdf-wasm.ts:

   let wasmModule: typeof import("../../../pkg/spdf_wasm") | null = null;
   
   export async function loadWasm() {
     if (wasmModule) return wasmModule;
     wasmModule = await import("../../../pkg/spdf_wasm");
     await wasmModule.default(); // Initialize the WASM module
     return wasmModule;
   }
   
   export async function validateSpdf(bytes: Uint8Array): Promise<ValidationReport> {
     const wasm = await loadWasm();
     return JSON.parse(wasm.validate_spdf_wasm(bytes));
   }

3. In the document viewer, use WASM validation:
   - After loading the SPDF bytes, call validateSpdf(bytes) locally in the browser
   - Show validation results in the properties panel
   - This runs entirely client-side — instant, no server cost

Build steps to add to justfile:
build-wasm: 
  wasm-pack build crates/spdf-wasm --target web --out-dir apps/studio/pkg
```

---

## Day 30 — Studio Polish + Export Flow

🎯 **Goal:** User can export a document as PDF directly from Studio. Upload-to-download full flow works in UI.

### 🤖 Claude Code Task 30.1 — Export and Polish

```
Add the export flow and final polish to the Studio.

1. Add export button to the document viewer toolbar:
   - "Download SPDF" → downloads the .spdf file from the signed URL
   - "Download PDF" → calls POST /v1/documents/{id}/render and downloads the PDF
   - "Copy as JSON" → copies semantic.json to clipboard

2. Add an upload progress indicator:
   - When PDF is uploading: show progress bar
   - When conversion is in progress: show animated steps matching the job's current_step:
     "Uploading" → "Extracting text" → "AI analysis" → "Building structure" → "Generating" → "Complete"
   - Show the model used (Haiku or Sonnet) as a small badge

3. Add confidence report display:
   - Overall confidence score shown as a circular progress indicator
   - List of low-confidence elements with their reasons
   - "Review" button for each low-confidence element that jumps to it in the tree

4. Add a settings page:
   - Show current API key prefix
   - "Rotate API Key" button (calls POST /v1/account/api-key/rotate, shows key once)
   - Show current plan and usage stats (from GET /v1/account/usage)
   - "Upgrade" button that calls POST /v1/billing/checkout and redirects to Stripe
```

✅ **Week 6 Complete Verification:**
- Studio loads at localhost:5173 with Clerk auth
- PDF drag-drop uploads, conversion tracks in real-time
- Document viewer shows element tree + PDF preview
- Export to PDF works
- WASM validation runs in browser console without errors

---

# WEEK 7 — TypeScript SDK + Python SDK

**Theme:** Publish developer SDKs. The first thing a developer installs to use SPDF.

**Weekly Goal:** `pip install spdf-sdk` and `npm install @spdf/sdk` both work and generate a valid SPDF invoice in under 25 lines of code.

---

## Days 31–32 — Python SDK

### 🤖 Claude Code Task 31.1 — Python SDK
**Machine:** Enterprise desktop · **Time:** 3 hours

```
Create the complete Python SDK in packages/spdf-python/.

This is a developer-facing package published to PyPI.
It wraps the PyO3 bindings with a clean, idiomatic Python API.

Directory structure:
packages/spdf-python/
├── spdf/
│   ├── __init__.py        (exports: Document, Page, elements, styles, signing)
│   ├── document.py        (Document class — the main entry point)
│   ├── page.py            (Page class)
│   ├── template.py        (Template loading and variable binding)
│   ├── signing.py         (Sign and verify documents)
│   ├── extraction.py      (Extract structured data)
│   ├── exceptions.py      (SpdfError hierarchy)
│   └── elements/
│       ├── __init__.py    (re-exports all element types)
│       ├── content.py     (Heading, Paragraph, Table, Image)
│       ├── domain.py      (InvoiceHeader, LineItem, LineItemTable, PaymentTerms)
│       └── trust.py       (SignatureBlock, Stamp)
├── tests/
│   ├── test_generation.py
│   ├── test_extraction.py
│   └── test_signing.py
├── pyproject.toml
└── README.md

The Document class must support this exact API (matching the PRD):
  from spdf import Document, Page
  from spdf.elements import Heading, Paragraph
  from spdf.elements.domain import InvoiceHeader, LineItemTable, PaymentTerms

  doc = Document(title="Invoice #INV-2025-001", locale="en-IN", document_type="Invoice")
  page = Page(size="A4")
  page.add(InvoiceHeader(invoice_number="INV-2025-001", ...))
  page.add(LineItemTable(currency="INR", tax_scheme="GST_18", items=[...]))
  page.add(PaymentTerms(terms="Net 30", ...))
  doc.add_page(page)
  doc.export.spdf("invoice.spdf")
  doc.export.pdf("invoice.pdf")
  doc.export.json("invoice.json")

All financial values in element constructors accept: str (decimal string), int, or float
Internally always converted to and stored as decimal strings ("1234.56")

Write a README.md showing the complete invoice generation example in 20 lines.
Write a getting_started.py example script.
```

---

## Days 33–34 — TypeScript SDK

### 🤖 Claude Code Task 33.1 — TypeScript SDK

```
Create the TypeScript SDK in packages/spdf-js/.

This is a developer-facing package published to npm as @spdf/sdk.
It uses the WASM build for browser support and Node.js.

packages/spdf-js/
├── src/
│   ├── index.ts              (public API exports)
│   ├── document.ts           (Document class — async, returns Promises)
│   ├── page.ts               (Page class)
│   ├── elements/
│   │   ├── index.ts
│   │   ├── content.ts        (Heading, Paragraph, Table, Image)
│   │   ├── domain.ts         (InvoiceHeader, LineItem, LineItemTable, PaymentTerms)
│   │   └── trust.ts          (SignatureBlock)
│   ├── wasm-loader.ts        (lazy WASM module initialization)
│   ├── node-adapter.ts       (Node.js I/O: fs, Buffer)
│   └── browser-adapter.ts   (Browser I/O: Uint8Array, Blob, download)
├── tests/
│   ├── generation.test.ts
│   └── extraction.test.ts
├── package.json
├── tsconfig.json
└── README.md

The API must be async (WASM is async) and match this usage:
  import { Document, Page, InvoiceHeader, LineItemTable } from "@spdf/sdk";

  const doc = await Document.create({ title: "Invoice #001", locale: "en-IN" });
  const page = new Page({ size: "A4" });
  page.add(new InvoiceHeader({ invoiceNumber: "INV-001", ... }));
  page.add(new LineItemTable({ currency: "INR", items: [...] }));
  doc.addPage(page);

  // Node.js
  const bytes = await doc.export.spdf();
  fs.writeFileSync("invoice.spdf", bytes);

  // Browser
  const blob = await doc.export.blob("spdf");
  const url = URL.createObjectURL(blob);

Full TypeScript types for all element constructors.
100% type coverage — no any types in public API.
```

---

## Day 35 — SDK Integration Tests + README Polish

🎯 **Goal:** Both SDKs generate the same invoice and the SPDF files are byte-compatible.

### 💻 Run on TUF F15
```bash
# Python SDK test
cd packages/spdf-python
pip install -e .
python tests/getting_started.py
# Must produce: invoice.spdf and invoice.pdf

# TypeScript SDK test  
cd packages/spdf-js
npm install && npm run build
node tests/getting_started.js
# Must produce: invoice.spdf

# Cross-SDK compatibility: validate Python SDK output with TypeScript SDK
# Both files must pass spdf validate
```

---

# WEEK 8 — Production Infrastructure + CI/CD

**Theme:** Deploy to Railway. GitHub Actions CI. Everything automated.

**Weekly Goal:** Push to `main` → tests run → deploys to Railway → API live at `api.spdf.dev`.

---

## Day 36 — Domain + DNS Setup

### 🖐 Manual Action 36.1 — Register Domain and Configure DNS
**Machine:** Enterprise desktop browser · **Time:** 1 hour

1. Go to [Cloudflare Registrar](https://www.cloudflare.com/products/registrar/) (best price, DNS included)
2. Register: `spdf.dev` (~$10/year)
3. In Cloudflare DNS, plan these records (configure after Railway deploy):
   - `api.spdf.dev` → Railway API service
   - `studio.spdf.dev` → Vercel Studio frontend
   - `docs.spdf.dev` → Documentation site
   - `spdf.dev` → GitHub Pages (spec landing page)

---

## Days 37–38 — GitHub Actions CI Pipeline

### 🤖 Claude Code Task 37.1 — CI/CD Pipeline

```
Create GitHub Actions workflows in .github/workflows/.

1. .github/workflows/ci.yml — runs on every PR and push to main:

   Jobs (run in parallel):

   rust-core:
   - runs-on: ubuntu-latest
   - steps: checkout, install Rust stable, rust-cache, cargo test --workspace, cargo clippy -- -D warnings, cargo audit, build WASM (wasm-pack build), build PyO3 (maturin build --release)
   - Upload artifacts: WASM package and PyO3 wheel

   python-backend:
   - runs-on: ubuntu-latest
   - needs: rust-core
   - services: postgres:15, redis:7
   - steps: checkout, download rust artifacts, setup Python 3.12, install deps, ruff check, mypy, pytest --cov

   frontend:
   - runs-on: ubuntu-latest
   - needs: rust-core
   - steps: checkout, download WASM artifact, setup Node 20, npm ci, eslint, tsc --noEmit, npm test, npm run build

   deploy (only on push to main, after all tests pass):
   - needs: [rust-core, python-backend, frontend]
   - steps: railway deploy API service, railway deploy worker service
   - Use secrets: RAILWAY_TOKEN

2. .github/workflows/release.yml — runs on version tag push (v*):
   - Build and publish Python SDK to PyPI
   - Build and publish TypeScript SDK to npm
   - Create GitHub Release with changelog

3. .github/workflows/security.yml — runs weekly:
   - cargo audit (Rust dependency vulnerabilities)
   - pip-audit (Python dependency vulnerabilities)
   - npm audit (Node dependency vulnerabilities)

All workflows must use GitHub Actions cache for:
- Rust build artifacts (Swatinem/rust-cache@v2)
- pip packages
- npm packages
```

---

## Days 39–40 — Railway Deployment

### 🖐 Manual Action 39.1 — Configure Railway Services
**Machine:** TUF F15 or enterprise browser · **Time:** 2 hours

1. Go to [railway.app](https://railway.app) → New Project → Deploy from GitHub
2. Connect your GHE repository

**Create API service:**
1. In Railway project → New Service → GitHub Repo
2. Settings:
   - Root directory: `services/api`
   - Build command: `pip install -r requirements.txt`
   - Start command: `sh -c 'alembic upgrade head && uvicorn main:app --host 0.0.0.0 --port $PORT'`
   - Health check path: `/v1/health`
3. Add environment variables from Doppler:
   - In Doppler: `doppler secrets download --format env --config prd > railway_vars.txt`
   - In Railway: Variables → paste all from the file
   - Delete `railway_vars.txt` immediately
4. Add custom domain: `api.spdf.dev`

**Create Worker service:**
1. New Service → GitHub Repo (same repo)
2. Settings:
   - Root directory: `services/worker`
   - Start command: `celery -A celery_app worker --loglevel=info --concurrency=4`
3. Same environment variables as API service

**Deploy:**
```bash
# Install Railway CLI on TUF F15
npm install -g @railway/cli
railway login
railway link  # link to your project

# Deploy
railway up --service spdf-api
railway up --service spdf-worker
```

**Verify:**
```bash
curl https://api.spdf.dev/v1/health
# Must return: {"status": "ok", ...}
```

---

# WEEK 9 — Vercel Studio Deploy + Observability

**Theme:** Studio deployed to production. Logging, error tracking, and uptime monitoring configured.

---

## Day 41 — Vercel Studio Deployment

### 🖐 Manual Action 41.1 — Deploy Studio to Vercel

1. Go to [vercel.com](https://vercel.com) → New Project → Import from GitHub
2. Select your GHE repo → set Root Directory to `apps/studio`
3. Framework: Vite
4. Build command: `npm run build`
5. Output directory: `dist`
6. Environment variables:
   - `VITE_CLERK_PUBLISHABLE_KEY` = your Clerk publishable key
   - `VITE_API_URL` = `https://api.spdf.dev`
7. Add custom domain: `studio.spdf.dev`
8. Deploy → wait ~2 minutes → visit `https://studio.spdf.dev`

---

## Days 42–43 — Observability Stack

### 🤖 Claude Code Task 42.1 — Structured Logging and Monitoring

```
Implement complete observability in services/api/ and services/worker/.

1. services/api/middleware/logging.py — structured request logging:
   Use structlog to emit JSON log lines for every request:
   {
     "level": "info",
     "event": "request.completed",
     "request_id": "req_...",
     "method": "POST",
     "path": "/v1/documents/generate",
     "status_code": 201,
     "duration_ms": 847,
     "user_id": "user_...",
     "tier": "PRO",
     "document_id": "spdf-...",
     "timestamp": "2025-03-15T09:30:01.288Z"
   }
   
   Redact these fields from logs automatically (replace value with "[REDACTED]"):
   - Any key containing: "key", "secret", "token", "password", "hash"
   - Authorization header value

2. Sentry integration:
   In main.py startup: sentry_sdk.init(dsn=config.sentry_dsn, traces_sample_rate=0.1)
   Every unhandled exception automatically sent to Sentry with request context

3. Add /v1/health/ready to check all dependencies:
   - Database: run SELECT 1
   - Redis: run PING
   - R2: run HeadBucket on spdf-documents
   - Anthropic API: check ANTHROPIC_API_KEY is set (don't make a real API call)
   Return degraded (not 503) if Anthropic is unavailable — heuristic fallback covers it

4. services/worker/tasks/conversion.py — add job progress logging:
   After each pipeline step, update job.current_step and job.progress in database
   Also update Redis cache: SET job:status:{job_id} {status, progress, step} EX 3600
   This makes the Studio progress indicator work in real-time

5. Add these Axiom log shipping lines to the Railway service (as env var):
   LOG_DRAIN_URL = your Axiom HTTP endpoint
   In main.py: configure structlog to ship to this URL in production
```

---

## Days 44–45 — Security Hardening

### 🤖 Claude Code Task 44.1 — Security Hardening

```
Add security hardening to services/api/.

1. Input validation:
   - Maximum file upload size: 50MB (enforce in middleware before saving to R2)
   - Maximum JSON body size: 1MB
   - Validate PDF magic bytes (%PDF) before processing
   - Sanitize all template variable values (strip HTML tags, limit string lengths)

2. SSRF prevention for webhooks:
   In webhook_url validation, block these IP ranges:
   - 127.0.0.0/8 (localhost)
   - 10.0.0.0/8 (private)
   - 172.16.0.0/12 (private)
   - 192.168.0.0/16 (private)
   - 169.254.0.0/16 (link-local / AWS metadata)
   Only allow https:// scheme, block http://

3. Rate limit on auth endpoints:
   POST /v1/account/api-key/rotate: max 5 per hour per user

4. Add security headers middleware:
   X-Content-Type-Options: nosniff
   X-Frame-Options: DENY
   X-XSS-Protection: 1; mode=block
   Referrer-Policy: strict-origin-when-cross-origin
   Content-Security-Policy: default-src 'self'

5. SQL injection prevention audit:
   Review all database queries — ensure 100% use SQLAlchemy ORM or parameterized queries
   Zero raw SQL strings with user input

6. Dependency pinning:
   Generate requirements.txt with exact pinned versions using pip freeze
   Add pip-audit to CI (already in security.yml)
```

---

# WEEK 10 — Testing Suite

**Theme:** Comprehensive test coverage. Nothing ships without tests passing.

---

## Days 46–48 — Test Suite

### 🤖 Claude Code Task 46.1 — Comprehensive Test Suite

```
Create a comprehensive test suite for the entire SPDF platform.

1. crates/spdf-core/tests/ — Rust tests:
   
   integration_tests.rs (already exists — expand it):
   - test_complete_invoice_generation — full round-trip (already written)
   - test_signing_and_verification — sign, verify, tamper, verify again
   - test_cryptographic_erasure — redact element, verify it's gone, verify proof_hash
   - test_semantic_diff — create two invoice versions, diff, verify changes detected
   - test_forward_compatibility — parse SPDF with unknown element type, verify preserved
   - test_validation_all_error_codes — trigger every E_ and F_ error code
   - test_audit_chain_integrity — verify chain, break it, verify detection
   
   fuzz_tests.rs:
   - fuzz_container_parser — feed random bytes to read_container, must never panic
   - fuzz_json_parser — feed random JSON to parse_semantic, must never panic

2. services/api/tests/ — Python API tests:
   
   test_documents.py:
   - test_generate_invoice — generate document, verify response schema
   - test_generate_missing_variable — verify 422 TEMPLATE_VARIABLE_MISSING
   - test_download_document — generate then download, verify it's valid SPDF
   - test_extract_invoice_data — generate invoice, extract, verify all fields
   - test_delete_signed_document — verify 409 error
   
   test_auth.py:
   - test_missing_auth_header — verify 401 AUTH_REQUIRED
   - test_invalid_api_key — verify 401 INVALID_API_KEY
   - test_valid_api_key — verify authenticated response
   - test_api_key_rotation — rotate key, verify old key fails, new key works
   
   test_rate_limiting.py:
   - test_rate_limit_enforced — exceed FREE tier limit, verify 429
   - test_rate_limit_headers — verify X-RateLimit-* on every response
   - test_pro_tier_higher_limit — verify PRO has higher limits
   
   test_conversion.py:
   - test_upload_pdf — upload PDF, verify 202 + job_id
   - test_job_polling — mock Celery, verify job status transitions
   - test_invalid_file_type — upload .txt, verify 400 UNSUPPORTED_FILE_TYPE
   - test_file_too_large — upload 51MB file, verify 413

3. apps/studio/tests/ — Frontend tests:
   
   Use Vitest + React Testing Library:
   - DocumentCard.test.tsx — renders document with correct state badge
   - UploadZone.test.tsx — drag-drop fires correct API call
   - ElementTree.test.tsx — tree renders correctly, click selects node
```

---

## Days 49–50 — Performance Testing and Load Testing

### 🤖 Claude Code Task 49.1 — Performance Tests

```
Create performance benchmarks and load tests.

1. crates/spdf-core/benches/benchmarks.rs (using criterion):
   
   bench_parse_spdf: parse a 50-page SPDF document 100 times, target < 200ms p95
   bench_generate_invoice: generate a 2-page invoice 1000 times, target < 500ms p95
   bench_render_pdf: render a 2-page document to PDF 100 times, target < 1000ms p95
   bench_validate: validate a complex document 1000 times, target < 50ms p95
   bench_diff: diff two 20-page documents 100 times, target < 2000ms p95

2. scripts/load_test.py using httpx async:
   
   async def load_test_generate(concurrency: int, total_requests: int):
     Fire `concurrency` concurrent POST /v1/documents/generate requests
     Measure: p50, p95, p99 latency, error rate
     Target: p95 < 2000ms, error rate < 0.1%
   
   async def load_test_extract(concurrency: int, total_requests: int):
     Fire concurrent POST /v1/documents/{id}/extract requests
     Target: p95 < 1000ms

Run benchmarks and include results in a BENCHMARK.md file.
```

---

# WEEK 11 — GTM Preparation

**Theme:** Everything needed for a successful public launch. Docs, landing page, Product Hunt assets.

**Weekly Goal:** Documentation site live. GitHub spec repo public. Product Hunt draft submitted.

---

## Day 51 — Documentation Site

### 🤖 Claude Code Task 51.1 — Documentation with Mintlify

```
Set up the SPDF documentation site using Mintlify.

Create docs/ directory with this structure:
docs/
├── mint.json           (Mintlify config)
├── introduction.mdx    (What is SPDF? Getting started in 5 minutes)
├── quickstart.mdx      (pip install spdf → generate invoice → done)
├── concepts/
│   ├── format.mdx      (SPDF file format overview)
│   ├── dom.mdx         (Document Object Model)
│   └── security.mdx    (Signing, verification, redaction)
├── sdk/
│   ├── python.mdx      (Python SDK full reference)
│   └── javascript.mdx  (TypeScript SDK full reference)
├── api-reference/
│   └── (auto-generated from OpenAPI spec)
└── guides/
    ├── invoice-generation.mdx    (Generate your first GST invoice)
    ├── pdf-conversion.mdx        (Convert existing PDFs to SPDF)
    ├── ai-extraction.mdx         (Extract structured data from invoices)
    └── signing.mdx               (Digitally sign a document)

mint.json must configure:
- name: "SPDF Documentation"
- colors: primary blue
- navigation with all sections
- API reference pointing to https://api.spdf.dev/v1/openapi.json

The quickstart.mdx must demonstrate:
  pip install spdf-sdk

  from spdf import Document, Page
  from spdf.elements.domain import InvoiceHeader, LineItemTable

  doc = Document(title="My First Invoice", locale="en-IN", document_type="Invoice")
  page = Page(size="A4")
  page.add(InvoiceHeader(invoice_number="INV-001", vendor_name="My Company", client_name="Client"))
  page.add(LineItemTable(currency="INR", items=[
      {"description": "Consulting", "qty": "10", "unit_price": "5000.00"}
  ]))
  doc.add_page(page)
  doc.export.pdf("invoice.pdf")
  print("Invoice generated!")

Write all guides as if explaining to a developer who has never heard of SPDF.
Every guide must have a working code example.
```

---

## Day 52 — Landing Page + GitHub Spec Repository

### 🖐 Manual Action 52.1 — Publish the Format Specification

**Machine:** Enterprise desktop · **Time:** 1 hour

1. On GHE, create a new **public** repository: `spdf-spec`
   (This is separate from the private `spdf` platform repo)
2. Copy the SPDF Format Specification v1.0 document (SPDF-FORMAT-2025-001) as `SPEC.md`
3. Create a `README.md` that is the landing page for the spec:
   - What is SPDF
   - Why it exists (1993 vs 2025)
   - The five core properties table
   - Links to: SDK, API docs, Studio
   - Contributing section
   - CC BY 4.0 license badge
4. Add `LICENSE` file with CC BY 4.0 text
5. Add `CONTRIBUTING.md`

### 🤖 Claude Code Task 52.2 — Landing Page

```
Create a landing page for spdf.dev in a new directory apps/landing/.

Use plain HTML, CSS, and minimal JavaScript (no framework — fast load time).

The page must:
1. Hero section:
   - Headline: "The document format for the next 30 years"
   - Subheadline: "SPDF keeps every promise PDF made in 1993 — and adds everything 2025 demands."
   - Two CTAs: "Read the Spec" (→ GitHub) and "Try the Studio" (→ studio.spdf.dev)
   - A code snippet showing the 10-line Python invoice example

2. Problem section:
   - "PDF was designed in 1993 for 1993" 
   - Three pain points: AI can't read it, developers hate generating it, enterprises can't extract data

3. Solution section: the dual-layer architecture diagram (HTML/CSS, not an image)

4. Features grid: 6 cards — Visual fidelity, Semantic structure, AI-native, Cryptographic signing, True redaction, Developer-first

5. Code example tabs: Python SDK / TypeScript SDK / REST API showing invoice generation

6. Use cases section: B2B Invoices, Legal Contracts, Government Certificates

7. Footer: GitHub, Docs, Studio, Spec, License (CC BY 4.0)

Design: dark background (#0D1117), white text, blue accent (#2563EB).
Mobile responsive. No external dependencies except a Google Font (Inter).
Fast — target < 50KB total page weight.
```

---

## Days 53–54 — Product Hunt Assets + Hacker News Post

### 🤖 Claude Code Task 53.1 — Launch Content

```
Write the following launch content pieces:

1. HACKER_NEWS_POST.md — the "Show HN" post for Hacker News:
   Title: "Show HN: SPDF – An open replacement for PDF with a semantic layer"
   
   Write 400-500 words covering:
   - The problem (PDF is a photograph, not data)
   - What we built (the dual-layer architecture in plain English)
   - Why now (AI makes this urgent — enterprises spending $2-5B on PDF parsing)
   - Technical choices made and why (Rust core, ZIP container, JSON semantic layer)
   - What's open: spec (CC BY 4.0) + core engine (MIT)
   - What's commercial: the platform (SPDF Studio + API)
   - Current state: what works, what's coming
   - Ask: try the Python SDK, open issues, star the spec repo
   
   Tone: technical, honest, direct. No marketing fluff. Hackers hate hype.

2. PRODUCT_HUNT_DESCRIPTION.md — the Product Hunt listing:
   Tagline (60 chars max): "PDF was designed in 1993. SPDF is designed for today."
   
   Description (260 chars): punchy version of the value prop
   
   First comment (400 words): maker comment explaining the technical choices,
   what problem you personally experienced that led to building this,
   and what you're most proud of technically

3. TWITTER_THREAD.md — 8-tweet launch thread:
   Tweet 1: the hook (the 1993 vs 2025 problem)
   Tweets 2-4: the problem in concrete terms (invoice processing costs, AI parsing costs)
   Tweet 5: the solution (dual-layer architecture)
   Tweet 6: the code example (Python SDK in 10 lines)
   Tweet 7: what's open vs commercial
   Tweet 8: CTA (link to spec + Studio)

4. DEV_TO_POST.md — full technical article for Dev.to (1500 words):
   "I got tired of PDF so I built a replacement"
   Deep dive into the architecture, the Rust core engine, the ZIP container trick,
   why the semantic layer is separate from layout, and why AI makes this necessary now.
```

---

## Day 55 — Smoke Tests + Launch Checklist

### 🤖 Claude Code Task 55.1 — Smoke Test Script

```
Create scripts/smoke_test.py — a comprehensive production smoke test.

The script must test the LIVE production API (https://api.spdf.dev):

1. Health check: GET /v1/health → verify status: "ok"
2. Auth rejected: GET /v1/documents without auth → verify 401 AUTH_REQUIRED
3. Generate invoice: POST /v1/documents/generate with test data → verify 201
4. Download SPDF: download from the returned URL → verify ZIP magic bytes
5. Extract data: POST /v1/documents/{id}/extract → verify InvoiceHeader fields
6. Render PDF: POST /v1/documents/{id}/render → verify %PDF magic bytes
7. Rate limit headers: verify X-RateLimit-* on every response
8. Validate SPDF: POST /v1/validate with the downloaded file → verify is_valid: true
9. Delete document: DELETE /v1/documents/{id} → verify 204

Print a clear pass/fail for each test.
Exit code 0 if all pass, 1 if any fail.
Total runtime target: < 30 seconds.

Run as: python scripts/smoke_test.py --env production --api-key $TEST_API_KEY
```

---

# WEEK 12 — Launch Week

**Theme:** Public launch execution. Everything is ready. This week is about distribution, not code.

---

## Day 56 — Pre-Launch Final Checks

### 🖐 Manual Action 56.1 — Pre-Launch Checklist
**Machine:** Both · **Time:** Full day

Run through this checklist completely before launching:

**Technical:**
- [ ] `python scripts/smoke_test.py --env production` — all 9 tests pass
- [ ] `https://api.spdf.dev/v1/health` — returns `{"status": "ok"}`
- [ ] `https://studio.spdf.dev` — loads, Clerk auth works, can upload a PDF
- [ ] `pip install spdf-sdk && python getting_started.py` — generates a valid invoice
- [ ] `npm install @spdf/sdk && node getting_started.js` — generates a valid invoice
- [ ] `https://docs.spdf.dev` — documentation site loads completely
- [ ] `https://spdf.dev` — landing page loads, all links work
- [ ] GHE spec repo is public: `github.com/YOUR_USERNAME/spdf-spec`
- [ ] Error rate on Sentry: 0 active issues
- [ ] Railway services: both API and Worker showing healthy
- [ ] Stripe: test payment flow works end-to-end (use Stripe test card 4242 4242 4242 4242)

**Content:**
- [ ] Hacker News post draft reviewed and ready to submit
- [ ] Product Hunt listing draft complete (submit 3 days before for review)
- [ ] Dev.to article saved as draft
- [ ] Twitter thread draft ready

**Accounts:**
- [ ] Stripe in LIVE mode (switch from test mode — requires business details)
- [ ] Anthropic API production key set (not the dev key with $50 limit)
- [ ] Railway services updated with production environment variables from Doppler

---

## Day 57 — Submit Product Hunt + Hacker News

### 🖐 Manual Action 57.1 — Launch Submissions
**Machine:** Enterprise desktop · **Time:** 3 hours

**Product Hunt (submit evening before for next-day launch):**
1. Go to producthunt.com → Submit
2. Fill in listing using PRODUCT_HUNT_DESCRIPTION.md
3. Schedule for the next morning (12:01 AM PST)
4. Upload screenshots: Studio upload screen, element tree view, code examples

**Hacker News:**
1. Go to news.ycombinator.com → Submit
2. Title: "Show HN: SPDF – An open replacement for PDF with a semantic layer"
3. URL: link to the GHE spec repo
4. Post the text from HACKER_NEWS_POST.md as a comment on your own post immediately after submission

**Dev.to:**
1. Publish the technical article from DEV_TO_POST.md
2. Tags: #rust #python #opensource #pdf

---

## Days 58–60 — Community Response + Iteration

🎯 **Goal:** Respond to every comment and issue within 2 hours. Fix any critical bugs same day. Ship improvements daily.

**Daily pattern for Days 58–60:**

**Morning (enterprise desktop — 2 hours):**
- Respond to all HN comments, GitHub issues, Product Hunt comments
- Identify the 1–2 most-requested features or bugs
- Write fixes using Claude Code, push to GHE

**Afternoon (TUF F15 — 2 hours):**
- Pull, test fixes, push verified changes
- Deploy to production via `railway up`
- Tweet progress updates using the thread format

**What to track:**
- GitHub spec repo stars (target: 200 in week 1)
- Studio signups (target: 100 in week 1)
- API keys created (target: 50 in week 1)
- PyPI downloads (target: 500 in week 1)
- Any enterprise inbound inquiries (respond within 1 hour)

---

## Week 12 KPIs — Definition of a Successful Launch

| Metric | Target | Stretch |
|---|---|---|
| GHE spec repo stars | 200 | 500 |
| Studio registered users | 100 | 500 |
| Active SDK developers | 20 | 100 |
| API keys created | 50 | 200 |
| PyPI downloads | 500 | 2,000 |
| HN points | 100 | 300 |
| Product Hunt votes | 50 | 200 |
| MRR at end of Week 12 | $0 (still free) | First paying Pro user |
| Enterprise inbound conversations | 1 | 5 |

---

# Appendix A — Daily Git Workflow

Every working day follows this exact git discipline:

**On enterprise desktop (start of session):**
```bash
cd C:/Projects/spdf
git pull origin main
git checkout -b feature/day-XX-description   # new branch for each day's work
# ... write code with Claude Code ...
git add -A
git commit -m "feat: [description of what was built]"
git push origin feature/day-XX-description
# Create PR on GHE → merge to main
```

**On TUF F15 (verification session):**
```bash
cd ~/projects/spdf
git pull origin main
# ... run tests ...
# If fixes needed:
git add -A
git commit -m "fix: [description of fix]"
git push origin main  # or feature branch
```

**Commit message format:**
- `feat:` new functionality
- `fix:` bug fix
- `chore:` tooling, config, deps
- `test:` tests only
- `docs:` documentation only
- `refactor:` code change, no behavior change

---

# Appendix B — Claude Code Prompt Discipline

The quality of Claude Code output depends entirely on prompt quality. Follow these rules:

**Always include in every Claude Code prompt:**
1. The specific file paths you want created or modified
2. The exact function signatures you need
3. References to the relevant spec document section number
4. A concrete test case that must pass

**Always review before committing:**
1. Does the code compile? (Test on TUF F15 before pushing)
2. Does it match the API contract? (Check against SPDF-API-2025-001)
3. Does it match the DB schema? (Check against SPDF-DB-2025-001)
4. Are financial values stored as strings, never floats?
5. Are there any hardcoded secrets? (Should never be)

**When Claude Code produces compilation errors:**
1. Copy the COMPLETE error output (not just the last few lines)
2. Return to Claude Code: "The code you generated has these compilation errors: [paste full output]. Please fix them."
3. Never manually edit Rust compilation errors — let Claude Code fix them

---

# Appendix C — Environment Variable Reference

| Variable | Used By | Where Set |
|---|---|---|
| `DATABASE_URL` | API, Worker | Doppler |
| `REDIS_URL` | API, Worker | Doppler |
| `R2_ENDPOINT` | API, Worker | Doppler |
| `R2_ACCESS_KEY_ID` | API, Worker | Doppler |
| `R2_SECRET_ACCESS_KEY` | API, Worker | Doppler |
| `R2_BUCKET_DOCUMENTS` | API, Worker | Doppler |
| `R2_BUCKET_UPLOADS` | API, Worker | Doppler |
| `ANTHROPIC_API_KEY` | Worker | Doppler |
| `CLERK_SECRET_KEY` | API | Doppler |
| `CLERK_WEBHOOK_SECRET` | API | Doppler |
| `STRIPE_SECRET_KEY` | API | Doppler |
| `STRIPE_WEBHOOK_SECRET` | API | Doppler |
| `STRIPE_PRICE_PRO` | API | Doppler |
| `STRIPE_PRICE_TEAM` | API | Doppler |
| `RESEND_API_KEY` | API | Doppler |
| `SENTRY_DSN` | API, Worker | Doppler |
| `RAILWAY_TOKEN` | CI/CD | GitHub Secrets |
| `VITE_CLERK_PUBLISHABLE_KEY` | Studio | Vercel env |
| `VITE_API_URL` | Studio | Vercel env |

---

# Appendix D — Risk Register

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| Rust compilation errors stall progress | High | Medium | Budget 30 min per error round. Claude Code fixes all Rust errors — never manually edit |
| crates.io blocked on enterprise desktop | High | Low | Enterprise desktop never runs cargo. All Rust builds on TUF F15 only |
| Claude API costs exceed $50/month limit | Medium | Low | Raise limit in Anthropic console. Monitor cost dashboard weekly |
| GHE sync conflict between machines | Medium | Low | One machine pushes at a time. Always pull before starting a session |
| PyO3 build complexity | Medium | Medium | maturin develop handles it. If build fails, paste error to Claude Code |
| Stripe integration complexity | Low | Medium | Use Stripe's test mode end-to-end. Only switch to live mode on launch day |
| IT notices developer tools on enterprise desktop | Low | High | Enterprise desktop only runs VS Code + Git. No runtime tools, no builds |

---

*— End of SPDF 12-Week Development & GTM Plan —*
*SPDF-SPRINT-2025-001 · Version 1.0 · March 2025*
