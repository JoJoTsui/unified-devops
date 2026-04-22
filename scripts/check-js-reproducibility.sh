#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKSPACES=("skills" "agents")
MODE="${1:-lockfiles}"

workspace_path() {
  local name="$1"
  printf "%s/%s" "$ROOT_DIR" "$name"
}

lockfile_for_workspace() {
  local ws="$1"
  local ws_dir
  ws_dir="$(workspace_path "$ws")"

  if [[ -f "$ws_dir/package-lock.json" ]]; then
    printf "%s" "$ws_dir/package-lock.json"
    return 0
  fi

  if [[ -f "$ws_dir/npm-shrinkwrap.json" ]]; then
    printf "%s" "$ws_dir/npm-shrinkwrap.json"
    return 0
  fi

  if [[ -f "$ws_dir/bun.lockb" ]]; then
    printf "%s" "$ws_dir/bun.lockb"
    return 0
  fi

  if [[ -f "$ws_dir/bun.lock" ]]; then
    printf "%s" "$ws_dir/bun.lock"
    return 0
  fi

  return 1
}

check_lockfiles() {
  local missing=0

  for ws in "${WORKSPACES[@]}"; do
    if lockfile="$(lockfile_for_workspace "$ws")"; then
      printf "[repro] lockfile ok: %s -> %s\n" "$ws" "$lockfile"
    else
      printf "[repro] missing lockfile for workspace: %s\n" "$ws" >&2
      missing=1
    fi
  done

  if [[ "$missing" -ne 0 ]]; then
    printf "[repro] create lockfiles before CI install:\n" >&2
    printf "[repro]   npm: npm install --package-lock-only --prefix <workspace>\n" >&2
    printf "[repro]   bun: bun install --cwd <workspace>\n" >&2
    return 1
  fi

  printf "[repro] lockfile check passed\n"
}

npm_ci_install() {
  command -v npm >/dev/null 2>&1 || {
    printf "[repro] npm not found\n" >&2
    return 1
  }

  check_lockfiles

  for ws in "${WORKSPACES[@]}"; do
    printf "[repro] npm ci --prefix %s\n" "$ws"
    npm ci --ignore-scripts --prefix "$(workspace_path "$ws")"
  done
}

bun_frozen_install() {
  command -v bun >/dev/null 2>&1 || {
    printf "[repro] bun not found\n" >&2
    return 1
  }

  check_lockfiles

  for ws in "${WORKSPACES[@]}"; do
    printf "[repro] bun install --frozen-lockfile --cwd %s\n" "$ws"
    bun install --frozen-lockfile --cwd "$(workspace_path "$ws")"
  done
}

case "$MODE" in
  lockfiles)
    check_lockfiles
    ;;
  npm-ci)
    npm_ci_install
    ;;
  bun-frozen)
    bun_frozen_install
    ;;
  auto)
    check_lockfiles
    if command -v bun >/dev/null 2>&1; then
      bun_frozen_install
    elif command -v npm >/dev/null 2>&1; then
      npm_ci_install
    else
      printf "[repro] neither bun nor npm found\n" >&2
      exit 1
    fi
    ;;
  *)
    printf "usage: %s [lockfiles|npm-ci|bun-frozen|auto]\n" "$0" >&2
    exit 2
    ;;
esac
