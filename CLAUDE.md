# SPDF Project Guide

## What is SPDF?
Structured PDF — a next-gen document format that wraps PDF's visual fidelity with a typed JSON semantic layer. Files use the `.spdf` extension (ZIP archives internally).

## Repository Layout
```
SPDF/
├── crates/                  # Rust workspace
│   ├── spdf-core/           # Types, DOM, container I/O, manifest
│   ├── spdf-renderer/       # PDF generation via lopdf
│   ├── spdf-validator/      # Schema + structural validation
│   ├── spdf-python/         # PyO3 bindings for FastAPI
│   └── spdf-wasm/           # WASM bindings for Studio
├── api/                     # FastAPI backend (Python 3.12)
│   ├── app/
│   │   ├── routers/         # Endpoint definitions
│   │   ├── services/        # Business logic
│   │   ├── models/          # SQLAlchemy models
│   │   ├── middleware/       # Auth, rate limiting
│   │   └── workers/         # Celery task definitions
│   └── tests/
├── studio/                  # React 18 + Vite frontend
│   └── src/
├── scripts/                 # Setup, build-status, utilities
├── docs/
│   ├── specs/               # Archived specification documents
│   └── guides/              # Developer guides
├── .build-results/          # Build output from ASUS TUF (gitignored logs)
├── Cargo.toml               # Rust workspace root
├── justfile                 # Task runner (just <recipe>)
└── CLAUDE.md                # This file
```

## Two-Machine Workflow
- **Enterprise Desktop:** Code authoring via Claude Code. Push to GHE.
- **ASUS TUF Laptop:** Pull, build, test. Push results via `just status`.
- **Sync contract:** `.build-results/BUILD_STATUS.md` tracks pass/fail state.

## Coding Standards
- **Rust:** `cargo fmt` + `cargo clippy -- -D warnings`. No `unwrap()` in library code.
- **Python:** Black formatter, type hints on all public functions, no `# type: ignore`.
- **TypeScript:** ESLint + Prettier. Strict mode.
- **Naming:** snake_case (Rust/Python), camelCase (TypeScript). Files match module names.
- **Financial values:** Always decimal strings (`"2500.00"`), never floats.
- **Comments:** Only where logic is non-obvious. No boilerplate/filler comments.
- **Error handling:** Typed errors (`SpdfError`), never string-only errors.

## Key Commands (on ASUS TUF)
```bash
just build          # cargo build --workspace
just test           # cargo test --workspace
just lint           # cargo clippy
just fmt            # cargo fmt
just check          # fmt-check + lint + test
just status         # generate BUILD_STATUS.md
just api-dev        # uvicorn dev server
just studio-dev     # vite dev server
```

## Architecture Decisions
See `docs/specs/` for full ADRs. Key ones:
- ADR-001: Rust core engine (single source for Python + WASM)
- ADR-004: Cloudflare R2 (zero egress fees)
- ADR-006: Decimal-as-string for financial values
- ADR-007: Original PDF as render layer
- ADR-008: Cursor-based pagination
