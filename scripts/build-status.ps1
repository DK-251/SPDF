# Generate BUILD_STATUS.md after build/test on ASUS TUF (Windows)
# Usage: just status  (or: powershell -ExecutionPolicy Bypass -File scripts/build-status.ps1)

$ErrorActionPreference = "Continue"

# Refresh PATH to pick up tools installed in current session
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")

# Also add common tool locations
$cargoBin = "$env:USERPROFILE\.cargo\bin"
if (Test-Path $cargoBin) { $env:Path += ";$cargoBin" }

$resultsDir = ".build-results"
$statusFile = "$resultsDir\BUILD_STATUS.md"

if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

$version = if (Test-Path "VERSION") { (Get-Content "VERSION" -Raw).Trim() } else { "unknown" }
$commit = & git rev-parse --short HEAD 2>$null; if (-not $commit) { $commit = "unknown" }
$branch = & git branch --show-current 2>$null; if (-not $branch) { $branch = "unknown" }
$date = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
$machine = $env:COMPUTERNAME

$results = @{}

function Run-Check {
    param($name, $logFile, [string[]]$args_list)

    Write-Host "Running: $name ..." -NoNewline
    $logPath = "$resultsDir\$logFile"
    $LASTEXITCODE = 0

    try {
        & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logPath
        if ($LASTEXITCODE -eq 0) {
            Write-Host " PASS" -ForegroundColor Green
            return "PASS"
        } else {
            Write-Host " FAIL (exit code: $LASTEXITCODE)" -ForegroundColor Red
            return "FAIL"
        }
    } catch {
        $_.Exception.Message | Out-File -FilePath $logPath -Encoding utf8
        Write-Host " FAIL (exception)" -ForegroundColor Red
        return "FAIL"
    }
}

# --- Rust checks ---
$hasCargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($hasCargo) {
    Write-Host "`n=== Rust ===" -ForegroundColor Cyan
    $results["rust_build"]  = Run-Check "cargo build"     "rust-build.log"   @("cargo", "build", "--workspace")
    $results["rust_test"]   = Run-Check "cargo test"      "rust-test.log"    @("cargo", "test", "--workspace")
    $results["rust_clippy"] = Run-Check "cargo clippy"    "rust-clippy.log"  @("cargo", "clippy", "--workspace", "--", "-D", "warnings")
    $results["rust_fmt"]    = Run-Check "cargo fmt check" "rust-fmt.log"     @("cargo", "fmt", "--all", "--", "--check")
} else {
    Write-Host "`ncargo not found in PATH - skipping Rust checks" -ForegroundColor Yellow
    Write-Host "  PATH searched: $env:Path" -ForegroundColor DarkGray
    @("rust_build","rust_test","rust_clippy","rust_fmt") | ForEach-Object { $results[$_] = "SKIP" }
}

# --- Python checks ---
$hasPython = Get-Command python -ErrorAction SilentlyContinue
if ((Test-Path "api") -and $hasPython) {
    Write-Host "`n=== Python ===" -ForegroundColor Cyan
    $results["py_install"] = Run-Check "pip install" "python-install.log" @("pip", "install", "-e", "api/.[dev]")
    $results["py_test"]    = Run-Check "pytest"      "python-test.log"    @("python", "-m", "pytest", "api/tests/", "-v")
} else {
    if (-not $hasPython) { Write-Host "`npython not found - skipping Python checks" -ForegroundColor Yellow }
    @("py_install","py_test") | ForEach-Object { $results[$_] = "SKIP" }
}

# --- Frontend checks ---
$hasNpm = Get-Command npm -ErrorAction SilentlyContinue
if ((Test-Path "studio\package.json") -and $hasNpm) {
    Write-Host "`n=== Frontend ===" -ForegroundColor Cyan
    $results["fe_install"] = Run-Check "npm install" "frontend-install.log" @("npm", "--prefix", "studio", "install")
    $results["fe_build"]   = Run-Check "npm build"   "frontend-build.log"   @("npm", "--prefix", "studio", "run", "build")
} else {
    @("fe_install","fe_build") | ForEach-Object { $results[$_] = "SKIP" }
}

# --- Status icon ---
function Icon($status) {
    switch ($status) {
        "PASS" { return "[x]" }
        "FAIL" { return "[ ]" }
        "SKIP" { return "[-]" }
        default { return "[-]" }
    }
}

# --- Write BUILD_STATUS.md ---
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

# Append error log snippets
$logFiles = Get-ChildItem "$resultsDir\*.log" -ErrorAction SilentlyContinue
foreach ($log in $logFiles) {
    $logContent = Get-Content $log.FullName -Raw -ErrorAction SilentlyContinue
    if ($logContent -and ($logContent -match "error|failed|panic|FAILED")) {
        $lastLines = (Get-Content $log.FullName -Tail 50) -join "`n"
        $content += "`n### $($log.Name)`n``````n$lastLines`n```````n"
    }
}

$content | Out-File -FilePath $statusFile -Encoding utf8 -Force

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "Build status written to $statusFile" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "`nNext steps:"
Write-Host "  git add .build-results" -ForegroundColor White
Write-Host "  git commit -m 'build: status $version'" -ForegroundColor White
Write-Host "  git push" -ForegroundColor White
