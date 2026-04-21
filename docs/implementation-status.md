# Implementation Status (Phase 1)

## Completed

- Created repository scaffold under `unified-shell-platform`.
- Added Rust CLI command set:
  - doctor
  - plan
  - apply
  - sync
  - rollback
  - integrate
- Added canonical config model (`config/platform.toml`) with Phase 1 tool and agent targets.
- Added profile overlays for OS/shell/agent context-aware variable resolution.
- Added config loader wiring in CLI commands.
- Added render outputs for VS Code, Claude Code, and Kiro into `generated/agents`.
- Added resolved environment output into `generated/env/resolved.env`.
- Added managed file mapping model and canonical config entries.
- Added generated chezmoi manifest output in `generated/chezmoi/managed-files.toml`.
- Added generated chezmoi source-state tree under `generated/chezmoi/source-state`.
- Added canonical Atuin settings model in `config/platform.toml`.
- Added generated Atuin config output in `generated/atuin/config.toml`.
- Added Atuin bootstrap helper script `scripts/atuin-bootstrap.sh` with `bootstrap`, `login`, `register`, `import`, `sync`, and `setup` modes.
- Added `just` tasks for Atuin bootstrap/login/sync/setup.
- Added non-interactive Atuin credential flow support via ATUIN_USERNAME, ATUIN_PASSWORD, ATUIN_KEY, and ATUIN_EMAIL.
- Wired CLI integrate/apply flows to invoke `chezmoi diff/apply` when available (with safe skip when unavailable).
- Added deploy-state persistence at `generated/state/deploy-state.json` during apply.
- Added rollback backup snapshots under `generated/rollback-backups` and restore logic for managed targets.
- Implemented rollback cleanup for generated artifacts and deploy-state metadata.
- Added rollback preview mode via `cargo run -- rollback --preview` and `just rollback-preview`.
- Seeded initial managed templates:
  - `templates/bash/dot_bashrc.tmpl`
  - `templates/nushell/config.nu`
  - `templates/starship/starship.toml`
- Added JS workspace scaffolding (`skills`, `agents`) and bun/npm install flows.
- Added helper scripts:
  - `scripts/bootstrap.sh`
  - `scripts/install-skills-agents.sh`
- Validated command flow:
  - `cargo run -- doctor`
  - `cargo run -- plan`
  - `cargo run -- apply`

## Current blocker

- No compile blocker remains for the Rust scaffold on this host.
- Runtime dependency tools for full integration are not yet installed (`chezmoi`, `atuin`, `just`, `direnv`, `bun`, `npm`).

## Next implementation items

1. Expand renderer payloads from placeholders to full target-specific schemas.
2. Add more deterministic profile precedence tests for host and interactive overrides.
3. Wire bun/npm lockfile and reproducibility checks into CI tasks.
4. Add additional safety checks before live rollback on hosts with active sessions.
5. Add credential source hardening for Atuin auth automation.
