# SPDF Changelog

All notable changes tracked per snapshot version. Each snapshot represents one push-build-test cycle between Enterprise Desktop and ASUS TUF.

Format: `MAJOR.MINOR.PATCH-snapshot.N`
- **MAJOR:** Breaking format/API changes
- **MINOR:** New features or crate additions
- **PATCH:** Bug fixes, refinements
- **snapshot.N:** Increments every push-build cycle

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
