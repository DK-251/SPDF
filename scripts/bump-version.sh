#!/usr/bin/env bash
# Bump the snapshot version number.
# Usage:
#   bash scripts/bump-version.sh              # increments snapshot: 0.1.0-snapshot.1 -> 0.1.0-snapshot.2
#   bash scripts/bump-version.sh patch        # increments patch:    0.1.0 -> 0.1.1-snapshot.1
#   bash scripts/bump-version.sh minor        # increments minor:    0.1.0 -> 0.2.0-snapshot.1
#   bash scripts/bump-version.sh major        # increments major:    0.1.0 -> 1.0.0-snapshot.1

set -euo pipefail

VERSION_FILE="VERSION"
CURRENT=$(cat "$VERSION_FILE" | tr -d '[:space:]')

# Parse: MAJOR.MINOR.PATCH-snapshot.N
BASE=$(echo "$CURRENT" | sed 's/-snapshot\..*//')
MAJOR=$(echo "$BASE" | cut -d. -f1)
MINOR=$(echo "$BASE" | cut -d. -f2)
PATCH=$(echo "$BASE" | cut -d. -f3)
SNAP=$(echo "$CURRENT" | grep -oP 'snapshot\.\K\d+' || echo "0")

BUMP_TYPE="${1:-snapshot}"

case "$BUMP_TYPE" in
    snapshot)
        SNAP=$((SNAP + 1))
        NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}-snapshot.${SNAP}"
        ;;
    patch)
        PATCH=$((PATCH + 1))
        NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}-snapshot.1"
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}-snapshot.1"
        ;;
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}-snapshot.1"
        ;;
    *)
        echo "Usage: $0 [snapshot|patch|minor|major]"
        exit 1
        ;;
esac

echo "$NEW_VERSION" > "$VERSION_FILE"
echo "$CURRENT -> $NEW_VERSION"
