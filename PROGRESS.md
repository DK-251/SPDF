# SPDF Development Progress

> Single source of truth for what's done, what's in progress, and what's next.
> Updated every snapshot. Maps directly to Dev Plan V2.

## Current State
- **Version:** 0.1.0-snapshot.1
- **Phase:** 1 — Foundation
- **Status:** Scaffold complete, first green build on TUF

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
| 6 | Container tests | TODO | — | Round-trip: create → write → read → validate |
| 7 | DOM serialization tests | TODO | — | All 25 elements serialize/deserialize correctly |
| 8 | Document validator | TODO | — | E_ and F_ error codes, structural rules |
| 9 | PDF renderer (basic) | TODO | — | Text, tables, images via lopdf |
| 10 | Integration test | TODO | — | Full round-trip: create → write → read → validate → render |

---

## Phase 2: Backend API (Weeks 3–5)

### Week 3: API Core

| Day | Task | Status | Snapshot | Notes |
|-----|------|--------|----------|-------|
| 11 | PyO3 bindings | TODO | — | |
| 12-13 | Document generation endpoint | TODO | — | |
| 14-15 | PDF upload + Celery + Claude conversion | TODO | — | |

### Week 4: Auth & Billing

| Day | Task | Status | Snapshot | Notes |
|-----|------|--------|----------|-------|
| 16 | Clerk JWT auth | TODO | — | |
| 17 | Redis rate limiting | TODO | — | |
| 18 | Stripe webhooks | TODO | — | |
| 19-20 | Templates + E2E test | TODO | — | |

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
