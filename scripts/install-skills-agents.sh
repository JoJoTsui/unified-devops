#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

MODE="${1:-auto}"

install_with_bun() {
  printf "[install] using bun\n"
  bun install --cwd "$ROOT_DIR/skills"
  bun install --cwd "$ROOT_DIR/agents"
}

install_with_npm() {
  printf "[install] using npm\n"
  npm install --prefix "$ROOT_DIR/skills"
  npm install --prefix "$ROOT_DIR/agents"
}

case "$MODE" in
  bun)
    install_with_bun
    ;;
  npm)
    install_with_npm
    ;;
  auto)
    if command -v bun >/dev/null 2>&1; then
      install_with_bun
    elif command -v npm >/dev/null 2>&1; then
      install_with_npm
    else
      printf "[install] error: neither bun nor npm found\n" >&2
      exit 1
    fi
    ;;
  *)
    printf "usage: %s [auto|bun|npm]\n" "$0" >&2
    exit 2
    ;;
esac
