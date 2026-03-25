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

$steps = @(
    @{ Name = "cargo fmt --check"; Log = "check-fmt.log";    Cmd = @("cargo", "fmt", "--all", "--", "--check") },
    @{ Name = "cargo clippy";      Log = "check-clippy.log"; Cmd = @("cargo", "clippy", "--workspace", "--", "-D", "warnings") },
    @{ Name = "cargo test";        Log = "check-test.log";   Cmd = @("cargo", "test", "--workspace") }
)

$results = @{}
$allPass = $true

foreach ($step in $steps) {
    $logPath = "$resultsDir\$($step.Log)"
    Write-Host "Running: $($step.Name) ..." -NoNewline
    $LASTEXITCODE = 0

    try {
        & $step.Cmd[0] $step.Cmd[1..($step.Cmd.Length - 1)] *> $logPath
        if ($LASTEXITCODE -eq 0) {
            Write-Host " PASS" -ForegroundColor Green
            $results[$step.Name] = "PASS"
        } else {
            Write-Host " FAIL (exit $LASTEXITCODE)" -ForegroundColor Red
            $results[$step.Name] = "FAIL"
            $allPass = $false
        }
    } catch {
        $_.Exception.Message | Out-File -FilePath $logPath -Encoding utf8
        Write-Host " FAIL (exception)" -ForegroundColor Red
        $results[$step.Name] = "FAIL"
        $allPass = $false
    }
}

# Build the report
function Icon($status) {
    if ($status -eq "PASS") { return "[x]" } else { return "[ ]" }
}

$overallIcon = if ($allPass) { "PASS" } else { "FAIL" }

$report = @"
# Check Results

## Run Info
- **Version:** $version
- **Commit:** $commit
- **Branch:** $branch
- **Date:** $date
- **Machine:** $machine
- **Overall:** $overallIcon

## Steps
"@

foreach ($step in $steps) {
    $s = $results[$step.Name]
    $report += "`n- $(Icon $s) $($step.Name): $s"
}

# Append full logs for any failures
$hasFailure = $false
foreach ($step in $steps) {
    if ($results[$step.Name] -eq "FAIL") {
        $hasFailure = $true
        $logPath = "$resultsDir\$($step.Log)"
        $logContent = Get-Content $logPath -Raw -ErrorAction SilentlyContinue
        if ($logContent) {
            $tail = (Get-Content $logPath -Tail 80) -join "`n"
            $report += "`n`n### $($step.Name) (last 80 lines)`n``````n$tail`n```````n"
        }
    }
}

# If all passed, note it
if (-not $hasFailure) {
    $report += "`n`nAll checks passed. No error logs to show."
}

$report | Out-File -FilePath $reportFile -Encoding utf8 -Force

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
if ($allPass) {
    Write-Host "  ALL CHECKS PASSED" -ForegroundColor Green
} else {
    Write-Host "  SOME CHECKS FAILED — see $reportFile" -ForegroundColor Red
}
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor White
Write-Host "  git add .build-results" -ForegroundColor Yellow
Write-Host "  git commit -m 'check: $version'" -ForegroundColor Yellow
Write-Host "  git push" -ForegroundColor Yellow
