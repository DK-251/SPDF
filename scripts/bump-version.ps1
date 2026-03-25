# Bump the snapshot version number (Windows)
# Usage:
#   powershell -File scripts/bump-version.ps1              # snapshot: 0.1.0-snapshot.1 -> 0.1.0-snapshot.2
#   powershell -File scripts/bump-version.ps1 patch        # patch:    0.1.0 -> 0.1.1-snapshot.1
#   powershell -File scripts/bump-version.ps1 minor        # minor:    0.1.0 -> 0.2.0-snapshot.1
#   powershell -File scripts/bump-version.ps1 major        # major:    0.1.0 -> 1.0.0-snapshot.1

param([string]$BumpType = "snapshot")

$current = (Get-Content "VERSION").Trim()

if ($current -match "^(\d+)\.(\d+)\.(\d+)(-snapshot\.(\d+))?$") {
    $major = [int]$Matches[1]
    $minor = [int]$Matches[2]
    $patch = [int]$Matches[3]
    $snap  = if ($Matches[5]) { [int]$Matches[5] } else { 0 }
} else {
    Write-Host "Cannot parse VERSION: $current" -ForegroundColor Red
    exit 1
}

switch ($BumpType) {
    "snapshot" {
        $snap++
        $newVersion = "$major.$minor.$patch-snapshot.$snap"
    }
    "patch" {
        $patch++
        $newVersion = "$major.$minor.$patch-snapshot.1"
    }
    "minor" {
        $minor++
        $patch = 0
        $newVersion = "$major.$minor.$patch-snapshot.1"
    }
    "major" {
        $major++
        $minor = 0
        $patch = 0
        $newVersion = "$major.$minor.$patch-snapshot.1"
    }
    default {
        Write-Host "Usage: bump-version.ps1 [snapshot|patch|minor|major]"
        exit 1
    }
}

$newVersion | Out-File -FilePath "VERSION" -Encoding utf8 -NoNewline
Write-Host "$current -> $newVersion"
