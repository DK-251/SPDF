# SPDF Project Task Runner
# Usage: just <recipe>

# Default recipe: show available commands
default:
    @just --list

# Show current version
version:
    @cat VERSION

# Bump snapshot version (run before each push from Enterprise Desktop)
bump:
    bash scripts/bump-version.sh

# Bump minor version (new feature added)
bump-minor:
    bash scripts/bump-version.sh minor

# Bump patch version (bug fix)
bump-patch:
    bash scripts/bump-version.sh patch

# --- Rust ---

# Build all Rust crates
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run all Rust tests
test:
    cargo test --workspace

# Run clippy linter
lint:
    cargo clippy --workspace -- -D warnings

# Format Rust code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Full Rust CI check (format + lint + test)
check: fmt-check lint test

# --- Python API ---

# Install Python dependencies
api-install:
    cd api && pip install -e ".[dev]"

# Run API server (dev)
api-dev:
    cd api && uvicorn app.main:app --reload --port 8000

# Run Python tests
api-test:
    cd api && pytest tests/ -v

# --- Studio Frontend ---

# Install frontend dependencies
studio-install:
    cd studio && npm install

# Run Studio dev server
studio-dev:
    cd studio && npm run dev

# Build Studio for production
studio-build:
    cd studio && npm run build

# Run frontend tests
studio-test:
    cd studio && npm test

# --- Cross-cutting ---

# Run all tests across the entire project
test-all: test api-test studio-test

# Generate build status report (run on ASUS TUF after build)
status:
    bash scripts/build-status.sh
