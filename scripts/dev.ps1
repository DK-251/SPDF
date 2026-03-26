# Launch API + Studio dev servers with log capture
# Usage: just dev
# Logs: .build-results/api-dev.log, .build-results/studio-dev.log
# Press Ctrl+C to stop both servers

$ErrorActionPreference = "Continue"

$resultsDir = ".build-results"
if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

$apiLog = Join-Path $resultsDir "api-dev.log"
$studioLog = Join-Path $resultsDir "studio-dev.log"

# Clear old logs
"" | Set-Content $apiLog
"" | Set-Content $studioLog

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  SPDF Dev Servers" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  API:    http://localhost:8000" -ForegroundColor Green
Write-Host "  Studio: http://localhost:5173" -ForegroundColor Green
Write-Host ""
Write-Host "  Logs:" -ForegroundColor Yellow
Write-Host "    $apiLog" -ForegroundColor Yellow
Write-Host "    $studioLog" -ForegroundColor Yellow
Write-Host ""
Write-Host "  Press Ctrl+C to stop both servers" -ForegroundColor DarkGray
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Activate virtualenv for API
$venvDir = ".venv"
if (Test-Path "$venvDir\Scripts\Activate.ps1") {
    & "$venvDir\Scripts\Activate.ps1"
}

# Start API server as background job
$apiJob = Start-Job -ScriptBlock {
    param($root, $logFile, $venvDir)
    Set-Location $root
    if (Test-Path "$venvDir\Scripts\python.exe") {
        $python = Join-Path $venvDir "Scripts\python.exe"
    } else {
        $python = "python"
    }
    & $python -m uvicorn app.main:app --reload --port 8000 --app-dir api 2>&1 |
        Tee-Object -FilePath $logFile
} -ArgumentList (Get-Location).Path, $apiLog, $venvDir

Write-Host "[API] Started (job $($apiJob.Id))" -ForegroundColor Green

# Start Studio dev server as background job
$studioJob = Start-Job -ScriptBlock {
    param($root, $logFile)
    Set-Location (Join-Path $root "studio")
    npx vite --port 5173 2>&1 |
        Tee-Object -FilePath $logFile
} -ArgumentList (Get-Location).Path, $studioLog

Write-Host "[Studio] Started (job $($studioJob.Id))" -ForegroundColor Green
Write-Host ""

# Stream output from both jobs until Ctrl+C
try {
    while ($true) {
        # Print any new output from API
        $apiOutput = Receive-Job $apiJob -ErrorAction SilentlyContinue
        if ($apiOutput) {
            $apiOutput | ForEach-Object { Write-Host "[API] $_" -ForegroundColor Blue }
        }

        # Print any new output from Studio
        $studioOutput = Receive-Job $studioJob -ErrorAction SilentlyContinue
        if ($studioOutput) {
            $studioOutput | ForEach-Object { Write-Host "[Studio] $_" -ForegroundColor Magenta }
        }

        # Check if either job died
        if ($apiJob.State -eq "Failed") {
            Write-Host "[API] CRASHED — check $apiLog" -ForegroundColor Red
        }
        if ($studioJob.State -eq "Failed") {
            Write-Host "[Studio] CRASHED — check $studioLog" -ForegroundColor Red
        }
        if ($apiJob.State -eq "Failed" -and $studioJob.State -eq "Failed") {
            break
        }

        Start-Sleep -Milliseconds 500
    }
}
finally {
    Write-Host ""
    Write-Host "Stopping servers..." -ForegroundColor Yellow
    Stop-Job $apiJob -ErrorAction SilentlyContinue
    Stop-Job $studioJob -ErrorAction SilentlyContinue
    Remove-Job $apiJob -Force -ErrorAction SilentlyContinue
    Remove-Job $studioJob -Force -ErrorAction SilentlyContinue
    Write-Host "Done. Logs saved to $resultsDir" -ForegroundColor Green
}
