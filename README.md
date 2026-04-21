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

This repo currently contains a runnable Phase 1 baseline:

- Rust CLI entrypoints
- Canonical config model
- Context-aware profile resolution and env merging
- Generated agent artifacts under `generated/agents`
- Generated environment artifact under `generated/env/resolved.env`
- Generated Atuin config under `generated/atuin/config.toml`
- Generated chezmoi manifest under `generated/chezmoi/managed-files.toml`
- Generated chezmoi source-state tree under `generated/chezmoi/source-state`
- Initial managed template payloads for bash, nushell, and starship
- Atuin bootstrap helper script and task entrypoint
- CLI wiring for `chezmoi diff/apply` execution when chezmoi is installed
- Deploy-state tracking and rollback restore flow for managed targets
- Rollback preview mode via `cargo run -- rollback --preview`

## Next Steps

1. Expand renderer payloads to target-specific schemas.
2. Add lockfile/reproducibility checks for skills and agents.
3. Add Atuin login/key bootstrap flow for first-time hosts.
4. Add additional rollback safety checks for active sessions.
