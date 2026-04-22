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
- Expanded renderer payloads to richer target-native schemas for VS Code, Claude Code, and Kiro.
- Added resolved environment output into `generated/env/resolved.env`.
- Added managed file mapping model and canonical config entries.
- Added generated chezmoi manifest output in `generated/chezmoi/managed-files.toml`.
- Added generated chezmoi source-state tree under `generated/chezmoi/source-state`.
- Added canonical Atuin settings model in `config/platform.toml`.
- Added generated Atuin config output in `generated/atuin/config.toml`.
- Added Atuin bootstrap helper script `scripts/atuin-bootstrap.sh` with `bootstrap`, `login`, `register`, `import`, `sync`, and `setup` modes.
- Added `just` tasks for Atuin bootstrap/login/sync/setup.
- Added non-interactive Atuin credential flow support via ATUIN_USERNAME, ATUIN_PASSWORD, ATUIN_KEY, and ATUIN_EMAIL.
- Added Atuin credential source hardening with secure `ATUIN_CREDENTIALS_FILE`, `ATUIN_PASSWORD_FILE`, and `ATUIN_KEY_FILE` inputs plus strict permissions checks.
- Wired CLI integrate/apply flows to invoke `chezmoi diff/apply` when available (with safe skip when unavailable).
- Added deploy-state persistence at `generated/state/deploy-state.json` during apply.
- Added rollback backup snapshots under `generated/rollback-backups` and restore logic for managed targets.
- Implemented rollback cleanup for generated artifacts and deploy-state metadata.
- Added rollback preview mode via `cargo run -- rollback --preview` and `just rollback-preview`.
- Added rollback safety gates: explicit `--force`, active-session detection, and `--allow-active-sessions` override.
- Added rollback external guard extension for sentinel/lock files with `--allow-external-locks` override and preview reporting.
- Added resolver edge-case tests for host/interactive matching and deterministic env precedence.
- Added resolver tie-breaker policy ordering: less-specific profiles apply before more-specific profiles, with declaration order preserved for equal specificity.
- Seeded initial managed templates:
  - `templates/bash/dot_bashrc.tmpl`
  - `templates/nushell/config.nu`
  - `templates/starship/starship.toml`
- Added JS workspace scaffolding (`skills`, `agents`) and bun/npm install flows.
- Added JS lockfile/reproducibility checks and CI-oriented `just` tasks (`js-lockfiles-check`, `js-repro-npm`, `js-repro-bun`, `ci-check`).
- Added helper scripts:
  - `scripts/bootstrap.sh`
  - `scripts/install-skills-agents.sh`
  - `scripts/check-js-reproducibility.sh`
- Validated command flow:
  - `cargo run -- doctor`
  - `cargo run -- plan`
  - `cargo run -- apply`
  - `cargo run -- integrate`
  - `cargo run -- rollback --preview`
  - `cargo run -- rollback --force --allow-active-sessions`
  - `cargo run -- rollback --force --allow-active-sessions --allow-external-locks`

## Current blocker

- No compile blocker remains for the Rust scaffold on this host.
- Core compiled tools are now installed (`chezmoi`, `atuin`, `just`) and integrated paths execute.
- Root Bun workspace lockfile is now generated at `bun.lock`; commit it to keep CI reproducibility checks green.
- Atuin first-run auth still needs actual credential provisioning to complete account bootstrap on each host.

## Next implementation items

None pending for the current Phase 1 scope.
