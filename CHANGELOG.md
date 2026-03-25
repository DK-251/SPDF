# SPDF Changelog

All notable changes tracked per snapshot version. Each snapshot represents one push-build-test cycle between Enterprise Desktop and ASUS TUF.

Format: `MAJOR.MINOR.PATCH-snapshot.N`
- **MAJOR:** Breaking format/API changes
- **MINOR:** New features or crate additions
- **PATCH:** Bug fixes, refinements
- **snapshot.N:** Increments every push-build cycle

---

## [0.1.0-snapshot.7] - 2026-03-26

### Added
- Account router (`api/app/routers/account.py`) with 3 endpoints:
  - `GET /api/v1/account/api-key` -- view current key prefix and metadata
  - `POST /api/v1/account/api-key/rotate` -- generate new key, returns full key once
  - `GET /api/v1/account/usage` -- today's usage per endpoint family with tier limits
- Rate-limit middleware (`api/app/middleware/rate_limit.py`):
  - Per-user, per-endpoint-family, per-calendar-day (UTC) quota enforcement
  - API key authentication via `Authorization: Bearer sk_...` header
  - `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` headers on all responses
  - ENTERPRISE tier unlimited; FREE/PRO/TEAM tiered limits
  - Health endpoint bypasses auth and rate limiting
- In-memory stores (`api/app/services/stores.py`): UserStore + RateLimitStore with clean interfaces for future DB/Redis swap
- API key service (`api/app/services/api_keys.py`): generation (`sk_{type}_{26 chars}`), bcrypt hashing, verification, test key seeding
- Pydantic schemas: `ApiKeyResponse`, `ApiKeyRotateResponse`, `UsageResponse`
- Error codes: `RATE_LIMIT_EXCEEDED` (429), `UNAUTHORIZED` (401)
- 18 account endpoint tests (`test_account.py`): key info, rotation, old key invalidation, usage tracking, auth failures, method guards
- 15 rate-limit tests (`test_rate_limit.py`): auth enforcement, header presence, quota exceeded 429, family isolation, enterprise unlimited, response structure
- `passlib[bcrypt]` dependency added to `api/pyproject.toml`

### Changed
- `conftest.py`: `client` fixture now includes auth headers by default; added `raw_client`, `auth_headers`, `_reset_stores` (autouse) fixtures
- `main.py`: wired RateLimitMiddleware + account router + test user seeding

### Fixed
- Replaced `passlib[bcrypt]` with direct `bcrypt>=4.0.0` -- passlib incompatible with bcrypt 4.x (`__about__` removed, strict 72-byte limit in bug detection)

### Build Target
- [x] just check -- ALL PASS (93 Rust + 61 Python tests)

---

## [0.1.0-snapshot.6] - 2026-03-25

### Added
- Unified `just check` script (`scripts/check.ps1`) — runs Rust (fmt + clippy + test) and Python (pip install + maturin develop + pytest) in one pass
- Structured `CHECK_RESULTS.md` output with pass/fail per step, failure details, and test summary counts
- 14 new API edge case tests (30 total): empty file, no file field, missing fields, wrong content type, non-invoice extract, financial string preservation, custom layers, method-not-allowed
- 3 new Rust validator tests: E_011 (redaction empty eid), LineItemTable E_007/E_008 coverage
- `sample_non_invoice_semantic` fixture for testing extract on plain documents

### Fixed
- Rewrote `check.ps1` for PowerShell compatibility: `[char]96` for markdown backticks, `Out-String` for stderr capture, `ArrayList` for step collection, null guards on `$LASTEXITCODE`
- Changed `api/pyproject.toml` build-backend from `setuptools.backends._legacy:_Backend` to `setuptools.build_meta` (legacy backend unavailable on TUF)
- `check.ps1` now creates `.venv` and sets `VIRTUAL_ENV` before Python steps (maturin develop requires a virtualenv)
- Deleted obsolete `scripts/check-log.ps1`

### Changed
- `just check` now runs full CI (Rust + Python) instead of Rust-only bare commands
- Removed separate `.log` files — all output captured inline in CHECK_RESULTS.md

### Build Target
- [x] just check — ALL PASS (93 Rust + 32 Python tests)

---

## [0.1.0-snapshot.5] - 2026-03-25

### Added
- FastAPI application entry point (`api/app/main.py`) with CORS middleware
- Document router (`api/app/routers/documents.py`) with 6 endpoints:
  - `POST /api/v1/documents/generate` — build .spdf from layer JSON dicts
  - `POST /api/v1/documents/validate` — validate uploaded .spdf container
  - `POST /api/v1/documents/render` — render .spdf semantic layer to PDF
  - `POST /api/v1/documents/parse` — parse and validate semantic JSON
  - `POST /api/v1/documents/extract` — extract structured invoice data from .spdf
  - `GET /api/v1/health` — health check with engine version
- Pydantic request/response schemas (`api/app/schemas.py`)
- Structured error handling (`api/app/errors.py`) — maps engine errors to HTTP status codes
- `api/pyproject.toml` — project dependencies (fastapi, uvicorn, pydantic, python-multipart)
- Test suite: 16 endpoint tests (`api/tests/test_documents.py`) with shared fixtures (`conftest.py`)
- Upload size enforcement (100 MB limit) and ZIP magic byte validation

### Build Target
- [x] pip install -e ".[dev]" + maturin develop
- [x] pytest tests/ -v
- [ ] uvicorn app.main:app --port 8000

---

## [0.1.0-snapshot.4] - 2026-03-25

### Added
- `spdf-python` PyO3 bindings: 5 functions exposed as `spdf_native` Python module
  - `validate_spdf()` — validate container + document, returns combined report as JSON
  - `generate_spdf()` — build .spdf container from layer JSONs with auto-rendered PDF
  - `render_to_pdf()` — extract semantic layer and render to PDF bytes
  - `parse_semantic()` — parse and validate semantic JSON, return Document as JSON
  - `extract_invoice_data()` — extract InvoiceHeader, LineItemTable, PaymentTerms as structured dict
- `api/app/services/spdf_engine.py` — Python `SpdfEngine` wrapper class over `spdf_native`
- Binding logic tests (20 tests): validate, generate, render, parse, extract paths + JSON output shape
- Phase 1 regression tests (6 tests): checksum integrity, state machine, element serde, validator codes, PDF output

### Fixed
- `MAX_DECOMPRESSION_RATIO` raised from 100 to 1000 — fixes `large_layer_round_trip` false positive (real ZIP bombs exceed 1M:1)
- Removed unused `DocumentId` import in `spdf-python`
- Added `#![allow(clippy::useless_conversion)]` in `spdf-python` — suppresses pyo3 proc macro lint
- Fixed rustfmt formatting in `binding_logic_tests.rs` (7 locations) and `lib.rs` (2 locations)

### Build Target
- [x] cargo fmt --check
- [x] cargo clippy -- -D warnings
- [x] cargo test --workspace

---

## [0.1.0-snapshot.2] - 2026-03-25

### Added
- `spdf-core` container round-trip tests (13 tests: write/read, checksums, assets, corruption)
- `spdf-core` DOM serialization tests (30 tests: all 18 element variants, types, state transitions)
- `spdf-validator` full implementation: 13 validation rules (E_001–E_013, F_001–F_005)
- `spdf-validator` tests (20 tests: every error code, manifest validation, report helpers)
- `spdf-renderer` PDF generation via lopdf (headings, paragraphs, tables, invoices, signatures)
- Integration test: create → validate → render PDF → write container → read → validate

### Build Target
- [ ] cargo build --workspace
- [ ] cargo test --workspace
- [ ] cargo clippy -- -D warnings

---

## [0.1.0-snapshot.1] - 2026-03-25

### Added
- Project monorepo scaffolded (Cargo workspace, 5 crates, API, Studio)
- `spdf-core` crate: error types, DOM (25 element types), container I/O, manifest
- `spdf-renderer`, `spdf-validator`, `spdf-python`, `spdf-wasm` crate stubs
- `scripts/setup-asus.sh` — one-time environment bootstrapper for ASUS TUF
- `scripts/build-status.sh` — generates BUILD_STATUS.md after each build cycle
- `justfile` with all task recipes
- `.gitignore`, `CLAUDE.md`, `CHANGELOG.md`, `VERSION`
- Spec docs archived to `docs/specs/`

### Build Target
- [ ] cargo build --workspace
- [ ] cargo test --workspace
- [ ] cargo clippy -- -D warnings

### Notes
- First snapshot. No build attempted yet. Pending ASUS TUF setup.
