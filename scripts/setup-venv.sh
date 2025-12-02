#!/usr/bin/env bash
# Convenience script to create and populate a per-repo virtualenv
set -euo pipefail

VENV_DIR=".venv"

if [ -d "$VENV_DIR" ]; then
  echo "Using existing venv at $VENV_DIR"
else
  echo "Creating venv at $VENV_DIR"
  python3 -m venv "$VENV_DIR"
fi

# Activate the venv for the rest of this script
# shellcheck source=/dev/null
source "$VENV_DIR/bin/activate"

python -m pip install --upgrade pip setuptools wheel

# Install advent-of-code-data by default (safe and small dependency)
pip install advent-of-code-data python-dotenv

echo "Setup complete. Activate with: source $VENV_DIR/bin/activate"
