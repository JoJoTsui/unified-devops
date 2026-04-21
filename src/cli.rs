use crate::integrations;
use crate::model::PlatformConfig;
use crate::render;
use crate::resolver::{merged_env, resolve_profile, Context};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    Rollback,
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
    maybe_run_chezmoi_apply(&config)?;
    println!("apply: generated phase-1 artifacts in generated/");
    Ok(())
}

pub fn sync() -> Result<()> {
    let config = load_config()?;
    write_generated(&config)?;
    println!("sync: regenerated phase-1 artifacts in generated/");
    Ok(())
}

pub fn rollback() -> Result<()> {
    println!("rollback: scaffolded rollback command");
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

fn maybe_run_chezmoi_diff(config: &PlatformConfig) -> Result<()> {
    let enabled = config
        .tools
        .chezmoi
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    if !enabled {
        println!("chezmoi: disabled in config; skipping diff");
        return Ok(());
    }

    run_chezmoi_subcommand("diff")
}

fn maybe_run_chezmoi_apply(config: &PlatformConfig) -> Result<()> {
    let enabled = config
        .tools
        .chezmoi
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    if !enabled {
        println!("chezmoi: disabled in config; skipping apply");
        return Ok(());
    }

    run_chezmoi_subcommand("apply")
}

fn run_chezmoi_subcommand(subcommand: &str) -> Result<()> {
    if !command_exists("chezmoi") {
        println!("chezmoi: not installed; skipping {subcommand}");
        return Ok(());
    }

    let source_path = Path::new("generated/chezmoi/source-state");
    let status = Command::new("chezmoi")
        .arg(subcommand)
        .arg("--source-path")
        .arg(source_path)
        .arg("--destination")
        .arg(home_dir())
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "chezmoi {} failed with status {}",
            subcommand,
            status
        ));
    }

    println!("chezmoi: {} completed", subcommand);
    Ok(())
}

fn command_exists(command: &str) -> bool {
    Command::new(command).arg("--version").output().is_ok()
}

fn home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "~".to_string())
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
}
