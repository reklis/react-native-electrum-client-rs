#!/bin/bash

set -e

# Default to minor bump if no argument provided
BUMP_TYPE=${1:-minor}

# Validate bump type
if [[ ! "$BUMP_TYPE" =~ ^(major|minor|patch)$ ]]; then
  echo "Error: Invalid bump type '$BUMP_TYPE'. Must be major, minor, or patch."
  exit 1
fi

# Bump version in package.json (without git tag)
echo "Bumping $BUMP_TYPE version..."
npm version $BUMP_TYPE --no-git-tag-version

# Get the new version from package.json
NEW_VERSION=$(node -p "require('./package.json').version")

echo "New version: $NEW_VERSION"

# Update Cargo.toml to match
echo "Updating Cargo.toml to match package.json version..."
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
else
  # Linux
  sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
fi

# Stage the version changes
echo "Staging version changes..."
git add package.json package-lock.json Cargo.toml

# Commit the version bump
echo "Committing version bump..."
git commit -m "Bump version to $NEW_VERSION"

# Create git tag with v prefix (required for workflow trigger)
echo "Creating git tag v$NEW_VERSION..."
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

# Push commit and tag
echo "Pushing to remote..."
git push origin HEAD
git push origin "v$NEW_VERSION"

echo ""
echo "✓ Version bump complete!"
echo "✓ Version: $NEW_VERSION"
echo "✓ Tag: v$NEW_VERSION"
echo "✓ Changes pushed to remote"
echo ""
echo "The release workflow should now trigger automatically."
