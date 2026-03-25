#!/usr/bin/env bash
# Generate BUILD_STATUS.md after build/test on ASUS TUF.
# Usage: just status  (or: bash scripts/build-status.sh)
#
# Runs all checks and writes results to .build-results/

set -uo pipefail

RESULTS_DIR=".build-results"
STATUS_FILE="$RESULTS_DIR/BUILD_STATUS.md"

mkdir -p "$RESULTS_DIR"

VERSION=$(cat VERSION 2>/dev/null || echo "unknown")
COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
MACHINE=$(hostname)

run_check() {
    local name="$1"
    local log_file="$2"
    shift 2
    local cmd=("$@")

    echo "Running: $name ..."
    if "${cmd[@]}" > "$RESULTS_DIR/$log_file" 2>&1; then
        echo "PASS"
        return 0
    else
        echo "FAIL"
        return 1
    fi
}

# Track results
declare -A RESULTS

# Rust checks
if command -v cargo &>/dev/null; then
    run_check "cargo build" "rust-build.log" cargo build --workspace && RESULTS[rust_build]="PASS" || RESULTS[rust_build]="FAIL"
    run_check "cargo test" "rust-test.log" cargo test --workspace && RESULTS[rust_test]="PASS" || RESULTS[rust_test]="FAIL"
    run_check "cargo clippy" "rust-clippy.log" cargo clippy --workspace -- -D warnings && RESULTS[rust_clippy]="PASS" || RESULTS[rust_clippy]="FAIL"
    run_check "cargo fmt check" "rust-fmt.log" cargo fmt --all -- --check && RESULTS[rust_fmt]="PASS" || RESULTS[rust_fmt]="FAIL"
else
    RESULTS[rust_build]="SKIP"
    RESULTS[rust_test]="SKIP"
    RESULTS[rust_clippy]="SKIP"
    RESULTS[rust_fmt]="SKIP"
fi

# Python checks
if [[ -d "api" ]] && command -v python3 &>/dev/null; then
    run_check "pip install" "python-install.log" pip3 install -e "api/.[dev]" && RESULTS[py_install]="PASS" || RESULTS[py_install]="FAIL"
    run_check "pytest" "python-test.log" python3 -m pytest api/tests/ -v && RESULTS[py_test]="PASS" || RESULTS[py_test]="FAIL"
else
    RESULTS[py_install]="SKIP"
    RESULTS[py_test]="SKIP"
fi

# Frontend checks
if [[ -d "studio" ]] && [[ -f "studio/package.json" ]] && command -v npm &>/dev/null; then
    run_check "npm install" "frontend-install.log" npm --prefix studio install && RESULTS[fe_install]="PASS" || RESULTS[fe_install]="FAIL"
    run_check "npm build" "frontend-build.log" npm --prefix studio run build && RESULTS[fe_build]="PASS" || RESULTS[fe_build]="FAIL"
else
    RESULTS[fe_install]="SKIP"
    RESULTS[fe_build]="SKIP"
fi

# Write BUILD_STATUS.md
status_icon() {
    case "$1" in
        PASS) echo "[x]" ;;
        FAIL) echo "[ ]" ;;
        SKIP) echo "[-]" ;;
    esac
}

cat > "$STATUS_FILE" <<EOF
# Build Status

## Last Run
- **Version:** $VERSION
- **Date:** $DATE
- **Commit:** $COMMIT
- **Branch:** $BRANCH
- **Machine:** $MACHINE

## Rust Core (crates/)
- $(status_icon "${RESULTS[rust_build]}") cargo build: ${RESULTS[rust_build]}
- $(status_icon "${RESULTS[rust_test]}") cargo test: ${RESULTS[rust_test]}
- $(status_icon "${RESULTS[rust_clippy]}") cargo clippy: ${RESULTS[rust_clippy]}
- $(status_icon "${RESULTS[rust_fmt]}") cargo fmt --check: ${RESULTS[rust_fmt]}

## Python API (api/)
- $(status_icon "${RESULTS[py_install]}") pip install: ${RESULTS[py_install]}
- $(status_icon "${RESULTS[py_test]}") pytest: ${RESULTS[py_test]}

## Studio Frontend (studio/)
- $(status_icon "${RESULTS[fe_install]}") npm install: ${RESULTS[fe_install]}
- $(status_icon "${RESULTS[fe_build]}") npm build: ${RESULTS[fe_build]}

## Error Logs
EOF

# Append any failure logs
for log_file in "$RESULTS_DIR"/*.log; do
    [[ -f "$log_file" ]] || continue
    base=$(basename "$log_file")
    if grep -qiE "error|failed|panic" "$log_file" 2>/dev/null; then
        echo "" >> "$STATUS_FILE"
        echo "### $base" >> "$STATUS_FILE"
        echo '```' >> "$STATUS_FILE"
        tail -50 "$log_file" >> "$STATUS_FILE"
        echo '```' >> "$STATUS_FILE"
    fi
done

echo ""
echo "Build status written to $STATUS_FILE"
echo "Commit and push: git add $RESULTS_DIR && git commit -m 'build: update status' && git push"
