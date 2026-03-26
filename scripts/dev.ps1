# Launch API + Studio dev servers with log capture
# Usage: just dev  OR  just dev-quick
# Logs: .build-results/api-dev.log, .build-results/studio-dev.log
# Press Ctrl+C to stop both servers

$ErrorActionPreference = "Continue"

$root = (Get-Location).Path
$resultsDir = Join-Path $root ".build-results"
if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

$apiLog = Join-Path $resultsDir "api-dev.log"
$studioLog = Join-Path $resultsDir "studio-dev.log"

# Clear old logs
"" | Out-File $apiLog -Encoding utf8
"" | Out-File $studioLog -Encoding utf8

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

# Find python
$venvDir = Join-Path $root ".venv"
if (Test-Path (Join-Path $venvDir "Scripts\python.exe")) {
    $python = Join-Path $venvDir "Scripts\python.exe"
} else {
    $python = "python"
}

# Start API server as a separate process with log redirect
$apiProc = Start-Process -FilePath $python `
    -ArgumentList "-m", "uvicorn", "app.main:app", "--reload", "--port", "8000", "--app-dir", (Join-Path $root "api") `
    -RedirectStandardOutput $apiLog `
    -RedirectStandardError (Join-Path $resultsDir "api-dev-err.log") `
    -PassThru -NoNewWindow:$false -WindowStyle Minimized

Write-Host "[API] Started (PID $($apiProc.Id))" -ForegroundColor Green

# Start Studio dev server
$studioDir = Join-Path $root "studio"
$npmCmd = (Get-Command npm -ErrorAction SilentlyContinue).Source
if (-not $npmCmd) { $npmCmd = "npm" }

$studioProc = Start-Process -FilePath $npmCmd `
    -ArgumentList "run", "dev", "--", "--port", "5173" `
    -WorkingDirectory $studioDir `
    -RedirectStandardOutput $studioLog `
    -RedirectStandardError (Join-Path $resultsDir "studio-dev-err.log") `
    -PassThru -NoNewWindow:$false -WindowStyle Minimized

Write-Host "[Studio] Started (PID $($studioProc.Id))" -ForegroundColor Green
Write-Host ""

# Wait a few seconds for servers to start, then tail logs
Start-Sleep -Seconds 3

Write-Host "--- Servers running. Tailing logs (Ctrl+C to stop) ---" -ForegroundColor DarkGray
Write-Host ""

try {
    $apiPos = 0
    $studioPos = 0

    while ($true) {
        # Check if processes are still alive
        if ($apiProc.HasExited) {
            Write-Host "[API] Process exited (code $($apiProc.ExitCode))" -ForegroundColor Red
            # Print error log
            $errLog = Join-Path $resultsDir "api-dev-err.log"
            if (Test-Path $errLog) {
                $errContent = Get-Content $errLog -Raw
                if ($errContent.Trim()) { Write-Host "[API-ERR] $errContent" -ForegroundColor Red }
            }
        }
        if ($studioProc.HasExited) {
            Write-Host "[Studio] Process exited (code $($studioProc.ExitCode))" -ForegroundColor Red
            $errLog = Join-Path $resultsDir "studio-dev-err.log"
            if (Test-Path $errLog) {
                $errContent = Get-Content $errLog -Raw
                if ($errContent.Trim()) { Write-Host "[Studio-ERR] $errContent" -ForegroundColor Red }
            }
        }
        if ($apiProc.HasExited -and $studioProc.HasExited) {
            Write-Host "Both servers stopped." -ForegroundColor Yellow
            break
        }

        # Tail API log
        if (Test-Path $apiLog) {
            $newLines = Get-Content $apiLog -ErrorAction SilentlyContinue | Select-Object -Skip $apiPos
            if ($newLines) {
                $newLines | ForEach-Object { Write-Host "[API] $_" -ForegroundColor Blue }
                $apiPos += ($newLines | Measure-Object).Count
            }
        }

        # Tail Studio log
        if (Test-Path $studioLog) {
            $newLines = Get-Content $studioLog -ErrorAction SilentlyContinue | Select-Object -Skip $studioPos
            if ($newLines) {
                $newLines | ForEach-Object { Write-Host "[Studio] $_" -ForegroundColor Magenta }
                $studioPos += ($newLines | Measure-Object).Count
            }
        }

        Start-Sleep -Milliseconds 1000
    }
}
finally {
    Write-Host ""
    Write-Host "Stopping servers..." -ForegroundColor Yellow

    if (-not $apiProc.HasExited) {
        Stop-Process -Id $apiProc.Id -Force -ErrorAction SilentlyContinue
        Write-Host "[API] Stopped" -ForegroundColor Yellow
    }
    if (-not $studioProc.HasExited) {
        Stop-Process -Id $studioProc.Id -Force -ErrorAction SilentlyContinue
        Write-Host "[Studio] Stopped" -ForegroundColor Yellow
    }

    # Also kill any orphaned uvicorn/node processes on our ports
    $portProcs = Get-NetTCPConnection -LocalPort 8000,5173 -ErrorAction SilentlyContinue | Select-Object -ExpandProperty OwningProcess -Unique
    foreach ($pid in $portProcs) {
        Stop-Process -Id $pid -Force -ErrorAction SilentlyContinue
    }

    Write-Host "Done. Logs saved to $resultsDir" -ForegroundColor Green
}
