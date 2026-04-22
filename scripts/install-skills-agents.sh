#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

MODE="${1:-auto}"

install_with_bun() {
  printf "[install] using bun\n"
  bun install --cwd "$ROOT_DIR"
}

install_with_npm() {
  printf "[install] using npm\n"
  npm install --prefix "$ROOT_DIR"
}

install_with_npm_ci() {
  printf "[install] using npm ci\n"
  npm ci --ignore-scripts --prefix "$ROOT_DIR"
}

install_with_bun_frozen() {
  printf "[install] using bun frozen lockfile\n"
  bun install --frozen-lockfile --cwd "$ROOT_DIR"
}

case "$MODE" in
  bun)
    install_with_bun
    ;;
  npm)
    install_with_npm
    ;;
  npm-ci)
    install_with_npm_ci
    ;;
  bun-frozen)
    install_with_bun_frozen
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
    printf "usage: %s [auto|bun|npm|npm-ci|bun-frozen]\n" "$0" >&2
    exit 2
    ;;
esac
