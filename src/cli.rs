use crate::integrations;
use crate::model::PlatformConfig;
use crate::render;
use crate::resolver::{merged_env, resolve_profile, Context};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

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
    println!("plan: agents in scope = vscode, claude_code, kiro");
    Ok(())
}

pub fn apply() -> Result<()> {
    let config = load_config()?;
    write_generated(&config)?;
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
    println!("integrate: scaffolded integration command");
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
        "generated/chezmoi/managed-files.toml",
        render::render_chezmoi_manifest(config),
    )?;

    Ok(())
}
