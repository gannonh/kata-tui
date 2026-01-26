#!/bin/sh
# Install git hooks for development

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
HOOK_DIR="$(git rev-parse --git-dir)/hooks"

cp "$SCRIPT_DIR/pre-commit" "$HOOK_DIR/pre-commit"
chmod +x "$HOOK_DIR/pre-commit"

echo "Git hooks installed."
