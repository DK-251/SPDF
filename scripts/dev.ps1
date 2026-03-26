# Launch API + Studio dev servers with full session logging
# Usage: just dev  OR  just dev-quick
# Logs saved to .build-results/
# Press Ctrl+C to stop both servers

# --- Transcript FIRST (captures everything from this point) ---
$resultsDir = Join-Path (Get-Location).Path ".build-results"
if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }
$sessionLog = Join-Path $resultsDir "dev-session.log"
try { Stop-Transcript -ErrorAction SilentlyContinue } catch { }
Start-Transcript -Path $sessionLog -Force | Out-Null

$ErrorActionPreference = "Continue"

$root = (Get-Location).Path
$apiLog = Join-Path $resultsDir "api-dev.log"
$apiErrLog = Join-Path $resultsDir "api-dev-err.log"
$studioLog = Join-Path $resultsDir "studio-dev.log"
$studioErrLog = Join-Path $resultsDir "studio-dev-err.log"

# Clear old logs
Set-Content -Path $apiLog -Value "" -Encoding UTF8
Set-Content -Path $apiErrLog -Value "" -Encoding UTF8
Set-Content -Path $studioLog -Value "" -Encoding UTF8
Set-Content -Path $studioErrLog -Value "" -Encoding UTF8

$startTime = Get-Date
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  SPDF Dev Servers" -ForegroundColor Cyan
Write-Host "  Started: $($startTime.ToString('yyyy-MM-dd HH:mm:ss'))" -ForegroundColor DarkGray
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  API:    http://localhost:8000" -ForegroundColor Green
Write-Host "  Studio: http://localhost:5173" -ForegroundColor Green
Write-Host ""
Write-Host "  Logs:" -ForegroundColor Yellow
Write-Host "    Session:    $sessionLog" -ForegroundColor DarkGray
Write-Host "    API out:    $apiLog" -ForegroundColor DarkGray
Write-Host "    API err:    $apiErrLog" -ForegroundColor DarkGray
Write-Host "    Studio out: $studioLog" -ForegroundColor DarkGray
Write-Host "    Studio err: $studioErrLog" -ForegroundColor DarkGray
Write-Host ""
Write-Host "  Press Ctrl+C to stop both servers" -ForegroundColor DarkGray
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Find python
$venvPython = Join-Path $root ".venv\Scripts\python.exe"
$python = "python"
if (Test-Path $venvPython) { $python = $venvPython }
Write-Host "[API] Using python: $python" -ForegroundColor Blue

# Start API server (splatting avoids backtick line continuation)
$apiArgs = @{
    FilePath = $python
    ArgumentList = "-m uvicorn app.main:app --reload --port 8000 --app-dir `"$(Join-Path $root 'api')`""
    RedirectStandardOutput = $apiLog
    RedirectStandardError = $apiErrLog
    PassThru = $true
    NoNewWindow = $true
}
$apiProc = Start-Process @apiArgs
Write-Host "[API] Started PID $($apiProc.Id)" -ForegroundColor Green

# Start Studio dev server via cmd.exe (avoids npm redirection issues)
$studioDir = Join-Path $root "studio"
$studioCmd = "cd /d `"$studioDir`" && npm run dev -- --port 5173"
$studioArgs = @{
    FilePath = "cmd.exe"
    ArgumentList = "/c $studioCmd"
    RedirectStandardOutput = $studioLog
    RedirectStandardError = $studioErrLog
    PassThru = $true
    NoNewWindow = $true
}
$studioProc = Start-Process @studioArgs
Write-Host "[Studio] Started PID $($studioProc.Id)" -ForegroundColor Green
Write-Host ""

Start-Sleep -Seconds 3
Write-Host "--- Tailing logs (Ctrl+C to stop) ---" -ForegroundColor DarkGray
Write-Host ""

$apiPos = 0
$studioPos = 0
$apiErrPos = 0
$studioErrPos = 0
$apiDead = $false
$studioDead = $false

try {
    while ($true) {
        # Tail API stdout
        if (Test-Path $apiLog) {
            $content = @(Get-Content $apiLog -ErrorAction SilentlyContinue)
            if ($content.Count -gt $apiPos) {
                $idx = $apiPos
                while ($idx -lt $content.Count) {
                    $ln = $content[$idx]
                    if ($ln.Trim()) { Write-Host "[API] $ln" -ForegroundColor Blue }
                    $idx++
                }
                $apiPos = $content.Count
            }
        }

        # Tail API stderr
        if (Test-Path $apiErrLog) {
            $content = @(Get-Content $apiErrLog -ErrorAction SilentlyContinue)
            if ($content.Count -gt $apiErrPos) {
                $idx = $apiErrPos
                while ($idx -lt $content.Count) {
                    $ln = $content[$idx]
                    if ($ln.Trim()) { Write-Host "[API-err] $ln" -ForegroundColor DarkBlue }
                    $idx++
                }
                $apiErrPos = $content.Count
            }
        }

        # Tail Studio stdout
        if (Test-Path $studioLog) {
            $content = @(Get-Content $studioLog -ErrorAction SilentlyContinue)
            if ($content.Count -gt $studioPos) {
                $idx = $studioPos
                while ($idx -lt $content.Count) {
                    $ln = $content[$idx]
                    if ($ln.Trim()) { Write-Host "[Studio] $ln" -ForegroundColor Magenta }
                    $idx++
                }
                $studioPos = $content.Count
            }
        }

        # Tail Studio stderr
        if (Test-Path $studioErrLog) {
            $content = @(Get-Content $studioErrLog -ErrorAction SilentlyContinue)
            if ($content.Count -gt $studioErrPos) {
                $idx = $studioErrPos
                while ($idx -lt $content.Count) {
                    $ln = $content[$idx]
                    if ($ln.Trim()) { Write-Host "[Studio-err] $ln" -ForegroundColor DarkMagenta }
                    $idx++
                }
                $studioErrPos = $content.Count
            }
        }

        # Process health
        if ($apiProc.HasExited -and (-not $apiDead)) {
            $apiDead = $true
            Write-Host "[API] EXITED with code $($apiProc.ExitCode)" -ForegroundColor Red
        }
        if ($studioProc.HasExited -and (-not $studioDead)) {
            $studioDead = $true
            Write-Host "[Studio] EXITED with code $($studioProc.ExitCode)" -ForegroundColor Red
        }
        if ($apiDead -and $studioDead) {
            Write-Host ""
            Write-Host "Both servers have exited. Check logs above." -ForegroundColor Yellow
            break
        }

        Start-Sleep -Seconds 1
    }
}
finally {
    $endTime = Get-Date
    $duration = $endTime - $startTime

    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  Stopping servers..." -ForegroundColor Yellow

    # Kill API process tree
    if (-not $apiProc.HasExited) {
        try {
            $apiPid = $apiProc.Id
            Stop-Process -Id $apiPid -Force -ErrorAction SilentlyContinue
            Get-CimInstance Win32_Process -Filter "ParentProcessId=$apiPid" -ErrorAction SilentlyContinue |
                ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
        } catch { }
        Write-Host "  [API] Stopped" -ForegroundColor Yellow
    }

    # Kill Studio process tree
    if (-not $studioProc.HasExited) {
        try {
            $studioPid = $studioProc.Id
            Stop-Process -Id $studioPid -Force -ErrorAction SilentlyContinue
            Get-CimInstance Win32_Process -Filter "ParentProcessId=$studioPid" -ErrorAction SilentlyContinue |
                ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
        } catch { }
        Write-Host "  [Studio] Stopped" -ForegroundColor Yellow
    }

    # Clean up orphaned processes on our ports
    try {
        Get-NetTCPConnection -LocalPort 8000 -ErrorAction SilentlyContinue |
            Select-Object -ExpandProperty OwningProcess -Unique |
            ForEach-Object { Stop-Process -Id $_ -Force -ErrorAction SilentlyContinue }
        Get-NetTCPConnection -LocalPort 5173 -ErrorAction SilentlyContinue |
            Select-Object -ExpandProperty OwningProcess -Unique |
            ForEach-Object { Stop-Process -Id $_ -Force -ErrorAction SilentlyContinue }
    } catch { }

    $durationStr = "{0:D2}:{1:D2}:{2:D2}" -f $duration.Hours, $duration.Minutes, $duration.Seconds
    Write-Host ""
    Write-Host "  Duration: $durationStr" -ForegroundColor DarkGray
    Write-Host "  Logs saved to: $resultsDir" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Cyan

    Stop-Transcript | Out-Null
}
