# Unified CI check: Rust (fmt + clippy + test) + Python (maturin + pytest)
# Usage: just check  (run on ASUS TUF after pull)
#
# Produces:
#   .build-results/CHECK_RESULTS.md -- structured pass/fail with per-module breakdown
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

    # Run pytest with verbose + warnings summary
    $null = $steps.Add((Run-Step "Python" "pytest" "python -m pytest api/tests/ -v --tb=short -W default::DeprecationWarning -W default::PendingDeprecationWarning"))
} else {
    if (-not $hasPython) {
        Write-Host ""
        Write-Host "python not found -- skipping Python checks" -ForegroundColor Yellow
    }
    $null = $steps.Add(@{ Section="Python"; Name="python (not found)"; Status="SKIP"; ExitCode=0; Output="" })
}

# --- Studio checks ---

$hasNode = Get-Command node -ErrorAction SilentlyContinue
if ($hasNode -and (Test-Path "studio/package.json")) {
    Write-Host ""
    Write-Host "=== Studio ===" -ForegroundColor Cyan
    $null = $steps.Add((Run-Step "Studio" "npm ci" "pushd studio; npm ci --prefer-offline; popd"))
    $null = $steps.Add((Run-Step "Studio" "vitest" "pushd studio; npx vitest run; popd"))
    $null = $steps.Add((Run-Step "Studio" "vite build" "pushd studio; npx vite build; popd"))
} else {
    if (-not $hasNode) {
        Write-Host ""
        Write-Host "node not found -- skipping Studio checks" -ForegroundColor Yellow
    }
    if ($hasNode -and -not (Test-Path "studio/package.json")) {
        Write-Host ""
        Write-Host "studio/package.json not found -- skipping Studio checks" -ForegroundColor Yellow
    }
    $null = $steps.Add(@{ Section="Studio"; Name="node/studio (not found)"; Status="SKIP"; ExitCode=0; Output="" })
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
$null = $lines.Add("| Field | Value |")
$null = $lines.Add("|-------|-------|")
$null = $lines.Add("| Version | $version |")
$null = $lines.Add("| Commit | ${bt}$commit${bt} |")
$null = $lines.Add("| Branch | $branch |")
$null = $lines.Add("| Date | $date |")
$null = $lines.Add("| Machine | $machine |")
$null = $lines.Add("| Overall | **$overallStatus** ($passCount pass, $failCount fail, $skipCount skip / $totalCount steps) |")
$null = $lines.Add("")

# --- Steps table ---

$null = $lines.Add("## Steps")
$null = $lines.Add("")
$null = $lines.Add("| # | Section | Step | Result |")
$null = $lines.Add("|---|---------|------|--------|")

$stepNum = 0
foreach ($step in $steps) {
    $stepNum++
    $icon = switch ($step.Status) { "PASS" { "PASS" } "FAIL" { "**FAIL**" } "SKIP" { "SKIP" } }
    $null = $lines.Add("| $stepNum | $($step.Section) | ${bt}$($step.Name)${bt} | $icon |")
}

# ============================
# Rust test breakdown by crate
# ============================

$cargoTestStep = $steps | Where-Object { $_.Name -eq "cargo test" } | Select-Object -First 1

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Rust Test Breakdown")

if ($cargoTestStep -and $cargoTestStep.Output) {
    $cargoOutput = $cargoTestStep.Output -split "\r?\n"

    # Parse: "Running unittests src/lib.rs (target\debug\deps\spdf_core-xxx)"
    # Followed by test lines, then "test result: ok. N passed; ..."
    $rustModules = [System.Collections.ArrayList]::new()
    $currentCrate = ""
    $rustTotalPassed = 0
    $rustTotalFailed = 0
    $rustWarnings = [System.Collections.ArrayList]::new()

    foreach ($rline in $cargoOutput) {
        # Detect crate being compiled/tested
        if ($rline -match "Running\s+(unittests\s+)?(.+?)\s+\(") {
            $currentCrate = $Matches[2]
            # Clean up: extract just the source file path
            if ($currentCrate -match "([^/\\]+)\.rs$") {
                # keep as-is
            }
        }
        # Detect doc-tests
        if ($rline -match "Doc-tests\s+(\S+)") {
            $currentCrate = "doc-tests/$($Matches[1])"
        }
        # Detect "Compiling" to get crate names
        if ($rline -match "^\s*Compiling\s+(\S+)\s+v") {
            # just informational, skip
        }
        # Parse test result lines
        if ($rline -match "test result: ok\.\s+(\d+) passed;\s+(\d+) failed;") {
            $passed = [int]$Matches[1]
            $failed = [int]$Matches[2]
            if ($passed -gt 0 -or $failed -gt 0) {
                $null = $rustModules.Add(@{
                    Source = $currentCrate
                    Passed = $passed
                    Failed = $failed
                })
                $rustTotalPassed += $passed
                $rustTotalFailed += $failed
            }
        }
        if ($rline -match "test result: FAILED\.\s+(\d+) passed;\s+(\d+) failed;") {
            $passed = [int]$Matches[1]
            $failed = [int]$Matches[2]
            $null = $rustModules.Add(@{
                Source = $currentCrate
                Passed = $passed
                Failed = $failed
            })
            $rustTotalPassed += $passed
            $rustTotalFailed += $failed
        }
        # Capture Rust warnings
        if ($rline -match "^warning(\[.+?\])?:\s+(.+)") {
            $warnText = $rline.Trim()
            if ($warnText -notmatch "generated \d+ warning") {
                $null = $rustWarnings.Add($warnText)
            }
        }
    }

    if ($rustModules.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("| Module | Passed | Failed |")
        $null = $lines.Add("|--------|--------|--------|")
        foreach ($mod in $rustModules) {
            $failStr = if ($mod.Failed -gt 0) { "**$($mod.Failed)**" } else { "0" }
            $null = $lines.Add("| ${bt}$($mod.Source)${bt} | $($mod.Passed) | $failStr |")
        }
        $null = $lines.Add("| **Total** | **$rustTotalPassed** | **$rustTotalFailed** |")
    } else {
        $null = $lines.Add("")
        $null = $lines.Add("*No Rust test results found.*")
    }

    # Rust warnings
    if ($rustWarnings.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("### Rust Warnings ($($rustWarnings.Count))")
        $null = $lines.Add("")
        $null = $lines.Add("${bt}${bt}${bt}text")
        foreach ($rw in $rustWarnings) {
            $null = $lines.Add($rw)
        }
        $null = $lines.Add("${bt}${bt}${bt}")
    }
} else {
    $null = $lines.Add("")
    $null = $lines.Add("*Rust tests were not run.*")
}

# ================================
# Python test breakdown by module
# ================================

$pytestStep = $steps | Where-Object { $_.Name -eq "pytest" } | Select-Object -First 1

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Python Test Breakdown")

if ($pytestStep -and $pytestStep.Output) {
    $pyOutput = $pytestStep.Output -split "\r?\n"

    # Parse pytest -v output lines like: "api/tests/test_documents.py::test_name PASSED"
    $pyModuleCounts = [ordered]@{}
    $pyTotalPassed = 0
    $pyTotalFailed = 0
    $pyTotalSkipped = 0
    $pyWarnings = [System.Collections.ArrayList]::new()
    $inWarnings = $false

    foreach ($pline in $pyOutput) {
        # Match pytest verbose result lines
        if ($pline -match "^(api/tests/\S+\.py)::\S+\s+(PASSED|FAILED|SKIPPED|ERROR|XFAIL|XPASS)") {
            $module = $Matches[1]
            $result = $Matches[2]
            if (-not $pyModuleCounts.Contains($module)) {
                $pyModuleCounts[$module] = @{ Passed=0; Failed=0; Skipped=0 }
            }
            switch ($result) {
                "PASSED"  { $pyModuleCounts[$module].Passed++; $pyTotalPassed++ }
                "FAILED"  { $pyModuleCounts[$module].Failed++; $pyTotalFailed++ }
                "SKIPPED" { $pyModuleCounts[$module].Skipped++; $pyTotalSkipped++ }
                "XFAIL"   { $pyModuleCounts[$module].Skipped++; $pyTotalSkipped++ }
                "ERROR"   { $pyModuleCounts[$module].Failed++; $pyTotalFailed++ }
            }
        }
        # Also match Windows-style paths (backslash)
        if ($pline -match "^(api\\tests\\\S+\.py)::\S+\s+(PASSED|FAILED|SKIPPED|ERROR|XFAIL|XPASS)") {
            $module = ($Matches[1] -replace "\\", "/")
            $result = $Matches[2]
            if (-not $pyModuleCounts.Contains($module)) {
                $pyModuleCounts[$module] = @{ Passed=0; Failed=0; Skipped=0 }
            }
            switch ($result) {
                "PASSED"  { $pyModuleCounts[$module].Passed++; $pyTotalPassed++ }
                "FAILED"  { $pyModuleCounts[$module].Failed++; $pyTotalFailed++ }
                "SKIPPED" { $pyModuleCounts[$module].Skipped++; $pyTotalSkipped++ }
                "XFAIL"   { $pyModuleCounts[$module].Skipped++; $pyTotalSkipped++ }
                "ERROR"   { $pyModuleCounts[$module].Failed++; $pyTotalFailed++ }
            }
        }

        # Capture warnings section
        if ($pline -match "^={2,}\s*warnings summary\s*={2,}$") {
            $inWarnings = $true
            continue
        }
        if ($inWarnings) {
            if ($pline -match "^={2,}\s*" -or $pline -match "^-{2,}\s*") {
                $inWarnings = $false
                continue
            }
            $trimmed = $pline.Trim()
            if ($trimmed -ne "" -and $trimmed -ne "-- Docs: https://docs.pytest.org/en/stable/how-to/capture-warnings.html") {
                $null = $pyWarnings.Add($trimmed)
            }
        }
    }

    if ($pyModuleCounts.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("| Module | Passed | Failed | Skipped |")
        $null = $lines.Add("|--------|--------|--------|---------|")
        foreach ($modKey in $pyModuleCounts.Keys) {
            $mc = $pyModuleCounts[$modKey]
            $modName = $modKey -replace "^api/tests/", ""
            $failStr = if ($mc.Failed -gt 0) { "**$($mc.Failed)**" } else { "0" }
            $skipStr = if ($mc.Skipped -gt 0) { "$($mc.Skipped)" } else { "0" }
            $null = $lines.Add("| ${bt}$modName${bt} | $($mc.Passed) | $failStr | $skipStr |")
        }
        $null = $lines.Add("| **Total** | **$pyTotalPassed** | **$pyTotalFailed** | **$pyTotalSkipped** |")
    } else {
        $null = $lines.Add("")
        $null = $lines.Add("*No Python test results found (could not parse pytest -v output).*")
    }

    # Python warnings
    if ($pyWarnings.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("### Python Warnings ($($pyWarnings.Count))")
        $null = $lines.Add("")
        $null = $lines.Add("${bt}${bt}${bt}text")
        foreach ($pw in $pyWarnings) {
            $null = $lines.Add($pw)
        }
        $null = $lines.Add("${bt}${bt}${bt}")
    }

    # Python summary line (the final pytest summary)
    $pySummaryLine = $pyOutput | Where-Object { $_ -match "^\s*={2,}.*(passed|failed|error).*={2,}\s*$" } | Select-Object -Last 1
    if ($pySummaryLine) {
        $null = $lines.Add("")
        $null = $lines.Add("**Summary:** ${bt}$($pySummaryLine.Trim())${bt}")
    }
} else {
    $null = $lines.Add("")
    $null = $lines.Add("*Python tests were not run.*")
}

# ================================
# Studio test breakdown
# ================================

$vitestStep = $steps | Where-Object { $_.Name -eq "vitest" } | Select-Object -First 1
$studioTotalPassed = 0
$studioTotalFailed = 0

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Studio Test Breakdown")

if ($vitestStep -and $vitestStep.Output) {
    $vitestOutput = $vitestStep.Output -split "\r?\n"

    # Parse vitest output: "Tests  42 passed (42)"  or "Tests  3 failed | 39 passed (42)"
    foreach ($vline in $vitestOutput) {
        if ($vline -match "Tests\s+(\d+)\s+passed") {
            $studioTotalPassed = [int]$Matches[1]
        }
        if ($vline -match "(\d+)\s+failed") {
            $studioTotalFailed = [int]$Matches[1]
        }
    }

    # Parse per-file results: " PASS  src/__tests__/App.test.tsx (5 tests)"
    $studioFiles = [System.Collections.ArrayList]::new()
    foreach ($vline in $vitestOutput) {
        if ($vline -match "(PASS|FAIL)\s+(\S+\.test\.\S+)\s*\((\d+)\s+test") {
            $null = $studioFiles.Add(@{
                Status = $Matches[1]
                File   = $Matches[2]
                Count  = [int]$Matches[3]
            })
        }
    }

    if ($studioFiles.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("| File | Tests | Result |")
        $null = $lines.Add("|------|-------|--------|")
        foreach ($sf in $studioFiles) {
            $icon = if ($sf.Status -eq "PASS") { "PASS" } else { "**FAIL**" }
            $null = $lines.Add("| ${bt}$($sf.File)${bt} | $($sf.Count) | $icon |")
        }
        $null = $lines.Add("| **Total** | **$($studioTotalPassed + $studioTotalFailed)** | |")
    } else {
        $null = $lines.Add("")
        $null = $lines.Add("*Vitest: $studioTotalPassed passed, $studioTotalFailed failed*")
    }

    # Vitest summary line
    $vitestSummary = $vitestOutput | Where-Object { $_ -match "Tests\s+\d+" } | Select-Object -Last 1
    if ($vitestSummary) {
        $null = $lines.Add("")
        $null = $lines.Add("**Summary:** ${bt}$($vitestSummary.Trim())${bt}")
    }
} else {
    $null = $lines.Add("")
    $null = $lines.Add("*Studio tests were not run.*")
}

# ================================
# Grand total
# ================================

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Grand Total")
$null = $lines.Add("")

$grandPassed = $rustTotalPassed + $pyTotalPassed + $studioTotalPassed
$grandFailed = $rustTotalFailed + $pyTotalFailed + $studioTotalFailed
$grandSkipped = $pyTotalSkipped
$grandTotal = $grandPassed + $grandFailed + $grandSkipped

$null = $lines.Add("| | Passed | Failed | Skipped | Total |")
$null = $lines.Add("|--|--------|--------|---------|-------|")
$null = $lines.Add("| Rust | $rustTotalPassed | $rustTotalFailed | 0 | $($rustTotalPassed + $rustTotalFailed) |")
$null = $lines.Add("| Python | $pyTotalPassed | $pyTotalFailed | $pyTotalSkipped | $($pyTotalPassed + $pyTotalFailed + $pyTotalSkipped) |")
$null = $lines.Add("| Studio | $studioTotalPassed | $studioTotalFailed | 0 | $($studioTotalPassed + $studioTotalFailed) |")
$null = $lines.Add("| **Total** | **$grandPassed** | **$grandFailed** | **$grandSkipped** | **$grandTotal** |")

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
}

# Write report
($lines -join "`r`n") | Out-File -FilePath $reportFile -Encoding utf8 -Force

# --- Console summary ---

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
if ($allPass) {
    Write-Host "  ALL CHECKS PASSED ($passCount/$totalCount steps)" -ForegroundColor Green
    Write-Host "  Tests: $grandPassed passed, $grandFailed failed, $grandSkipped skipped ($grandTotal total)" -ForegroundColor Green
} else {
    Write-Host "  CHECKS FAILING ($failCount/$totalCount steps failed)" -ForegroundColor Red
    Write-Host "  Tests: $grandPassed passed, $grandFailed failed, $grandSkipped skipped ($grandTotal total)" -ForegroundColor Red
}
Write-Host "  Report: $reportFile" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor White
Write-Host "  git add .build-results" -ForegroundColor Yellow
Write-Host "  git commit -m 'check: $version'" -ForegroundColor Yellow
Write-Host "  git push" -ForegroundColor Yellow
