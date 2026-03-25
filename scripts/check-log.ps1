# Run full Rust CI checks and capture logs for review
# Usage: just check-log  (run on ASUS TUF after pull)
#
# Produces:
#   .build-results/CHECK_RESULTS.md  — summary with pass/fail per step
#   .build-results/check-fmt.log     — cargo fmt output
#   .build-results/check-clippy.log  — cargo clippy output
#   .build-results/check-test.log    — cargo test output
#
# After running, push to GHE:
#   git add .build-results && git commit -m "check: <version>" && git push

$ErrorActionPreference = "Continue"

# Refresh PATH to pick up tools installed in current session
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
$cargoBin = "$env:USERPROFILE\.cargo\bin"
if (Test-Path $cargoBin) { $env:Path += ";$cargoBin" }

$resultsDir = ".build-results"
$reportFile = "$resultsDir\CHECK_RESULTS.md"

if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

$version = if (Test-Path "VERSION") { (Get-Content "VERSION" -Raw).Trim() } else { "unknown" }
$commit  = & git rev-parse --short HEAD 2>$null; if (-not $commit) { $commit = "unknown" }
$branch  = & git branch --show-current 2>$null; if (-not $branch) { $branch = "unknown" }
$date    = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
$machine = $env:COMPUTERNAME

# Define steps as parallel arrays (avoids hashtable issues in PS)
$stepNames = @("cargo fmt --check", "cargo clippy", "cargo test")
$stepLogs  = @("check-fmt.log", "check-clippy.log", "check-test.log")
$stepCmds  = @(
    "cargo fmt --all -- --check",
    "cargo clippy --workspace -- -D warnings",
    "cargo test --workspace"
)

$resultStatus = @("", "", "")
$allPass = $true

for ($i = 0; $i -lt $stepNames.Length; $i++) {
    $name    = $stepNames[$i]
    $logFile = $stepLogs[$i]
    $cmd     = $stepCmds[$i]
    $logPath = "$resultsDir\$logFile"

    Write-Host "Running: $name ..." -NoNewline

    # Use Invoke-Expression with redirection to capture all output
    Invoke-Expression "$cmd *> `"$logPath`" 2>&1"

    if ($LASTEXITCODE -eq 0) {
        Write-Host " PASS" -ForegroundColor Green
        $resultStatus[$i] = "PASS"
    } else {
        Write-Host " FAIL (exit $LASTEXITCODE)" -ForegroundColor Red
        $resultStatus[$i] = "FAIL"
        $allPass = $false
    }
}

# Build the report
$overallStatus = if ($allPass) { "PASS" } else { "FAIL" }

$report = @"
# Check Results

## Run Info
- **Version:** $version
- **Commit:** $commit
- **Branch:** $branch
- **Date:** $date
- **Machine:** $machine
- **Overall:** $overallStatus

## Steps
"@

for ($i = 0; $i -lt $stepNames.Length; $i++) {
    $s = $resultStatus[$i]
    $icon = if ($s -eq "PASS") { "[x]" } else { "[ ]" }
    $report += "`n- $icon $($stepNames[$i]): $s"
}

# Append full logs for failures
$hasFailure = $false
for ($i = 0; $i -lt $stepNames.Length; $i++) {
    if ($resultStatus[$i] -eq "FAIL") {
        $hasFailure = $true
        $logPath = "$resultsDir\$($stepLogs[$i])"
        if (Test-Path $logPath) {
            $tail = (Get-Content $logPath -Tail 80) -join "`n"
            $report += "`n`n### $($stepNames[$i]) (last 80 lines)`n`````````n$tail`n``````````n"
        }
    }
}

if (-not $hasFailure) {
    $report += "`n`nAll checks passed. No error logs to show."
}

$report | Out-File -FilePath $reportFile -Encoding utf8 -Force

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
if ($allPass) {
    Write-Host "  ALL CHECKS PASSED" -ForegroundColor Green
} else {
    Write-Host "  SOME CHECKS FAILED - see $reportFile" -ForegroundColor Red
}
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor White
Write-Host "  git add .build-results" -ForegroundColor Yellow
Write-Host "  git commit -m 'check: $version'" -ForegroundColor Yellow
Write-Host "  git push" -ForegroundColor Yellow
