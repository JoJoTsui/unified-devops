#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GENERATED_CONFIG="$ROOT_DIR/generated/atuin/config.toml"
TARGET_DIR="${HOME}/.config/atuin"
TARGET_CONFIG="${TARGET_DIR}/config.toml"
ATUIN_KEY_PATH="${HOME}/.local/share/atuin/key"
MODE="${1:-bootstrap}"

require_atuin() {
  if ! command -v atuin >/dev/null 2>&1; then
    printf "[atuin-bootstrap] error: atuin is not installed\n" >&2
    return 1
  fi
}

ensure_generated_config() {
  if [[ ! -f "$GENERATED_CONFIG" ]]; then
    printf "[atuin-bootstrap] error: %s not found; run 'cargo run -- apply' first\n" "$GENERATED_CONFIG" >&2
    exit 2
  fi
}

write_config() {
  mkdir -p "$TARGET_DIR"
  cp "$GENERATED_CONFIG" "$TARGET_CONFIG"
  printf "[atuin-bootstrap] wrote %s\n" "$TARGET_CONFIG"
}

login_if_needed() {
  if ! command -v atuin >/dev/null 2>&1; then
    printf "[atuin-bootstrap] atuin not installed; skipping login flow\n"
    return 0
  fi

  if [[ -f "$ATUIN_KEY_PATH" ]]; then
    printf "[atuin-bootstrap] key already exists at %s; skipping login\n" "$ATUIN_KEY_PATH"
    return 0
  fi

  printf "[atuin-bootstrap] no key found; starting atuin login flow\n"
  atuin login

  if [[ -f "$ATUIN_KEY_PATH" ]]; then
    printf "[atuin-bootstrap] login completed and key material detected at %s\n" "$ATUIN_KEY_PATH"
  else
    printf "[atuin-bootstrap] login completed but key file is still missing; check atuin account setup\n" >&2
  fi
}

import_history() {
  if ! command -v atuin >/dev/null 2>&1; then
    printf "[atuin-bootstrap] atuin not installed; skipping import\n"
    return 0
  fi

  if [[ -f "${HOME}/.bash_history" ]]; then
    atuin import auto
    printf "[atuin-bootstrap] imported local history via 'atuin import auto'\n"
  else
    printf "[atuin-bootstrap] skipped import: no compatible history file found\n"
  fi
}

sync_history() {
  if ! command -v atuin >/dev/null 2>&1; then
    printf "[atuin-bootstrap] atuin not installed; skipping sync\n"
    return 0
  fi
  atuin sync || true
  printf "[atuin-bootstrap] attempted sync (non-fatal)\n"
}

setup_all() {
  write_config
  login_if_needed
  import_history
  sync_history
}

case "$MODE" in
  bootstrap)
    ensure_generated_config
    write_config
    ;;
  login)
    ensure_generated_config
    write_config
    login_if_needed
    ;;
  import)
    ensure_generated_config
    write_config
    import_history
    ;;
  sync)
    ensure_generated_config
    write_config
    sync_history
    ;;
  setup)
    ensure_generated_config
    setup_all
    ;;
  *)
    printf "usage: %s [bootstrap|login|import|sync|setup]\n" "$0" >&2
    exit 2
    ;;
esac

printf "[atuin-bootstrap] complete\n"