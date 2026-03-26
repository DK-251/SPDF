# Unified CI check: Rust + Python + Studio
# Produces: .build-results/CHECK_RESULTS.md + .build-results/check-session.log

# --- Transcript FIRST (captures everything from this point) ---
$resultsDir = ".build-results"
if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }
$sessionLog = Join-Path $resultsDir "check-session.log"
try { Stop-Transcript -ErrorAction SilentlyContinue } catch { }
Start-Transcript -Path $sessionLog -Force | Out-Null

$ErrorActionPreference = "Continue"

# Backtick for markdown code fences
$bt = [char]96

# ANSI escape pattern: build via subexpression to avoid PS parse ambiguity
$ansiPattern = "$([char]27)" + '\[[0-9;]*[A-Za-z]'

# Refresh PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
if (Test-Path $cargoBin) { $env:Path += ";$cargoBin" }

$reportFile = Join-Path $resultsDir "CHECK_RESULTS.md"

$version = "unknown"
if (Test-Path "VERSION") { $version = (Get-Content "VERSION" -Raw).Trim() }

$commit = "unknown"
$commitOut = & git rev-parse --short HEAD 2>$null
if ($commitOut) { $commit = $commitOut }

$branch = "unknown"
$branchOut = & git branch --show-current 2>$null
if ($branchOut) { $branch = $branchOut }

$date = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
$machine = $env:COMPUTERNAME

Write-Host "SPDF Check -- $version ($commit) on $machine" -ForegroundColor Cyan
Write-Host "Started: $date" -ForegroundColor DarkGray
Write-Host ""

# --- Step runner ---

$steps = New-Object System.Collections.ArrayList

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

# Helper: strip ANSI escape codes
function Remove-AnsiCodes {
    param([string]$Text)
    return [regex]::Replace($Text, $script:ansiPattern, '')
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
    $null = $steps.Add((Run-Step "Studio" "npm install" "pushd studio; npm install; popd"))
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

$overallStatus = "FAILING"
if ($allPass) { $overallStatus = "ALL PASS" }

$lines = New-Object System.Collections.ArrayList
$null = $lines.Add("# CHECK RESULTS")
$null = $lines.Add("")
$null = $lines.Add("## Run Info")
$null = $lines.Add("| Field | Value |")
$null = $lines.Add("|-------|-------|")
$null = $lines.Add("| Version | $version |")
$null = $lines.Add("| Commit | ${bt}${commit}${bt} |")
$null = $lines.Add("| Branch | $branch |")
$null = $lines.Add("| Date | $date |")
$null = $lines.Add("| Machine | $machine |")
$null = $lines.Add("| Overall | **${overallStatus}** ($passCount pass, $failCount fail, $skipCount skip / $totalCount steps) |")
$null = $lines.Add("")

# Steps table
$null = $lines.Add("## Steps")
$null = $lines.Add("")
$null = $lines.Add("| # | Section | Step | Result |")
$null = $lines.Add("|---|---------|------|--------|")

$stepNum = 0
foreach ($step in $steps) {
    $stepNum++
    $icon = $step.Status
    if ($step.Status -eq "FAIL") { $icon = "**FAIL**" }
    $null = $lines.Add("| $stepNum | $($step.Section) | ${bt}$($step.Name)${bt} | $icon |")
}

# ============================
# Rust test breakdown
# ============================

$cargoTestStep = $steps | Where-Object { $_.Name -eq "cargo test" } | Select-Object -First 1

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Rust Test Breakdown")

$rustTotalPassed = 0
$rustTotalFailed = 0

if ($cargoTestStep -and $cargoTestStep.Output) {
    $cargoOutput = $cargoTestStep.Output -split "`r?`n"

    $rustModules = New-Object System.Collections.ArrayList
    $currentCrate = ""
    $rustWarnings = New-Object System.Collections.ArrayList

    foreach ($rline in $cargoOutput) {
        if ($rline -match 'Running\s+(unittests\s+)?(.+?)\s+\(') {
            $currentCrate = $Matches[2]
        }
        if ($rline -match 'Doc-tests\s+(\S+)') {
            $currentCrate = "doc-tests/" + $Matches[1]
        }
        if ($rline -match 'test result: ok\.\s+(\d+) passed;\s+(\d+) failed;') {
            $passed = [int]$Matches[1]
            $failed = [int]$Matches[2]
            if (($passed -gt 0) -or ($failed -gt 0)) {
                $null = $rustModules.Add(@{ Source = $currentCrate; Passed = $passed; Failed = $failed })
                $rustTotalPassed += $passed
                $rustTotalFailed += $failed
            }
        }
        if ($rline -match 'test result: FAILED\.\s+(\d+) passed;\s+(\d+) failed;') {
            $passed = [int]$Matches[1]
            $failed = [int]$Matches[2]
            $null = $rustModules.Add(@{ Source = $currentCrate; Passed = $passed; Failed = $failed })
            $rustTotalPassed += $passed
            $rustTotalFailed += $failed
        }
        if ($rline -match '^warning(\[.+?\])?:\s+(.+)') {
            $warnText = $rline.Trim()
            if ($warnText -notmatch 'generated \d+ warning') {
                $null = $rustWarnings.Add($warnText)
            }
        }
    }

    if ($rustModules.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("| Module | Passed | Failed |")
        $null = $lines.Add("|--------|--------|--------|")
        foreach ($mod in $rustModules) {
            $failStr = "0"
            if ($mod.Failed -gt 0) { $failStr = "**$($mod.Failed)**" }
            $null = $lines.Add("| ${bt}$($mod.Source)${bt} | $($mod.Passed) | $failStr |")
        }
        $null = $lines.Add("| **Total** | **$rustTotalPassed** | **$rustTotalFailed** |")
    } else {
        $null = $lines.Add("")
        $null = $lines.Add("*No Rust test results found.*")
    }

    if ($rustWarnings.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("### Rust Warnings ($($rustWarnings.Count))")
        $null = $lines.Add("")
        $null = $lines.Add("${bt}${bt}${bt}text")
        foreach ($rw in $rustWarnings) { $null = $lines.Add($rw) }
        $null = $lines.Add("${bt}${bt}${bt}")
    }
} else {
    $null = $lines.Add("")
    $null = $lines.Add("*Rust tests were not run.*")
}

# ============================
# Python test breakdown
# ============================

$pytestStep = $steps | Where-Object { $_.Name -eq "pytest" } | Select-Object -First 1

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Python Test Breakdown")

$pyTotalPassed = 0
$pyTotalFailed = 0
$pyTotalSkipped = 0

if ($pytestStep -and $pytestStep.Output) {
    $pyOutput = $pytestStep.Output -split "`r?`n"

    $pyModuleCounts = [ordered]@{}
    $pyWarnings = New-Object System.Collections.ArrayList
    $inWarnings = $false

    foreach ($pline in $pyOutput) {
        # Forward-slash paths
        if ($pline -match '^(api/tests/\S+\.py)::\S+\s+(PASSED|FAILED|SKIPPED|ERROR|XFAIL|XPASS)') {
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
        # Backslash paths (Windows)
        if ($pline -match '^(api\\tests\\\S+\.py)::\S+\s+(PASSED|FAILED|SKIPPED|ERROR|XFAIL|XPASS)') {
            $module = $Matches[1] -replace '\\', '/'
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

        # Warnings section
        if ($pline -match '^={2,}\s*warnings summary\s*={2,}$') {
            $inWarnings = $true
            continue
        }
        if ($inWarnings) {
            if (($pline -match '^={2,}\s*') -or ($pline -match '^-{2,}\s*')) {
                $inWarnings = $false
                continue
            }
            $trimmed = $pline.Trim()
            if (($trimmed -ne '') -and ($trimmed -ne '-- Docs: https://docs.pytest.org/en/stable/how-to/capture-warnings.html')) {
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
            $modName = $modKey -replace '^api/tests/', ''
            $failStr = "0"
            if ($mc.Failed -gt 0) { $failStr = "**$($mc.Failed)**" }
            $skipStr = "0"
            if ($mc.Skipped -gt 0) { $skipStr = "$($mc.Skipped)" }
            $null = $lines.Add("| ${bt}${modName}${bt} | $($mc.Passed) | $failStr | $skipStr |")
        }
        $null = $lines.Add("| **Total** | **$pyTotalPassed** | **$pyTotalFailed** | **$pyTotalSkipped** |")
    } else {
        $null = $lines.Add("")
        $null = $lines.Add("*No Python test results found.*")
    }

    if ($pyWarnings.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("### Python Warnings ($($pyWarnings.Count))")
        $null = $lines.Add("")
        $null = $lines.Add("${bt}${bt}${bt}text")
        foreach ($pw in $pyWarnings) { $null = $lines.Add($pw) }
        $null = $lines.Add("${bt}${bt}${bt}")
    }

    $pySummaryLine = $pyOutput | Where-Object { $_ -match '^\s*={2,}.*(passed|failed|error).*={2,}\s*$' } | Select-Object -Last 1
    if ($pySummaryLine) {
        $null = $lines.Add("")
        $null = $lines.Add("**Summary:** ${bt}$($pySummaryLine.Trim())${bt}")
    }
} else {
    $null = $lines.Add("")
    $null = $lines.Add("*Python tests were not run.*")
}

# ============================
# Studio test breakdown
# ============================

$vitestStep = $steps | Where-Object { $_.Name -eq "vitest" } | Select-Object -First 1
$studioTotalPassed = 0
$studioTotalFailed = 0

$null = $lines.Add("")
$null = $lines.Add("---")
$null = $lines.Add("")
$null = $lines.Add("## Studio Test Breakdown")

if ($vitestStep -and $vitestStep.Output) {
    # Strip ANSI codes
    $cleanOutput = Remove-AnsiCodes $vitestStep.Output
    $vitestOutput = $cleanOutput -split "`r?`n"

    # Parse totals: "Tests  70 passed (70)" or "Tests  1 failed | 69 passed (70)"
    foreach ($vline in $vitestOutput) {
        if ($vline -match 'Tests\s+.*?(\d+)\s+passed') {
            $studioTotalPassed = [int]$Matches[1]
        }
        if ($vline -match 'Tests\s+.*?(\d+)\s+failed') {
            $studioTotalFailed = [int]$Matches[1]
        }
    }

    # Parse per-file: "src/__tests__/App.test.tsx (5 tests) 42ms"
    $studioFiles = New-Object System.Collections.ArrayList
    foreach ($vline in $vitestOutput) {
        if ($vline -match '\s+(\S+\.test\.\S+)\s+\((\d+)\s+test') {
            $fName = $Matches[1]
            $fCount = [int]$Matches[2]
            $fStatus = "PASS"
            if ($vline -match 'FAIL') { $fStatus = "FAIL" }
            $null = $studioFiles.Add(@{ Status = $fStatus; File = $fName; Count = $fCount })
        }
    }

    if ($studioFiles.Count -gt 0) {
        $null = $lines.Add("")
        $null = $lines.Add("| File | Tests | Result |")
        $null = $lines.Add("|------|-------|--------|")
        foreach ($sf in $studioFiles) {
            $sfIcon = "PASS"
            if ($sf.Status -eq "FAIL") { $sfIcon = "**FAIL**" }
            $null = $lines.Add("| ${bt}$($sf.File)${bt} | $($sf.Count) | $sfIcon |")
        }
        $sfTotal = $studioTotalPassed + $studioTotalFailed
        $null = $lines.Add("| **Total** | **$sfTotal** | |")
    } else {
        $null = $lines.Add("")
        $null = $lines.Add("*Vitest: $studioTotalPassed passed, $studioTotalFailed failed*")
    }

    $vitestSummary = $vitestOutput | Where-Object { $_ -match 'Tests\s+.*\d+' } | Select-Object -Last 1
    if ($vitestSummary) {
        $null = $lines.Add("")
        $null = $lines.Add("**Summary:** ${bt}$($vitestSummary.Trim())${bt}")
    }
} else {
    $null = $lines.Add("")
    $null = $lines.Add("*Studio tests were not run.*")
}

# ============================
# Grand total
# ============================

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
$rustTotal = $rustTotalPassed + $rustTotalFailed
$pyTotal = $pyTotalPassed + $pyTotalFailed + $pyTotalSkipped
$stTotal = $studioTotalPassed + $studioTotalFailed
$null = $lines.Add("| Rust | $rustTotalPassed | $rustTotalFailed | 0 | $rustTotal |")
$null = $lines.Add("| Python | $pyTotalPassed | $pyTotalFailed | $pyTotalSkipped | $pyTotal |")
$null = $lines.Add("| Studio | $studioTotalPassed | $studioTotalFailed | 0 | $stTotal |")
$null = $lines.Add("| **Total** | **$grandPassed** | **$grandFailed** | **$grandSkipped** | **$grandTotal** |")

# Failure details
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
            $outputLines = $f.Output -split "`r?`n"
            if ($outputLines.Count -gt 100) {
                $startIdx = $outputLines.Count - 100
                $endIdx = $outputLines.Count - 1
                $outputLines = $outputLines[$startIdx..$endIdx]
                $null = $lines.Add("*(truncated to last 100 lines)*")
                $null = $lines.Add("")
            }
            $null = $lines.Add("${bt}${bt}${bt}text")
            foreach ($ol in $outputLines) { $null = $lines.Add($ol) }
            $null = $lines.Add("${bt}${bt}${bt}")
        }
    }
}

# Write report
$reportContent = $lines -join "`r`n"
[System.IO.File]::WriteAllText($reportFile, $reportContent, [System.Text.UTF8Encoding]::new($false))

# Console summary
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
Write-Host "  Session log: $sessionLog" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor White
Write-Host "  git add .build-results" -ForegroundColor Yellow
Write-Host "  git commit -m 'check: $version'" -ForegroundColor Yellow
Write-Host "  git push" -ForegroundColor Yellow

Stop-Transcript | Out-Null
