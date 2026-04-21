#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GENERATED_CONFIG="$ROOT_DIR/generated/atuin/config.toml"
TARGET_DIR="${HOME}/.config/atuin"
TARGET_CONFIG="${TARGET_DIR}/config.toml"

if ! command -v atuin >/dev/null 2>&1; then
  printf "[atuin-bootstrap] error: atuin is not installed\n" >&2
  exit 1
fi

if [[ ! -f "$GENERATED_CONFIG" ]]; then
  printf "[atuin-bootstrap] error: %s not found; run 'cargo run -- apply' first\n" "$GENERATED_CONFIG" >&2
  exit 2
fi

mkdir -p "$TARGET_DIR"
cp "$GENERATED_CONFIG" "$TARGET_CONFIG"
printf "[atuin-bootstrap] wrote %s\n" "$TARGET_CONFIG"

if [[ "${1:-}" == "import" ]]; then
  if [[ -f "${HOME}/.bash_history" ]]; then
    atuin import auto
    printf "[atuin-bootstrap] imported local history via 'atuin import auto'\n"
  else
    printf "[atuin-bootstrap] skipped import: no compatible history file found\n"
  fi
fi

if [[ "${2:-}" == "sync" || "${1:-}" == "sync" ]]; then
  atuin sync || true
  printf "[atuin-bootstrap] attempted sync (non-fatal)\n"
fi

printf "[atuin-bootstrap] complete\n"