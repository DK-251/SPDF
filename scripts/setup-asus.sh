#!/usr/bin/env bash
# SPDF Development Environment Setup & Validation
# Target: ASUS TUF F15 (personal laptop, unrestricted)
# Usage: bash scripts/setup-asus.sh
#
# This script:
#   1. Checks each required tool
#   2. Installs missing tools (user-level where possible)
#   3. Prints a final status report

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0
WARN=0

pass()  { echo -e "  ${GREEN}[OK]${NC} $1"; ((PASS++)); }
fail()  { echo -e "  ${RED}[FAIL]${NC} $1"; ((FAIL++)); }
warn()  { echo -e "  ${YELLOW}[WARN]${NC} $1"; ((WARN++)); }

section() { echo -e "\n${YELLOW}--- $1 ---${NC}"; }

check_cmd() {
    if command -v "$1" &>/dev/null; then
        local ver
        ver=$("$1" --version 2>/dev/null | head -1)
        pass "$1 ($ver)"
        return 0
    else
        return 1
    fi
}

# ============================================================
section "Operating System"
# ============================================================

echo "  OS: $(uname -a)"

# ============================================================
section "Git"
# ============================================================

if check_cmd git; then
    GIT_NAME=$(git config --global user.name 2>/dev/null || echo "")
    GIT_EMAIL=$(git config --global user.email 2>/dev/null || echo "")
    if [[ -z "$GIT_NAME" || -z "$GIT_EMAIL" ]]; then
        warn "Git identity not configured globally. Set it with:"
        echo "       git config --global user.name 'Deepak Sahu'"
        echo "       git config --global user.email 'Deepak.Sahu4@cognizant.com'"
    else
        pass "Git identity: $GIT_NAME <$GIT_EMAIL>"
    fi
else
    fail "git not found"
    echo "       Install: https://git-scm.com/downloads"
fi

# ============================================================
section "Rust Toolchain"
# ============================================================

if check_cmd rustc; then
    RUST_VER=$(rustc --version | grep -oP '\d+\.\d+\.\d+')
    REQUIRED="1.75.0"
    if [[ "$(printf '%s\n' "$REQUIRED" "$RUST_VER" | sort -V | head -1)" == "$REQUIRED" ]]; then
        pass "Rust version $RUST_VER >= $REQUIRED"
    else
        warn "Rust $RUST_VER is below required $REQUIRED. Run: rustup update stable"
    fi
else
    fail "rustc not found"
    echo "       Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "       Then: source \$HOME/.cargo/env"
fi

if check_cmd cargo; then :; else
    fail "cargo not found (install Rust first)"
fi

# Check Rust targets
if command -v rustup &>/dev/null; then
    if rustup target list --installed 2>/dev/null | grep -q "wasm32-unknown-unknown"; then
        pass "WASM target installed"
    else
        warn "WASM target missing. Install: rustup target add wasm32-unknown-unknown"
    fi
fi

# ============================================================
section "Python"
# ============================================================

PYTHON_CMD=""
for cmd in python3 python; do
    if command -v "$cmd" &>/dev/null; then
        VER=$("$cmd" --version 2>&1 | grep -oP '\d+\.\d+' | head -1)
        if [[ "$VER" == "3.12" || "$VER" == "3.13" || "$VER" == "3.14" ]]; then
            PYTHON_CMD="$cmd"
            pass "$cmd ($("$cmd" --version 2>&1))"
            break
        else
            warn "$cmd is version $VER (need 3.12+)"
        fi
    fi
done

if [[ -z "$PYTHON_CMD" ]]; then
    fail "Python 3.12+ not found"
    echo "       Install: https://www.python.org/downloads/"
    echo "       Or (Linux): sudo apt install python3.12 python3.12-venv"
fi

if command -v pip3 &>/dev/null || command -v pip &>/dev/null; then
    PIP_CMD=$(command -v pip3 || command -v pip)
    pass "pip ($($PIP_CMD --version 2>&1 | head -1))"
else
    fail "pip not found"
fi

# ============================================================
section "Node.js"
# ============================================================

if check_cmd node; then
    NODE_VER=$(node --version | grep -oP '\d+' | head -1)
    if (( NODE_VER >= 20 )); then
        pass "Node.js v$NODE_VER >= 20"
    else
        warn "Node.js v$NODE_VER is below recommended v20+"
    fi
else
    fail "node not found"
    echo "       Install: https://nodejs.org/ (LTS recommended)"
fi

check_cmd npm || fail "npm not found"

# ============================================================
section "Development Tools"
# ============================================================

if check_cmd wasm-pack; then :; else
    warn "wasm-pack not found. Install: cargo install wasm-pack"
fi

if check_cmd just; then :; else
    warn "just not found. Install: cargo install just"
fi

if check_cmd maturin; then :; else
    warn "maturin not found (needed for PyO3 builds). Install: pip install maturin"
fi

# ============================================================
section "Docker (optional, for local DB/Redis)"
# ============================================================

if check_cmd docker; then
    if docker info &>/dev/null 2>&1; then
        pass "Docker daemon running"
    else
        warn "Docker installed but daemon not running"
    fi
else
    warn "Docker not found (will use cloud services for DB/Redis)"
fi

# ============================================================
section "Auto-Install Missing Tools"
# ============================================================

read -rp "Attempt to install missing tools? [y/N] " INSTALL
if [[ "$INSTALL" =~ ^[Yy]$ ]]; then

    # Rust
    if ! command -v rustc &>/dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        source "$HOME/.cargo/env"
        pass "Rust installed"
    fi

    # WASM target
    if command -v rustup &>/dev/null; then
        rustup target add wasm32-unknown-unknown 2>/dev/null && pass "WASM target added"
    fi

    # just
    if ! command -v just &>/dev/null && command -v cargo &>/dev/null; then
        echo "Installing just..."
        cargo install just && pass "just installed"
    fi

    # wasm-pack
    if ! command -v wasm-pack &>/dev/null && command -v cargo &>/dev/null; then
        echo "Installing wasm-pack..."
        cargo install wasm-pack && pass "wasm-pack installed"
    fi

    # Python (suggest only, OS-dependent)
    if [[ -z "$PYTHON_CMD" ]]; then
        if command -v apt &>/dev/null; then
            echo "Installing Python 3.12 via apt..."
            sudo apt update && sudo apt install -y python3.12 python3.12-venv python3-pip
        elif command -v brew &>/dev/null; then
            echo "Installing Python 3.12 via brew..."
            brew install python@3.12
        else
            warn "Cannot auto-install Python. Download from https://www.python.org/downloads/"
        fi
    fi

    # maturin
    if ! command -v maturin &>/dev/null; then
        PIP_CMD=$(command -v pip3 2>/dev/null || command -v pip 2>/dev/null || echo "")
        if [[ -n "$PIP_CMD" ]]; then
            echo "Installing maturin..."
            $PIP_CMD install maturin && pass "maturin installed"
        fi
    fi
fi

# ============================================================
section "SUMMARY"
# ============================================================

echo ""
echo -e "  ${GREEN}Passed: $PASS${NC}"
echo -e "  ${YELLOW}Warnings: $WARN${NC}"
echo -e "  ${RED}Failed: $FAIL${NC}"
echo ""

if (( FAIL > 0 )); then
    echo -e "${RED}Environment NOT ready. Fix the failures above.${NC}"
    exit 1
elif (( WARN > 0 )); then
    echo -e "${YELLOW}Environment mostly ready. Address warnings for full capability.${NC}"
    exit 0
else
    echo -e "${GREEN}Environment fully ready. Start building!${NC}"
    exit 0
fi
