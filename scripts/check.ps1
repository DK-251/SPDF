# Unified CI check: Rust (fmt + clippy + test) + Python (maturin + pytest)
# Usage: just check  (run on ASUS TUF after pull)
#
# Produces:
#   .build-results/CHECK_RESULTS.md -- structured pass/fail with inline errors
#
# After running:
#   git add .build-results && git commit -m "check: <version>" && git push

$ErrorActionPreference = "Continue"

# Backtick character for markdown output (avoids PS escape conflicts)
$bt = [char]96

# Refresh PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
if (Test-Path $cargoBin) { $env:Path += ";$cargoBin" }

$resultsDir = ".build-results"
$reportFile = Join-Path $resultsDir "CHECK_RESULTS.md"

if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

# Clean old logs
Get-ChildItem (Join-Path $resultsDir "*.log") -ErrorAction SilentlyContinue | Remove-Item -Force

$version = if (Test-Path "VERSION") { (Get-Content "VERSION" -Raw).Trim() } else { "unknown" }
$commit = & git rev-parse --short HEAD 2>$null; if (-not $commit) { $commit = "unknown" }
$branch = & git branch --show-current 2>$null; if (-not $branch) { $branch = "unknown" }
$date = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
$machine = $env:COMPUTERNAME

# --- Step runner ---

$steps = [System.Collections.ArrayList]::new()

function Run-Step {
    param([string]$Section, [string]$Name, [string]$Cmd)

    Write-Host "  [$Section] $Name ..." -NoNewline

    $exitCode = 0
    $output = ""

    try {
        $output = (Invoke-Expression "$Cmd 2>&1" | Out-String).Trim()
        $exitCode = $LASTEXITCODE
        if ($null -eq $exitCode) { $exitCode = 0 }
    } catch {
        $output = $_.Exception.Message
        $exitCode = 1
    }

    if ($null -eq $output) { $output = "" }

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
    Write-Host ""
    Write-Host "=== Rust ===" -ForegroundColor Cyan
    $null = $steps.Add((Run-Step "Rust" "cargo fmt --check" "cargo fmt --all -- --check"))
    $null = $steps.Add((Run-Step "Rust" "cargo clippy" "cargo clippy --workspace -- -D warnings"))
    $null = $steps.Add((Run-Step "Rust" "cargo test" "cargo test --workspace"))
} else {
    Write-Host ""
    Write-Host "cargo not found -- skipping Rust checks" -ForegroundColor Yellow
    $null = $steps.Add(@{ Section="Rust"; Name="cargo (not found)"; Status="SKIP"; ExitCode=0; Output="" })
}

# --- Python checks ---

$hasPython = Get-Command python -ErrorAction SilentlyContinue
$hasMaturin = Get-Command maturin -ErrorAction SilentlyContinue

if ($hasPython -and (Test-Path "api")) {
    Write-Host ""
    Write-Host "=== Python ===" -ForegroundColor Cyan

    # Ensure venv exists so maturin develop can install into it
    $venvDir = ".venv"
    if (-not (Test-Path $venvDir)) {
        Write-Host "  Creating virtualenv ..." -ForegroundColor Yellow
        & python -m venv $venvDir
    }
    $env:VIRTUAL_ENV = (Resolve-Path $venvDir).Path
    $env:Path = (Join-Path $env:VIRTUAL_ENV "Scripts") + ";" + $env:Path

    $null = $steps.Add((Run-Step "Python" "pip install api[dev]" "pip install -e api/.[dev] --quiet"))

    if ($hasMaturin) {
        $null = $steps.Add((Run-Step "Python" "maturin develop" "maturin develop -m crates/spdf-python/Cargo.toml"))
    } else {
        Write-Host "  [Python] maturin not found -- skipping native build" -ForegroundColor Yellow
        $null = $steps.Add(@{ Section="Python"; Name="maturin (not found)"; Status="SKIP"; ExitCode=0; Output="" })
    }

    $null = $steps.Add((Run-Step "Python" "pytest" "python -m pytest api/tests/ -v --tb=short"))
} else {
    if (-not $hasPython) {
        Write-Host ""
        Write-Host "python not found -- skipping Python checks" -ForegroundColor Yellow
    }
    $null = $steps.Add(@{ Section="Python"; Name="python (not found)"; Status="SKIP"; ExitCode=0; Output="" })
}

# --- Build report ---

$failedSteps = @($steps | Where-Object { $_.Status -eq "FAIL" })
$passedSteps = @($steps | Where-Object { $_.Status -eq "PASS" })
$skippedSteps = @($steps | Where-Object { $_.Status -eq "SKIP" })

$passCount = $passedSteps.Count
$failCount = $failedSteps.Count
$skipCount = $skippedSteps.Count
$totalCount = $steps.Count
$allPass = ($failCount -eq 0)
$overallStatus = if ($allPass) { "ALL PASS" } else { "FAILING" }

$lines = [System.Collections.ArrayList]::new()
$null = $lines.Add("# CHECK RESULTS")
$null = $lines.Add("")
$null = $lines.Add("## Run Info")
$null = $lines.Add("- **Version:** $version")
$null = $lines.Add("- **Commit:** $commit")
$null = $lines.Add("- **Branch:** $branch")
$null = $lines.Add("- **Date:** $date")
$null = $lines.Add("- **Machine:** $machine")
$null = $lines.Add("- **Overall:** $overallStatus ($passCount pass, $failCount fail, $skipCount skip / $totalCount total)")
$null = $lines.Add("")
$null = $lines.Add("## Steps")

$currentSection = ""
foreach ($step in $steps) {
    if ($step.Section -ne $currentSection) {
        $null = $lines.Add("")
        $null = $lines.Add("### $($step.Section)")
        $currentSection = $step.Section
    }
    $icon = switch ($step.Status) { "PASS" { "[x]" } "FAIL" { "[ ]" } "SKIP" { "[-]" } }
    $null = $lines.Add("- $icon ${bt}$($step.Name)${bt}: **$($step.Status)**")
}

# --- Failure details ---

if ($failCount -gt 0) {
    $null = $lines.Add("")
    $null = $lines.Add("---")
    $null = $lines.Add("")
    $null = $lines.Add("## Failure Details")

    foreach ($f in $failedSteps) {
        $null = $lines.Add("")
        $null = $lines.Add("### $($f.Section): $($f.Name)")
        $null = $lines.Add("Exit code: $($f.ExitCode)")
        $null = $lines.Add("")

        if ($f.Output) {
            $outputLines = $f.Output -split "\r?\n"
            if ($outputLines.Count -gt 100) {
                $outputLines = $outputLines[($outputLines.Count - 100)..($outputLines.Count - 1)]
                $null = $lines.Add("*(truncated to last 100 lines)*")
                $null = $lines.Add("")
            }
            $null = $lines.Add("${bt}${bt}${bt}text")
            foreach ($ol in $outputLines) {
                $null = $lines.Add($ol)
            }
            $null = $lines.Add("${bt}${bt}${bt}")
        }
    }
} else {
    $null = $lines.Add("")
    $null = $lines.Add("---")
    $null = $lines.Add("")
    $null = $lines.Add("All checks passed. No errors to report.")
}

# --- Test summary ---

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Test Summary")

$cargoTestStep = $steps | Where-Object { $_.Name -eq "cargo test" } | Select-Object -First 1
if ($cargoTestStep -and $cargoTestStep.Output) {
    $testResultLines = @($cargoTestStep.Output -split "\r?\n" | Where-Object { $_ -match "^test result:" })
    if ($testResultLines.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("### Rust")
        foreach ($tl in $testResultLines) {
            $null = $lines.Add("- ${bt}$($tl.Trim())${bt}")
        }
    }
}

$pytestStep = $steps | Where-Object { $_.Name -eq "pytest" } | Select-Object -First 1
if ($pytestStep -and $pytestStep.Output) {
    $summaryLine = $pytestStep.Output -split "\r?\n" | Where-Object { $_ -match "passed|failed|error|skipped" } | Select-Object -Last 1
    if ($summaryLine) {
        $null = $lines.Add("")
        $null = $lines.Add("### Python")
        $null = $lines.Add("- ${bt}$($summaryLine.Trim())${bt}")
    }
}

# Write report
($lines -join "`r`n") | Out-File -FilePath $reportFile -Encoding utf8 -Force

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
