# Generate BUILD_STATUS.md after build/test on ASUS TUF (Windows)
# Usage: just status  (or: powershell -File scripts/build-status.ps1)

$ErrorActionPreference = "Continue"

$resultsDir = ".build-results"
$statusFile = "$resultsDir\BUILD_STATUS.md"

if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

$version = if (Test-Path "VERSION") { (Get-Content "VERSION").Trim() } else { "unknown" }
$commit = git rev-parse --short HEAD 2>$null; if (-not $commit) { $commit = "unknown" }
$branch = git branch --show-current 2>$null; if (-not $branch) { $branch = "unknown" }
$date = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
$machine = $env:COMPUTERNAME

$results = @{}

function Run-Check($name, $logFile, $command) {
    Write-Host "Running: $name ..."
    try {
        $output = Invoke-Expression "$command 2>&1" | Out-String
        $output | Out-File -FilePath "$resultsDir\$logFile" -Encoding utf8
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  PASS" -ForegroundColor Green
            return "PASS"
        } else {
            Write-Host "  FAIL" -ForegroundColor Red
            return "FAIL"
        }
    } catch {
        $_.Exception.Message | Out-File -FilePath "$resultsDir\$logFile" -Encoding utf8
        Write-Host "  FAIL" -ForegroundColor Red
        return "FAIL"
    }
}

# Rust checks
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    $results["rust_build"]  = Run-Check "cargo build"     "rust-build.log"  "cargo build --workspace"
    $results["rust_test"]   = Run-Check "cargo test"      "rust-test.log"   "cargo test --workspace"
    $results["rust_clippy"] = Run-Check "cargo clippy"    "rust-clippy.log" "cargo clippy --workspace -- -D warnings"
    $results["rust_fmt"]    = Run-Check "cargo fmt check" "rust-fmt.log"    "cargo fmt --all -- --check"
} else {
    "rust_build","rust_test","rust_clippy","rust_fmt" | ForEach-Object { $results[$_] = "SKIP" }
}

# Python checks
if ((Test-Path "api") -and (Get-Command python -ErrorAction SilentlyContinue)) {
    $results["py_install"] = Run-Check "pip install" "python-install.log" "pip install -e 'api/.[dev]'"
    $results["py_test"]    = Run-Check "pytest"      "python-test.log"    "python -m pytest api/tests/ -v"
} else {
    "py_install","py_test" | ForEach-Object { $results[$_] = "SKIP" }
}

# Frontend checks
if ((Test-Path "studio\package.json") -and (Get-Command npm -ErrorAction SilentlyContinue)) {
    $results["fe_install"] = Run-Check "npm install" "frontend-install.log" "npm --prefix studio install"
    $results["fe_build"]   = Run-Check "npm build"   "frontend-build.log"   "npm --prefix studio run build"
} else {
    "fe_install","fe_build" | ForEach-Object { $results[$_] = "SKIP" }
}

# Status icon
function Icon($status) {
    switch ($status) {
        "PASS" { return "[x]" }
        "FAIL" { return "[ ]" }
        "SKIP" { return "[-]" }
    }
}

# Write BUILD_STATUS.md
$content = @"
# Build Status

## Last Run
- **Version:** $version
- **Date:** $date
- **Commit:** $commit
- **Branch:** $branch
- **Machine:** $machine

## Rust Core (crates/)
- $(Icon $results["rust_build"]) cargo build: $($results["rust_build"])
- $(Icon $results["rust_test"]) cargo test: $($results["rust_test"])
- $(Icon $results["rust_clippy"]) cargo clippy: $($results["rust_clippy"])
- $(Icon $results["rust_fmt"]) cargo fmt --check: $($results["rust_fmt"])

## Python API (api/)
- $(Icon $results["py_install"]) pip install: $($results["py_install"])
- $(Icon $results["py_test"]) pytest: $($results["py_test"])

## Studio Frontend (studio/)
- $(Icon $results["fe_install"]) npm install: $($results["fe_install"])
- $(Icon $results["fe_build"]) npm build: $($results["fe_build"])

## Error Logs
"@

# Append error logs
Get-ChildItem "$resultsDir\*.log" | ForEach-Object {
    $logContent = Get-Content $_.FullName -Raw
    if ($logContent -match "error|failed|panic") {
        $content += "`n### $($_.Name)`n``````n"
        $content += ($logContent | Select-Object -Last 50) -join "`n"
        $content += "`n```````n"
    }
}

$content | Out-File -FilePath $statusFile -Encoding utf8

Write-Host ""
Write-Host "Build status written to $statusFile" -ForegroundColor Green
Write-Host "Commit and push:"
Write-Host "  git add $resultsDir && git commit -m 'build: status $version' && git push"
