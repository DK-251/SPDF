# SPDF Development Environment Setup & Validation
# Target: ASUS TUF F15 (Windows 11, personal laptop)
# Usage: powershell -ExecutionPolicy Bypass -File scripts/setup-asus.ps1

$ErrorActionPreference = "Continue"

$pass = 0
$fail = 0
$warn = 0

function Pass($msg) { Write-Host "  [OK]   $msg" -ForegroundColor Green; $script:pass++ }
function Fail($msg) { Write-Host "  [FAIL] $msg" -ForegroundColor Red; $script:fail++ }
function Warn($msg) { Write-Host "  [WARN] $msg" -ForegroundColor Yellow; $script:warn++ }
function Section($msg) { Write-Host "`n--- $msg ---" -ForegroundColor Yellow }

function Check-Command($cmd) {
    $found = Get-Command $cmd -ErrorAction SilentlyContinue
    if ($found) {
        $ver = & $cmd --version 2>&1 | Select-Object -First 1
        Pass "$cmd ($ver)"
        return $true
    }
    return $false
}

# ============================================================
Section "Operating System"
# ============================================================

$os = (Get-CimInstance Win32_OperatingSystem).Caption
Write-Host "  OS: $os"

# ============================================================
Section "Git"
# ============================================================

if (Check-Command "git") {
    $gitName = git config --global user.name 2>$null
    $gitEmail = git config --global user.email 2>$null
    if (-not $gitName -or -not $gitEmail) {
        Warn "Git identity not configured. Run:"
        Write-Host "       git config --global user.name 'Deepak Sahu'"
        Write-Host "       git config --global user.email 'Deepak.Sahu4@cognizant.com'"
    } else {
        Pass "Git identity: $gitName <$gitEmail>"
    }
} else {
    Fail "git not found"
    Write-Host "       Install: https://git-scm.com/downloads"
}

# ============================================================
Section "Rust Toolchain"
# ============================================================

if (Check-Command "rustc") {
    $rustVer = rustc --version 2>$null
    if ($rustVer -match "(\d+\.\d+\.\d+)") {
        $ver = [version]$Matches[1]
        if ($ver -ge [version]"1.75.0") {
            Pass "Rust version $($Matches[1]) >= 1.75.0"
        } else {
            Warn "Rust $($Matches[1]) below required 1.75.0. Run: rustup update stable"
        }
    }
} else {
    Fail "rustc not found"
    Write-Host "       Install: https://rustup.rs (download rustup-init.exe)"
}

if (-not (Check-Command "cargo")) {
    Fail "cargo not found (install Rust first)"
}

if (Get-Command rustup -ErrorAction SilentlyContinue) {
    $wasmTarget = rustup target list --installed 2>$null | Select-String "wasm32-unknown-unknown"
    if ($wasmTarget) {
        Pass "WASM target installed"
    } else {
        Warn "WASM target missing. Run: rustup target add wasm32-unknown-unknown"
    }
}

# ============================================================
Section "Python"
# ============================================================

$pythonCmd = $null
foreach ($cmd in @("python", "python3")) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        $pyVer = & $cmd --version 2>&1
        if ($pyVer -match "Python (3\.1[2-9]|3\.[2-9]\d)") {
            $pythonCmd = $cmd
            Pass "$cmd ($pyVer)"
            break
        } elseif ($pyVer -match "Python") {
            Warn "$cmd is $pyVer (need 3.12+)"
        }
    }
}

if (-not $pythonCmd) {
    Fail "Python 3.12+ not found"
    Write-Host "       Install: https://www.python.org/downloads/"
    Write-Host "       IMPORTANT: Check 'Add Python to PATH' during install"
}

$pipCmd = Get-Command pip -ErrorAction SilentlyContinue
if (-not $pipCmd) { $pipCmd = Get-Command pip3 -ErrorAction SilentlyContinue }
if ($pipCmd) {
    $pipVer = & $pipCmd.Name --version 2>&1 | Select-Object -First 1
    Pass "pip ($pipVer)"
} else {
    Fail "pip not found"
}

# ============================================================
Section "Node.js"
# ============================================================

if (Check-Command "node") {
    $nodeVer = node --version 2>$null
    if ($nodeVer -match "v(\d+)") {
        $major = [int]$Matches[1]
        if ($major -ge 20) {
            Pass "Node.js v$major >= 20"
        } else {
            Warn "Node.js v$major below recommended v20+"
        }
    }
} else {
    Fail "node not found"
    Write-Host "       Install: https://nodejs.org/ (LTS recommended)"
}

if (-not (Check-Command "npm")) { Fail "npm not found" }

# ============================================================
Section "Development Tools"
# ============================================================

if (-not (Check-Command "wasm-pack")) {
    Warn "wasm-pack not found. Run: cargo install wasm-pack"
}

if (-not (Check-Command "just")) {
    Warn "just not found. Run: cargo install just"
}

if (-not (Check-Command "maturin")) {
    Warn "maturin not found (needed for PyO3). Run: pip install maturin"
}

# ============================================================
Section "Docker (optional)"
# ============================================================

if (Check-Command "docker") {
    $dockerRunning = docker info 2>$null
    if ($LASTEXITCODE -eq 0) {
        Pass "Docker daemon running"
    } else {
        Warn "Docker installed but daemon not running"
    }
} else {
    Warn "Docker not found (will use cloud services for DB/Redis)"
}

# ============================================================
Section "Auto-Install Missing Tools"
# ============================================================

$install = Read-Host "Attempt to install missing tools? [y/N]"
if ($install -eq "y" -or $install -eq "Y") {

    # Check for winget
    $hasWinget = Get-Command winget -ErrorAction SilentlyContinue

    # Rust
    if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
        Write-Host "Installing Rust via rustup..."
        if ($hasWinget) {
            winget install Rustlang.Rustup --accept-package-agreements --accept-source-agreements
        } else {
            Write-Host "Download rustup-init.exe from https://rustup.rs"
            Start-Process "https://rustup.rs"
        }
    }

    # WASM target
    if (Get-Command rustup -ErrorAction SilentlyContinue) {
        rustup target add wasm32-unknown-unknown 2>$null
        Pass "WASM target added"
    }

    # just
    if (-not (Get-Command just -ErrorAction SilentlyContinue) -and (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "Installing just..."
        cargo install just
    }

    # wasm-pack
    if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue) -and (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "Installing wasm-pack..."
        cargo install wasm-pack
    }

    # Python
    if (-not $pythonCmd) {
        if ($hasWinget) {
            Write-Host "Installing Python 3.12..."
            winget install Python.Python.3.12 --accept-package-agreements --accept-source-agreements
        } else {
            Write-Host "Download Python from https://www.python.org/downloads/"
            Start-Process "https://www.python.org/downloads/"
        }
    }

    # Node.js
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        if ($hasWinget) {
            Write-Host "Installing Node.js LTS..."
            winget install OpenJS.NodeJS.LTS --accept-package-agreements --accept-source-agreements
        } else {
            Write-Host "Download Node.js from https://nodejs.org/"
            Start-Process "https://nodejs.org/"
        }
    }

    # maturin
    if (-not (Get-Command maturin -ErrorAction SilentlyContinue)) {
        $pip = Get-Command pip -ErrorAction SilentlyContinue
        if ($pip) {
            Write-Host "Installing maturin..."
            pip install maturin
        }
    }
}

# ============================================================
Section "SUMMARY"
# ============================================================

Write-Host ""
Write-Host "  Passed:   $pass" -ForegroundColor Green
Write-Host "  Warnings: $warn" -ForegroundColor Yellow
Write-Host "  Failed:   $fail" -ForegroundColor Red
Write-Host ""

if ($fail -gt 0) {
    Write-Host "Environment NOT ready. Fix the failures above." -ForegroundColor Red
    exit 1
} elseif ($warn -gt 0) {
    Write-Host "Environment mostly ready. Address warnings for full capability." -ForegroundColor Yellow
    exit 0
} else {
    Write-Host "Environment fully ready. Start building!" -ForegroundColor Green
    exit 0
}
