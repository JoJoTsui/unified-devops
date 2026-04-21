#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

printf "[bootstrap] root: %s\n" "$ROOT_DIR"

if command -v chezmoi >/dev/null 2>&1; then
  printf "[bootstrap] chezmoi found\n"
else
  printf "[bootstrap] chezmoi missing (install required)\n"
fi

if command -v atuin >/dev/null 2>&1; then
  printf "[bootstrap] atuin found\n"
else
  printf "[bootstrap] atuin missing (install required)\n"
fi

if command -v just >/dev/null 2>&1; then
  printf "[bootstrap] just found\n"
else
  printf "[bootstrap] just missing (install required)\n"
fi

if command -v direnv >/dev/null 2>&1; then
  printf "[bootstrap] direnv found\n"
else
  printf "[bootstrap] direnv missing (install required)\n"
fi

if command -v bun >/dev/null 2>&1; then
  printf "[bootstrap] bun found\n"
else
  printf "[bootstrap] bun missing (optional but preferred for JS installs)\n"
fi

if command -v npm >/dev/null 2>&1; then
  printf "[bootstrap] npm found\n"
else
  printf "[bootstrap] npm missing (fallback JS install path unavailable)\n"
fi

if command -v cc >/dev/null 2>&1; then
  printf "[bootstrap] C linker found\n"
else
  printf "[bootstrap] C linker missing; install build-essential/clang to compile Rust crates\n"
fi
