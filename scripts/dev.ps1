# Launch API + Studio dev servers with full session logging
# Usage: just dev  OR  just dev-quick
# Logs:
#   .build-results/dev-session.log   -- full console transcript (start to exit)
#   .build-results/api-dev.log       -- API server output
#   .build-results/studio-dev.log    -- Studio dev server output
# Press Ctrl+C to stop both servers

$ErrorActionPreference = "Continue"

$root = (Get-Location).Path
$resultsDir = Join-Path $root ".build-results"
if (-not (Test-Path $resultsDir)) { New-Item -ItemType Directory -Path $resultsDir | Out-Null }

$sessionLog = Join-Path $resultsDir "dev-session.log"
$apiLog = Join-Path $resultsDir "api-dev.log"
$apiErrLog = Join-Path $resultsDir "api-dev-err.log"
$studioLog = Join-Path $resultsDir "studio-dev.log"
$studioErrLog = Join-Path $resultsDir "studio-dev-err.log"

# Start session transcript
try { Stop-Transcript -ErrorAction SilentlyContinue } catch {}
Start-Transcript -Path $sessionLog -Force | Out-Null

# Clear old logs
"" | Out-File $apiLog -Encoding utf8
"" | Out-File $apiErrLog -Encoding utf8
"" | Out-File $studioLog -Encoding utf8
"" | Out-File $studioErrLog -Encoding utf8

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
Write-Host "    Session:  $sessionLog" -ForegroundColor DarkGray
Write-Host "    API out:  $apiLog" -ForegroundColor DarkGray
Write-Host "    API err:  $apiErrLog" -ForegroundColor DarkGray
Write-Host "    Studio:   $studioLog" -ForegroundColor DarkGray
Write-Host "    Studio err: $studioErrLog" -ForegroundColor DarkGray
Write-Host ""
Write-Host "  Press Ctrl+C to stop both servers" -ForegroundColor DarkGray
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Find python executable
$venvDir = Join-Path $root ".venv"
$venvPython = Join-Path $venvDir "Scripts\python.exe"
if (Test-Path $venvPython) {
    $python = $venvPython
} else {
    $python = "python"
}

Write-Host "[API] Using python: $python" -ForegroundColor Blue

# Start API server
$apiProc = Start-Process -FilePath $python `
    -ArgumentList "-m", "uvicorn", "app.main:app", "--reload", "--port", "8000", "--app-dir", (Join-Path $root "api") `
    -RedirectStandardOutput $apiLog `
    -RedirectStandardError $apiErrLog `
    -PassThru -WindowStyle Minimized

Write-Host "[API] Started PID $($apiProc.Id)" -ForegroundColor Green

# Start Studio dev server
$studioDir = Join-Path $root "studio"
$npmPath = (Get-Command npm -ErrorAction SilentlyContinue).Source
if (-not $npmPath) {
    Write-Host "[Studio] ERROR: npm not found in PATH" -ForegroundColor Red
    Stop-Transcript | Out-Null
    exit 1
}

# Use cmd /c to run npm so redirection works properly
$studioProc = Start-Process -FilePath "cmd.exe" `
    -ArgumentList "/c", "cd /d `"$studioDir`" && npm run dev -- --port 5173" `
    -RedirectStandardOutput $studioLog `
    -RedirectStandardError $studioErrLog `
    -PassThru -WindowStyle Minimized

Write-Host "[Studio] Started PID $($studioProc.Id)" -ForegroundColor Green
Write-Host ""

# Give servers a moment to start
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
            $content = Get-Content $apiLog -Encoding utf8 -ErrorAction SilentlyContinue
            if ($content -and $content.Count -gt $apiPos) {
                $newLines = $content[$apiPos..($content.Count - 1)]
                foreach ($ln in $newLines) {
                    if ($ln.Trim()) { Write-Host "[API] $ln" -ForegroundColor Blue }
                }
                $apiPos = $content.Count
            }
        }

        # Tail API stderr
        if (Test-Path $apiErrLog) {
            $content = Get-Content $apiErrLog -Encoding utf8 -ErrorAction SilentlyContinue
            if ($content -and $content.Count -gt $apiErrPos) {
                $newLines = $content[$apiErrPos..($content.Count - 1)]
                foreach ($ln in $newLines) {
                    if ($ln.Trim()) { Write-Host "[API-err] $ln" -ForegroundColor DarkBlue }
                }
                $apiErrPos = $content.Count
            }
        }

        # Tail Studio stdout
        if (Test-Path $studioLog) {
            $content = Get-Content $studioLog -Encoding utf8 -ErrorAction SilentlyContinue
            if ($content -and $content.Count -gt $studioPos) {
                $newLines = $content[$studioPos..($content.Count - 1)]
                foreach ($ln in $newLines) {
                    if ($ln.Trim()) { Write-Host "[Studio] $ln" -ForegroundColor Magenta }
                }
                $studioPos = $content.Count
            }
        }

        # Tail Studio stderr
        if (Test-Path $studioErrLog) {
            $content = Get-Content $studioErrLog -Encoding utf8 -ErrorAction SilentlyContinue
            if ($content -and $content.Count -gt $studioErrPos) {
                $newLines = $content[$studioErrPos..($content.Count - 1)]
                foreach ($ln in $newLines) {
                    if ($ln.Trim()) { Write-Host "[Studio-err] $ln" -ForegroundColor DarkMagenta }
                }
                $studioErrPos = $content.Count
            }
        }

        # Check process health
        if ($apiProc.HasExited -and -not $apiDead) {
            $apiDead = $true
            Write-Host "[API] EXITED with code $($apiProc.ExitCode)" -ForegroundColor Red
        }
        if ($studioProc.HasExited -and -not $studioDead) {
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
            Stop-Process -Id $apiProc.Id -Force -ErrorAction SilentlyContinue
            # Also kill child processes (uvicorn spawns reloader)
            Get-CimInstance Win32_Process -Filter "ParentProcessId=$($apiProc.Id)" -ErrorAction SilentlyContinue |
                ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
        } catch {}
        Write-Host "  [API] Stopped" -ForegroundColor Yellow
    }

    # Kill Studio process tree
    if (-not $studioProc.HasExited) {
        try {
            Stop-Process -Id $studioProc.Id -Force -ErrorAction SilentlyContinue
            Get-CimInstance Win32_Process -Filter "ParentProcessId=$($studioProc.Id)" -ErrorAction SilentlyContinue |
                ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
        } catch {}
        Write-Host "  [Studio] Stopped" -ForegroundColor Yellow
    }

    # Clean up any orphaned processes on our ports
    try {
        Get-NetTCPConnection -LocalPort 8000 -ErrorAction SilentlyContinue |
            Select-Object -ExpandProperty OwningProcess -Unique |
            ForEach-Object { Stop-Process -Id $_ -Force -ErrorAction SilentlyContinue }
        Get-NetTCPConnection -LocalPort 5173 -ErrorAction SilentlyContinue |
            Select-Object -ExpandProperty OwningProcess -Unique |
            ForEach-Object { Stop-Process -Id $_ -Force -ErrorAction SilentlyContinue }
    } catch {}

    Write-Host ""
    Write-Host "  Duration: $($duration.ToString('hh\:mm\:ss'))" -ForegroundColor DarkGray
    Write-Host "  Logs saved to: $resultsDir" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Cyan

    # Stop transcript
    Stop-Transcript | Out-Null
}
