# Unified CI check: Rust (fmt + clippy + test) + Python (maturin + pytest)
# Usage: just check  (run on ASUS TUF after pull)
#
# Produces:
#   .build-results/CHECK_RESULTS.md — structured pass/fail with inline errors
#
# After running:
#   git add .build-results && git commit -m "check: <version>" && git push

$ErrorActionPreference = "Continue"

# Refresh PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
$cargoBin = "$env:USERPROFILE\.cargo\bin"
if (Test-Path $cargoBin) { $env:Path += ";$cargoBin" }

$resultsDir = ".build-results"
$reportFile = "$resultsDir\CHECK_RESULTS.md"

if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

# Clean old logs
Get-ChildItem "$resultsDir\*.log" -ErrorAction SilentlyContinue | Remove-Item -Force

$version = if (Test-Path "VERSION") { (Get-Content "VERSION" -Raw).Trim() } else { "unknown" }
$commit  = & git rev-parse --short HEAD 2>$null; if (-not $commit) { $commit = "unknown" }
$branch  = & git branch --show-current 2>$null; if (-not $branch) { $branch = "unknown" }
$date    = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
$machine = $env:COMPUTERNAME

# --- Step runner ---

$steps = @()

function Run-Step {
    param([string]$Section, [string]$Name, [string]$Cmd)

    Write-Host "  [$Section] $Name ..." -NoNewline

    $tempOut = [System.IO.Path]::GetTempFileName()

    try {
        Invoke-Expression "$Cmd *> `"$tempOut`" 2>&1"
        $exitCode = $LASTEXITCODE
    } catch {
        $_.Exception.Message | Out-File -FilePath $tempOut -Encoding utf8 -Append
        $exitCode = 1
    }

    $output = if (Test-Path $tempOut) { Get-Content $tempOut -Raw -ErrorAction SilentlyContinue } else { "" }
    Remove-Item $tempOut -Force -ErrorAction SilentlyContinue

    if ($exitCode -eq 0) {
        Write-Host " PASS" -ForegroundColor Green
        $status = "PASS"
    } else {
        Write-Host " FAIL (exit $exitCode)" -ForegroundColor Red
        $status = "FAIL"
    }

    return @{
        Section  = $Section
        Name     = $Name
        Status   = $status
        ExitCode = $exitCode
        Output   = $output
    }
}

# --- Rust checks ---

$hasCargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($hasCargo) {
    Write-Host "`n=== Rust ===" -ForegroundColor Cyan
    $steps += Run-Step "Rust" "cargo fmt --check" "cargo fmt --all -- --check"
    $steps += Run-Step "Rust" "cargo clippy"      "cargo clippy --workspace -- -D warnings"
    $steps += Run-Step "Rust" "cargo test"         "cargo test --workspace 2>&1"
} else {
    Write-Host "`ncargo not found — skipping Rust checks" -ForegroundColor Yellow
    $steps += @{ Section="Rust"; Name="cargo (not found)"; Status="SKIP"; ExitCode=0; Output="" }
}

# --- Python checks ---

$hasPython = Get-Command python -ErrorAction SilentlyContinue
$hasMaturin = Get-Command maturin -ErrorAction SilentlyContinue

if ($hasPython -and (Test-Path "api")) {
    Write-Host "`n=== Python ===" -ForegroundColor Cyan

    # Install API deps
    $steps += Run-Step "Python" "pip install api[dev]" "pip install -e api/.[dev] --quiet"

    # Build PyO3 bindings if maturin is available
    if ($hasMaturin) {
        $steps += Run-Step "Python" "maturin develop" "maturin develop -m crates/spdf-python/Cargo.toml"
    } else {
        Write-Host "  [Python] maturin not found — skipping native build" -ForegroundColor Yellow
        $steps += @{ Section="Python"; Name="maturin (not found)"; Status="SKIP"; ExitCode=0; Output="" }
    }

    # Run pytest
    $steps += Run-Step "Python" "pytest" "python -m pytest api/tests/ -v --tb=short 2>&1"
} else {
    if (-not $hasPython) { Write-Host "`npython not found — skipping Python checks" -ForegroundColor Yellow }
    $steps += @{ Section="Python"; Name="python (not found)"; Status="SKIP"; ExitCode=0; Output="" }
}

# --- Build report ---

$allPass = -not ($steps | Where-Object { $_.Status -eq "FAIL" })
$overallStatus = if ($allPass) { "ALL PASS" } else { "FAILING" }
$passCount = ($steps | Where-Object { $_.Status -eq "PASS" }).Count
$failCount = ($steps | Where-Object { $_.Status -eq "FAIL" }).Count
$skipCount = ($steps | Where-Object { $_.Status -eq "SKIP" }).Count
$totalCount = $steps.Count

$report = @"
# CHECK RESULTS

## Run Info
- **Version:** $version
- **Commit:** $commit
- **Branch:** $branch
- **Date:** $date
- **Machine:** $machine
- **Overall:** $overallStatus ($passCount pass, $failCount fail, $skipCount skip / $totalCount total)

## Steps

"@

$currentSection = ""
foreach ($step in $steps) {
    if ($step.Section -ne $currentSection) {
        $report += "`n### $($step.Section)`n"
        $currentSection = $step.Section
    }
    $icon = switch ($step.Status) { "PASS" { "[x]" } "FAIL" { "[ ]" } "SKIP" { "[-]" } }
    $report += "- $icon ``$($step.Name)``: **$($step.Status)**`n"
}

# --- Failure details ---

$failures = $steps | Where-Object { $_.Status -eq "FAIL" }
if ($failures) {
    $report += "`n---`n`n## Failure Details`n"

    foreach ($f in $failures) {
        $report += "`n### $($f.Section): $($f.Name)`n"
        $report += "Exit code: $($f.ExitCode)`n`n"

        if ($f.Output) {
            $lines = $f.Output -split "`n"
            # Show last 100 lines max
            if ($lines.Count -gt 100) {
                $lines = $lines[($lines.Count - 100)..($lines.Count - 1)]
                $report += "*(truncated to last 100 lines)*`n`n"
            }
            $trimmed = ($lines -join "`n").TrimEnd()
            $report += "``````text`n$trimmed`n```````n"
        }
    }
} else {
    $report += "`n---`n`nAll checks passed. No errors to report.`n"
}

# --- Test summary (extract counts from cargo test + pytest output) ---

$report += "`n---`n`n## Test Summary`n"

$cargoTestStep = $steps | Where-Object { $_.Name -eq "cargo test" } | Select-Object -First 1
if ($cargoTestStep -and $cargoTestStep.Output) {
    $testLines = $cargoTestStep.Output -split "`n" | Where-Object { $_ -match "^test result:" }
    if ($testLines) {
        $report += "`n### Rust`n"
        foreach ($line in $testLines) {
            $report += "- ``$($line.Trim())```n"
        }
    }
}

$pytestStep = $steps | Where-Object { $_.Name -eq "pytest" } | Select-Object -First 1
if ($pytestStep -and $pytestStep.Output) {
    $summaryLines = $pytestStep.Output -split "`n" | Where-Object { $_ -match "passed|failed|error|skipped" } | Select-Object -Last 1
    if ($summaryLines) {
        $report += "`n### Python`n"
        $report += "- ``$($summaryLines.Trim())```n"
    }
}

$report | Out-File -FilePath $reportFile -Encoding utf8 -Force

# --- Console summary ---

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
if ($allPass) {
    Write-Host "  ALL CHECKS PASSED ($passCount/$totalCount)" -ForegroundColor Green
} else {
    Write-Host "  CHECKS FAILING ($failCount/$totalCount failed)" -ForegroundColor Red
}
Write-Host "  Report: $reportFile" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor White
Write-Host "  git add .build-results" -ForegroundColor Yellow
Write-Host "  git commit -m 'check: $version'" -ForegroundColor Yellow
Write-Host "  git push" -ForegroundColor Yellow
