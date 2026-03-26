# SPDF Development Progress

> Single source of truth for what's done, what's in progress, and what's next.
> Updated every snapshot. Maps directly to Dev Plan V2.

## Current State
- **Version:** 0.1.0-snapshot.8
- **Phase:** 2 — Backend API
- **Status:** Week 4 complete — JWT auth, billing, webhooks, templates all green on TUF

---

## Phase 1: Foundation (Weeks 1–2)

### Week 1: Environment & Scaffold

| Day | Task | Status | Snapshot | Notes |
|-----|------|--------|----------|-------|
| 1 | Dev environment setup | DONE | 0.1.0-s.1 | Two-machine workflow established |
| 2 | Git repo + config | DONE | 0.1.0-s.1 | GHE: github.com/DK-251/SPDF |
| 3 | Monorepo scaffold | DONE | 0.1.0-s.1 | 5 Rust crates, API, Studio dirs |
| 4 | Core types + DOM | DONE | 0.1.0-s.1 | 25 element types, state machine, error types |
| 5 | Container I/O | DONE | 0.1.0-s.1 | ZIP read/write with checksum + ZIP bomb protection |

### Week 2: Rust Core Engine

| Day | Task | Status | Snapshot | Notes |
|-----|------|--------|----------|-------|
| 6 | Container tests | DONE | 0.1.0-s.2 | 13 tests: round-trip, checksums, assets, corruption, edge cases |
| 7 | DOM serialization tests | DONE | 0.1.0-s.2 | 30 tests: all 18 element variants, types, state machine, full invoice |
| 8 | Document validator | DONE | 0.1.0-s.2 | 13 error codes (E_001–E_013, F_001–F_005), manifest validation |
| 9 | PDF renderer (basic) | DONE | 0.1.0-s.2 | Headings, paragraphs, tables, invoices, signatures via lopdf |
| 10 | Integration test | DONE | 0.1.0-s.2 | Full round-trip: create → validate → render → container → read → validate |

---

## Phase 2: Backend API (Weeks 3–5)

### Week 3: API Core

| Day | Task | Status | Snapshot | Notes |
|-----|------|--------|----------|-------|
| 11 | PyO3 bindings | DONE | 0.1.0-s.4 | 5 PyO3 functions + SpdfEngine wrapper + 20 binding logic tests + 6 regression tests |
| 12-13 | Document generation + extraction endpoints | DONE | 0.1.0-s.5 | FastAPI app, 6 endpoints, Pydantic schemas, error handling |
| 12-13 | Test wiring + edge cases + unified check | DONE | 0.1.0-s.6 | 32 API tests, 3 new validator tests, unified `just check` (all green on TUF) |
| 14-15 | Account endpoints + rate limiting | DONE | 0.1.0-s.7 | 3 account endpoints, rate-limit middleware, in-memory stores, 33 new tests |

### Week 4: Auth & Billing

| Day | Task | Status | Snapshot | Notes |
|-----|------|--------|----------|-------|
| 16 | JWT auth (dual: API key + JWT) | DONE | 0.1.0-s.8 | PyJWT, configurable secret/issuer, 12 tests |
| 17 | Subscription mgmt + billing endpoints | DONE | 0.1.0-s.8 | In-memory SubscriptionStore, 3 billing endpoints, 12 tests |
| 18 | Stripe webhook handler | DONE | 0.1.0-s.8 | Event dispatch, tier mutations, sig verify stub, 11 tests |
| 19-20 | Templates + E2E tests | DONE | 0.1.0-s.8 | CRUD with cursor pagination, 17 template + 8 E2E tests |

### Week 5: Advanced Features

| Day | Task | Status | Snapshot | Notes |
|-----|------|--------|----------|-------|
| 21-23 | Extraction, validation, redaction | TODO | — | |
| 24-25 | Signing & verification | TODO | — | |
| 26-27 | Semantic diff | TODO | — | |
| 28-30 | API hardening | TODO | — | |

---

## Phase 3: Studio Frontend (Weeks 6–7)
> Not started

## Phase 4: SDKs (Weeks 8–9)
> Not started

## Phase 5: Launch (Weeks 10–11)
> Not started

---

## Blockers & Decisions Log

| Date | Issue | Resolution | Snapshot |
|------|-------|------------|----------|
| 2026-03-25 | Zscaler blocks rustup.rs on enterprise desktop | Two-machine workflow: write here, build on TUF | 0.1.0-s.1 |
| 2026-03-25 | GitHub push protection blocked example Stripe keys | Sanitized to EXAMPLE_KEY_PLACEHOLDER | 0.1.0-s.1 |
| 2026-03-25 | LNK1104 msvcrt.lib missing on TUF | Installed MSVC C++ build tools | 0.1.0-s.1 |
| 2026-03-25 | zip crate v2.4 FileOptions needs type annotation | Changed to SimpleFileOptions | 0.1.0-s.1 |
| 2026-03-25 | large_layer_round_trip fails: decompression bomb false positive (ratio 461:1 vs max 100:1) | Raised MAX_DECOMPRESSION_RATIO to 1000 (real bombs exceed 1M:1) | 0.1.0-s.4 |
| 2026-03-25 | clippy: unused DocumentId import + pyo3 useless_conversion lint | Removed import, added crate-level #![allow] for pyo3 macro issue | 0.1.0-s.4 |
| 2026-03-25 | rustfmt: binding_logic_tests.rs + lib.rs formatting mismatches | Matched exact rustfmt output for all flagged lines | 0.1.0-s.4 |
| 2026-03-25 | check.ps1: PS backtick escapes, broken `*>` redirection, array flattening | Rewrote with `[char]96`, `Out-String` capture, `ArrayList` collections | 0.1.0-s.6 |
| 2026-03-25 | pyproject.toml: `setuptools.backends._legacy` unavailable on TUF | Changed build-backend to `setuptools.build_meta` | 0.1.0-s.6 |
| 2026-03-25 | maturin develop: no virtualenv found on TUF global Python | check.ps1 now creates `.venv` and sets `VIRTUAL_ENV` before Python steps | 0.1.0-s.6 |
| 2026-03-26 | passlib incompatible with bcrypt 4.x (`__about__` removed, 72-byte strict limit) | Replaced `passlib[bcrypt]` with direct `bcrypt>=4.0.0` API | 0.1.0-s.7 |
