#!/bin/bash

set -e

# Bump minor version in package.json (without git tag)
echo "Bumping minor version..."
npm version minor --no-git-tag-version

# Get the new version from package.json
NEW_VERSION=$(node -p "require('./package.json').version")

echo "New version: $NEW_VERSION"

# Update Cargo.toml to match
echo "Updating Cargo.toml to match package.json version..."
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

echo "Version bump complete! Both package.json and Cargo.toml are now at version $NEW_VERSION"
