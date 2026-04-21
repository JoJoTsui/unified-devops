use crate::integrations;
use crate::model::PlatformConfig;
use crate::render;
use crate::resolver::{merged_env, resolve_profile, Context};
use crate::state::{DeployState, ManagedBackup};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const DEPLOY_STATE_PATH: &str = "generated/state/deploy-state.json";
const ROLLBACK_BACKUP_ROOT: &str = "generated/rollback-backups";

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "Unified shell and agent platform orchestrator"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Doctor,
    Plan,
    Apply,
    Sync,
    Rollback {
        #[arg(long)]
        preview: bool,
    },
    Integrate,
}

pub fn doctor() -> Result<()> {
    let config = load_config()?;
    println!("doctor: Phase 1 scaffold is present");
    println!("config: {} ({})", config.core.name, config.core.phase);
    println!(
        "compiled preferred tools: {:?}",
        integrations::supported_compiled_tools()
    );
    println!(
        "compat fallback tools: {:?}",
        integrations::supported_fallback_tools()
    );
    Ok(())
}

pub fn plan() -> Result<()> {
    let config = load_config()?;
    let ctx = default_context();
    let profiles = resolve_profile(&config, &ctx);
    let env = merged_env(&profiles);
    println!("plan: matched profiles = {}", profiles.len());
    println!("plan: resolved env vars = {}", env.len());
    println!("plan: managed files = {}", config.managed_files.len());
    println!("plan: atuin enabled = {}", config.atuin.enabled);
    println!("plan: agents in scope = vscode, claude_code, kiro");
    Ok(())
}

pub fn apply() -> Result<()> {
    let config = load_config()?;
    write_generated(&config)?;
    let mut deploy_state = create_deploy_state(&config)?;

    match maybe_run_chezmoi_apply(&config)? {
        true => {
            deploy_state.chezmoi_apply_attempted = true;
            deploy_state.chezmoi_apply_succeeded = true;
        }
        false => {
            deploy_state.chezmoi_apply_attempted = false;
            deploy_state.chezmoi_apply_succeeded = false;
        }
    }

    save_deploy_state(&deploy_state)?;
    println!("apply: generated phase-1 artifacts in generated/");
    Ok(())
}

pub fn sync() -> Result<()> {
    let config = load_config()?;
    write_generated(&config)?;
    println!("sync: regenerated phase-1 artifacts in generated/");
    Ok(())
}

pub fn rollback(preview: bool) -> Result<()> {
    if !Path::new(DEPLOY_STATE_PATH).exists() {
        println!("rollback: no deploy state found; nothing to do");
        return Ok(());
    }

    let state = load_deploy_state()?;

    if preview {
        println!(
            "rollback preview: would restore {} managed targets",
            state.managed_backups.len()
        );
        for line in rollback_preview_lines(&state) {
            println!("{line}");
        }
        return Ok(());
    }

    restore_managed_targets(&state)?;

    if state.chezmoi_apply_succeeded {
        println!("rollback: previous chezmoi apply succeeded; local target restore completed");
    } else {
        println!("rollback: restored local targets from backups");
    }

    cleanup_generated_paths(&state)?;
    if Path::new(DEPLOY_STATE_PATH).exists() {
        fs::remove_file(DEPLOY_STATE_PATH)?;
    }
    println!("rollback: completed");
    Ok(())
}

pub fn integrate() -> Result<()> {
    let config = load_config()?;
    write_generated(&config)?;
    maybe_run_chezmoi_diff(&config)?;
    println!("integrate: executed generated artifact + chezmoi diff flow");
    println!("phase-1 tools: chezmoi, atuin, just, direnv, bun, npm");
    println!("phase-1 agent targets: vscode, claude_code, kiro");
    Ok(())
}

fn load_config() -> Result<PlatformConfig> {
    let raw = fs::read_to_string("config/platform.toml")?;
    let config: PlatformConfig = toml::from_str(&raw)?;
    Ok(config)
}

fn default_context() -> Context {
    Context {
        os: std::env::consts::OS.to_string(),
        shell: std::env::var("SHELL")
            .ok()
            .and_then(|path| {
                Path::new(&path)
                    .file_name()
                    .map(|value| value.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| "bash".to_string()),
        agent_ide: None,
        host_profile: Some("default".to_string()),
        interactive: true,
    }
}

fn write_generated(config: &PlatformConfig) -> Result<()> {
    let ctx = default_context();
    let profiles = resolve_profile(config, &ctx);
    let env = merged_env(&profiles);
    let env_pairs: Vec<(String, String)> =
        env.into_iter().map(|item| (item.key, item.value)).collect();

    fs::create_dir_all("generated/agents")?;
    fs::create_dir_all("generated/atuin")?;
    fs::create_dir_all("generated/chezmoi")?;
    fs::create_dir_all("generated/env")?;

    fs::write(
        "generated/agents/vscode.json",
        render::render_vscode(config, &env_pairs),
    )?;
    fs::write(
        "generated/agents/claude_code.json",
        render::render_claude_code(config, &env_pairs),
    )?;
    fs::write(
        "generated/agents/kiro.json",
        render::render_kiro(config, &env_pairs),
    )?;

    fs::write(
        "generated/env/resolved.env",
        render::render_env_pairs(&env_pairs),
    )?;
    fs::write(
        "generated/atuin/config.toml",
        render::render_atuin_config(config),
    )?;
    fs::write(
        "generated/chezmoi/managed-files.toml",
        render::render_chezmoi_manifest(config),
    )?;
    write_chezmoi_source_state(config)?;

    Ok(())
}

fn write_chezmoi_source_state(config: &PlatformConfig) -> Result<()> {
    let source_root = Path::new("generated/chezmoi/source-state");
    fs::create_dir_all(source_root)?;

    for managed in &config.managed_files {
        let src = Path::new(&managed.source);
        if !src.exists() {
            println!("chezmoi: source missing, skipping: {}", managed.source);
            continue;
        }

        let rel = map_target_to_chezmoi_path(&managed.target, managed.template);
        let dest = source_root.join(rel);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, &dest)?;
    }

    Ok(())
}

fn map_target_to_chezmoi_path(target: &str, template: bool) -> PathBuf {
    let trimmed = target.trim_start_matches("~/").trim_start_matches('/');
    let mut parts: Vec<String> = trimmed
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(segment_to_chezmoi)
        .collect();

    if parts.is_empty() {
        parts.push("dot_placeholder".to_string());
    }

    if template {
        let last = parts.len() - 1;
        if !parts[last].ends_with(".tmpl") {
            parts[last].push_str(".tmpl");
        }
    }

    parts.into_iter().collect()
}

fn segment_to_chezmoi(segment: &str) -> String {
    if let Some(stripped) = segment.strip_prefix('.') {
        if stripped.is_empty() {
            return "dot_".to_string();
        }
        return format!("dot_{stripped}");
    }
    segment.to_string()
}

fn maybe_run_chezmoi_diff(config: &PlatformConfig) -> Result<bool> {
    let enabled = config
        .tools
        .chezmoi
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    if !enabled {
        println!("chezmoi: disabled in config; skipping diff");
        return Ok(false);
    }

    run_chezmoi_subcommand("diff")
}

fn maybe_run_chezmoi_apply(config: &PlatformConfig) -> Result<bool> {
    let enabled = config
        .tools
        .chezmoi
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    if !enabled {
        println!("chezmoi: disabled in config; skipping apply");
        return Ok(false);
    }

    run_chezmoi_subcommand("apply")
}

fn run_chezmoi_subcommand(subcommand: &str) -> Result<bool> {
    if !command_exists("chezmoi") {
        println!("chezmoi: not installed; skipping {subcommand}");
        return Ok(false);
    }

    let source_path = Path::new("generated/chezmoi/source-state");
    let mut command = Command::new("chezmoi");
    command
        .arg("-S")
        .arg(source_path)
        .arg("-D")
        .arg(home_dir())
        .arg("--no-tty")
        .arg(subcommand);

    if subcommand == "apply" {
        command.arg("--force");
    }

    let status = command.status()?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "chezmoi {} failed with status {}",
            subcommand,
            status
        ));
    }

    println!("chezmoi: {} completed", subcommand);
    Ok(true)
}

fn command_exists(command: &str) -> bool {
    Command::new(command).arg("--version").output().is_ok()
}

fn home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "~".to_string())
}

fn create_deploy_state(config: &PlatformConfig) -> Result<DeployState> {
    fs::create_dir_all(ROLLBACK_BACKUP_ROOT)?;

    let mut backups = Vec::new();
    for managed in &config.managed_files {
        let target_path = expand_target_path(&managed.target);
        let existed = target_path.exists();
        let backup_name = sanitize_target_for_backup(&managed.target);
        let backup_path = Path::new(ROLLBACK_BACKUP_ROOT).join(format!("{backup_name}.bak"));

        if existed {
            if let Some(parent) = backup_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&target_path, &backup_path)?;
        }

        backups.push(ManagedBackup {
            target: managed.target.clone(),
            backup_path: backup_path.to_string_lossy().to_string(),
            existed,
        });
    }

    Ok(DeployState {
        version: "phase-1".to_string(),
        timestamp_unix: current_timestamp_unix(),
        managed_backups: backups,
        generated_paths: vec![
            "generated/agents".to_string(),
            "generated/atuin/config.toml".to_string(),
            "generated/chezmoi/managed-files.toml".to_string(),
            "generated/chezmoi/source-state".to_string(),
            "generated/env/resolved.env".to_string(),
        ],
        chezmoi_apply_attempted: false,
        chezmoi_apply_succeeded: false,
    })
}

fn save_deploy_state(state: &DeployState) -> Result<()> {
    let raw = serde_json::to_string_pretty(state)?;
    let path = Path::new(DEPLOY_STATE_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, raw)?;
    Ok(())
}

fn load_deploy_state() -> Result<DeployState> {
    let raw = fs::read_to_string(DEPLOY_STATE_PATH)?;
    let state: DeployState = serde_json::from_str(&raw)?;
    Ok(state)
}

fn restore_managed_targets(state: &DeployState) -> Result<()> {
    for backup in &state.managed_backups {
        let target = expand_target_path(&backup.target);
        let backup_path = Path::new(&backup.backup_path);

        if backup.existed {
            if !backup_path.exists() {
                println!("rollback: missing backup for {}; skipping", backup.target);
                continue;
            }
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(backup_path, &target)?;
        } else if target.exists() {
            fs::remove_file(&target)?;
        }
    }

    Ok(())
}

fn cleanup_generated_paths(state: &DeployState) -> Result<()> {
    for path in &state.generated_paths {
        let p = Path::new(path);
        if p.is_dir() {
            fs::remove_dir_all(p)?;
        } else if p.is_file() {
            fs::remove_file(p)?;
        }
    }

    let backup_root = Path::new(ROLLBACK_BACKUP_ROOT);
    if backup_root.exists() {
        fs::remove_dir_all(backup_root)?;
    }
    Ok(())
}

fn sanitize_target_for_backup(target: &str) -> String {
    target
        .replace('~', "home")
        .replace('/', "__")
        .replace('.', "dot")
        .replace(':', "_")
}

fn expand_target_path(target: &str) -> PathBuf {
    if let Some(rest) = target.strip_prefix("~/") {
        return Path::new(&home_dir()).join(rest);
    }

    PathBuf::from(target)
}

fn current_timestamp_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn rollback_preview_lines(state: &DeployState) -> Vec<String> {
    let mut lines = Vec::new();
    for backup in &state.managed_backups {
        let target = expand_target_path(&backup.target);
        if backup.existed {
            lines.push(format!(
                "would restore {} from {}",
                target.display(),
                backup.backup_path
            ));
        } else {
            lines.push(format!("would remove {}", target.display()));
        }
    }

    for path in &state.generated_paths {
        lines.push(format!("would remove generated path {path}"));
    }

    lines.push(format!("would remove deploy state {}", DEPLOY_STATE_PATH));
    lines.push(format!(
        "would remove rollback backups {}",
        ROLLBACK_BACKUP_ROOT
    ));
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_target_to_chezmoi_path_handles_dot_paths() {
        let mapped = map_target_to_chezmoi_path("~/.config/nushell/config.nu", false);
        assert_eq!(mapped, PathBuf::from("dot_config/nushell/config.nu"));

        let mapped_template = map_target_to_chezmoi_path("~/.bashrc", true);
        assert_eq!(mapped_template, PathBuf::from("dot_bashrc.tmpl"));
    }

    #[test]
    fn sanitize_target_for_backup_is_stable() {
        let sanitized = sanitize_target_for_backup("~/.config/nushell/config.nu");
        assert_eq!(sanitized, "home__dotconfig__nushell__configdotnu");
    }

    #[test]
    fn rollback_preview_lines_include_restore_and_cleanup() {
        let state = DeployState {
            version: "phase-1".into(),
            timestamp_unix: 1,
            managed_backups: vec![ManagedBackup {
                target: "~/.bashrc".into(),
                backup_path: "generated/rollback-backups/home__dotbashrc.bak".into(),
                existed: true,
            }],
            generated_paths: vec!["generated/env/resolved.env".into()],
            chezmoi_apply_attempted: true,
            chezmoi_apply_succeeded: true,
        };

        let lines = rollback_preview_lines(&state);
        assert!(lines.iter().any(|line| line.contains("would restore")));
        assert!(lines
            .iter()
            .any(|line| line.contains("generated/env/resolved.env")));
    }
}
