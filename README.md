# Unified Shell Platform

Phase 1 scaffold for a portable dotfiles and devops platform.

## Goals

- Shared configs across bash, nushell, and zsh
- Multi-OS support for macOS, Linux, WSL, and Windows
- Centralized agent and tool configs for VS Code, Claude Code, and Kiro
- Shared shell history via Atuin
- Adoption-first tooling with compiled binaries preferred
- bun and npm support for skills and agent installation workflows

## Phase 1 Stack

- chezmoi for dotfile state and apply/diff
- atuin for shared shell history
- just for task entrypoints
- direnv for per-project environment loading
- Rust orchestrator CLI for plan/apply/sync/rollback/integrate
- bun and npm for JS-based skill and agent package installs

## Current Status

This repo currently contains the first scaffold:

- Rust CLI entrypoints
- Canonical config model
- Initial README and docs structure

## Next Steps

1. Add config rendering for shells and agent IDEs.
2. Add chezmoi/atuin/just integration.
3. Add OS and shell variable resolution.
4. Add bun/npm install flows for skills and agents.
