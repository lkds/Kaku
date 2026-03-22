#!/bin/bash
# Release script for Kaku Windows

set -e

VERSION="${1:-0.8.0}"
TAG="v${VERSION}"

echo "🦀 Kaku Windows Release Script"
echo "================================"
echo "Version: ${VERSION}"
echo "Tag: ${TAG}"
echo ""

# Check if tag exists
if git tag -l | grep -q "^${TAG}$"; then
    echo "⚠️  Tag ${TAG} already exists"
    read -p "Delete and recreate? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git tag -d "${TAG}"
        git push origin --delete "${TAG}" 2>/dev/null || true
    else
        echo "Aborted"
        exit 1
    fi
fi

# Update version in Cargo.toml
echo "📝 Updating version in Cargo.toml files..."
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" kaku/Cargo.toml
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" kaku-gui/Cargo.toml

# Update version in installer scripts
sed -i "s/!define APP_VERSION \".*\"/!define APP_VERSION \"${VERSION}\"/" kaku.nsi
sed -i "s/Version=\".*\"/Version=\"${VERSION}\"/" wix/kaku.wxs

# Commit version bump
git add -A
git commit -m "chore: bump version to ${VERSION}" || true

# Create tag
echo "🏷️  Creating tag ${TAG}..."
git tag -a "${TAG}" -m "Release ${TAG}"

# Push
echo "🚀 Pushing to origin..."
git push origin main
git push origin "${TAG}"

echo ""
echo "✅ Done! GitHub Actions will build and create a release."
echo "   Check: https://github.com/lkds/Kaku/actions"
echo ""
echo "📦 Artifacts will be available at:"
echo "   https://github.com/lkds/Kaku/releases/tag/${TAG}"