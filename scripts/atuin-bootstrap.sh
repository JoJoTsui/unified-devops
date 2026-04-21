#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GENERATED_CONFIG="$ROOT_DIR/generated/atuin/config.toml"
TARGET_DIR="${HOME}/.config/atuin"
TARGET_CONFIG="${TARGET_DIR}/config.toml"
ATUIN_KEY_PATH="${HOME}/.local/share/atuin/key"

MODE="${1:-bootstrap}"

ATUIN_USERNAME="${ATUIN_USERNAME:-}"
ATUIN_PASSWORD="${ATUIN_PASSWORD:-}"
ATUIN_EMAIL="${ATUIN_EMAIL:-}"
ATUIN_KEY="${ATUIN_KEY:-}"

require_generated_config() {
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

has_atuin() {
  command -v atuin >/dev/null 2>&1
}

ensure_atuin_or_skip() {
  if ! has_atuin; then
    printf "[atuin-bootstrap] atuin not installed; skipping %s flow\n" "$1"
    return 1
  fi
  return 0
}

key_exists() {
  [[ -f "$ATUIN_KEY_PATH" ]]
}

login_with_existing_account() {
  if ! ensure_atuin_or_skip "login"; then
    return 0
  fi

  if key_exists; then
    printf "[atuin-bootstrap] key already exists at %s; skipping login\n" "$ATUIN_KEY_PATH"
    return 0
  fi

  if [[ -n "$ATUIN_USERNAME" && -n "$ATUIN_PASSWORD" && -n "$ATUIN_KEY" ]]; then
    printf "[atuin-bootstrap] attempting non-interactive login with provided credentials\n"
    atuin login -u "$ATUIN_USERNAME" -p "$ATUIN_PASSWORD" -k "$ATUIN_KEY"
  else
    printf "[atuin-bootstrap] login skipped: missing one of ATUIN_USERNAME/ATUIN_PASSWORD/ATUIN_KEY\n"
    printf "[atuin-bootstrap] hint: export ATUIN_USERNAME, ATUIN_PASSWORD, and ATUIN_KEY, then run './scripts/atuin-bootstrap.sh login'\n"
    return 0
  fi

  if key_exists; then
    printf "[atuin-bootstrap] login completed and key detected at %s\n" "$ATUIN_KEY_PATH"
  else
    printf "[atuin-bootstrap] warning: login finished but key file is still missing\n" >&2
  fi
}

register_first_account() {
  if ! ensure_atuin_or_skip "register"; then
    return 0
  fi

  if [[ -n "$ATUIN_USERNAME" && -n "$ATUIN_PASSWORD" && -n "$ATUIN_EMAIL" ]]; then
    printf "[atuin-bootstrap] attempting non-interactive register\n"
    atuin register -u "$ATUIN_USERNAME" -p "$ATUIN_PASSWORD" -e "$ATUIN_EMAIL"
    printf "[atuin-bootstrap] register command completed\n"
  else
    printf "[atuin-bootstrap] register skipped: missing one of ATUIN_USERNAME/ATUIN_PASSWORD/ATUIN_EMAIL\n"
    printf "[atuin-bootstrap] hint: export ATUIN_USERNAME, ATUIN_PASSWORD, and ATUIN_EMAIL, then run './scripts/atuin-bootstrap.sh register'\n"
  fi
}

import_history() {
  if ! ensure_atuin_or_skip "import"; then
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
  if ! ensure_atuin_or_skip "sync"; then
    return 0
  fi

  atuin sync || true
  printf "[atuin-bootstrap] attempted sync (non-fatal)\n"
}

setup_all() {
  write_config

  if ! key_exists; then
    if [[ -n "$ATUIN_USERNAME" && -n "$ATUIN_PASSWORD" && -n "$ATUIN_KEY" ]]; then
      login_with_existing_account
    elif [[ -n "$ATUIN_USERNAME" && -n "$ATUIN_PASSWORD" && -n "$ATUIN_EMAIL" ]]; then
      register_first_account
      if key_exists; then
        printf "[atuin-bootstrap] key material detected after register\n"
      else
        printf "[atuin-bootstrap] register completed; run 'atuin key' to verify local key material\n"
      fi
    else
      printf "[atuin-bootstrap] no key file found and no non-interactive credentials provided\n"
      printf "[atuin-bootstrap] to login existing account: set ATUIN_USERNAME, ATUIN_PASSWORD, ATUIN_KEY\n"
      printf "[atuin-bootstrap] to register first account: set ATUIN_USERNAME, ATUIN_PASSWORD, ATUIN_EMAIL\n"
    fi
  else
    printf "[atuin-bootstrap] key already present; skipping auth bootstrap\n"
  fi

  import_history
  sync_history
}

case "$MODE" in
  bootstrap)
    require_generated_config
    write_config
    ;;
  login)
    require_generated_config
    write_config
    login_with_existing_account
    ;;
  register)
    require_generated_config
    write_config
    register_first_account
    ;;
  import)
    require_generated_config
    write_config
    import_history
    ;;
  sync)
    require_generated_config
    write_config
    sync_history
    ;;
  setup)
    require_generated_config
    setup_all
    ;;
  *)
    printf "usage: %s [bootstrap|login|register|import|sync|setup]\n" "$0" >&2
    exit 2
    ;;
esac

printf "[atuin-bootstrap] complete\n"