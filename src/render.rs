use crate::model::{PlatformConfig, ToolSpec};
use serde::Serialize;
use serde_json::json;

pub fn render_vscode(config: &PlatformConfig, env_pairs: &[(String, String)]) -> String {
    let enabled = config
        .agents
        .vscode
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    let tool_summary = tool_summary(config);
    let env = env_object(env_pairs);

    serde_json::to_string_pretty(&json!({
        "$schema": "vscode://schemas/settings/default",
        "terminal.integrated.env.linux": env,
        "terminal.integrated.env.osx": env,
        "terminal.integrated.env.windows": env,
        "chat.tools.autoApprove": false,
        "chat.agent.maxRequests": 6,
        "unifiedShellPlatform.enabled": enabled,
        "unifiedShellPlatform.phase": config.core.phase,
        "unifiedShellPlatform.profile": "vscode",
        "unifiedShellPlatform.tools": tool_summary,
    }))
    .unwrap_or_else(|_| "{}".to_string())
        + "\n"
}

pub fn render_claude_code(config: &PlatformConfig, env_pairs: &[(String, String)]) -> String {
    let enabled = config
        .agents
        .claude_code
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    let tool_summary = tool_summary(config);
    let env = env_object(env_pairs);
    let enabled_tools = enabled_tool_names(config);

    serde_json::to_string_pretty(&json!({
        "version": 1,
        "enabled": enabled,
        "environment": {
            "inherit": true,
            "variables": env,
        },
        "permissions": {
            "allow": [
                "Bash(cargo *)",
                "Bash(just *)",
                "Bash(chezmoi *)"
            ],
            "deny": []
        },
        "integrations": {
            "tools": tool_summary,
            "preferred_compiled_tools": enabled_tools,
        },
        "metadata": {
            "phase": config.core.phase,
            "agent": "claude_code",
            "project": config.core.name,
        }
    }))
    .unwrap_or_else(|_| "{}".to_string())
        + "\n"
}

pub fn render_kiro(config: &PlatformConfig, env_pairs: &[(String, String)]) -> String {
    let enabled = config
        .agents
        .kiro
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    let tool_summary = tool_summary(config);
    let env = env_object(env_pairs);
    let enabled_tools = enabled_tool_names(config);

    serde_json::to_string_pretty(&json!({
        "version": 1,
        "enabled": enabled,
        "workspace": {
            "environment": env,
            "terminal": {
                "shell_integration": true,
                "inherit_env": true,
            },
        },
        "assistant": {
            "context": {
                "phase": config.core.phase,
                "project": config.core.name,
            },
            "features": {
                "auto_context": true,
                "include_recent_changes": true,
            },
        },
        "integrations": {
            "tools": tool_summary,
            "preferred_compiled_tools": enabled_tools,
        },
        "metadata": {
            "agent": "kiro",
        }
    }))
    .unwrap_or_else(|_| "{}".to_string())
        + "\n"
}

pub fn render_tool_spec(name: &str, tool: Option<&ToolSpec>) -> String {
    let enabled = tool.map(|item| item.enabled).unwrap_or(false);
    format!("# {name} enabled: {enabled}\n")
}

pub fn render_env_pairs(pairs: &[(String, String)]) -> String {
    let mut output = String::new();
    for (key, value) in pairs {
        output.push_str(&format!("{key}={value}\n"));
    }
    output
}

pub fn render_atuin_config(config: &PlatformConfig) -> String {
    toml::to_string_pretty(&config.atuin).unwrap_or_else(|_| String::new())
}

pub fn render_chezmoi_manifest(config: &PlatformConfig) -> String {
    #[derive(Serialize)]
    struct ChezmoiManifest<'a> {
        managed_files: &'a [crate::model::ManagedFile],
    }

    toml::to_string_pretty(&ChezmoiManifest {
        managed_files: &config.managed_files,
    })
    .unwrap_or_else(|_| "managed_files = []\n".to_string())
}

fn env_object(env_pairs: &[(String, String)]) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, value) in env_pairs {
        map.insert(key.clone(), serde_json::Value::String(value.clone()));
    }
    serde_json::Value::Object(map)
}

fn tool_summary(config: &PlatformConfig) -> serde_json::Value {
    fn tool_meta(tool: Option<&ToolSpec>) -> serde_json::Value {
        json!({
            "enabled": tool.map(|item| item.enabled).unwrap_or(false),
            "source": tool.and_then(|item| item.source.as_deref()).unwrap_or("unspecified"),
            "notes": tool.and_then(|item| item.notes.as_deref()).unwrap_or(""),
        })
    }

    json!({
        "chezmoi": tool_meta(config.tools.chezmoi.as_ref()),
        "atuin": tool_meta(config.tools.atuin.as_ref()),
        "just": tool_meta(config.tools.just.as_ref()),
        "direnv": tool_meta(config.tools.direnv.as_ref()),
        "bun": tool_meta(config.tools.bun.as_ref()),
        "npm": tool_meta(config.tools.npm.as_ref())
    })
}

fn enabled_tool_names(config: &PlatformConfig) -> Vec<&'static str> {
    let mut enabled = Vec::new();

    if config
        .tools
        .chezmoi
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false)
    {
        enabled.push("chezmoi");
    }
    if config
        .tools
        .atuin
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false)
    {
        enabled.push("atuin");
    }
    if config
        .tools
        .just
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false)
    {
        enabled.push("just");
    }
    if config
        .tools
        .direnv
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false)
    {
        enabled.push("direnv");
    }
    if config
        .tools
        .bun
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false)
    {
        enabled.push("bun");
    }
    if config
        .tools
        .npm
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false)
    {
        enabled.push("npm");
    }

    enabled
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AgentSet, AtuinConfig, CoreConfig, ManagedFile, PlatformConfig, ToolSet};

    #[test]
    fn render_pairs_as_env_lines() {
        let rendered = render_env_pairs(&[("A".into(), "1".into())]);
        assert!(rendered.contains("A=1"));
    }

    #[test]
    fn render_chezmoi_manifest_contains_targets() {
        let config = PlatformConfig {
            core: CoreConfig {
                name: "demo".into(),
                phase: "phase-1".into(),
            },
            profiles: vec![],
            atuin: AtuinConfig::default(),
            managed_files: vec![ManagedFile {
                name: "bashrc".into(),
                source: "templates/bash/dot_bashrc.tmpl".into(),
                target: "~/.bashrc".into(),
                template: true,
            }],
            tools: ToolSet::default(),
            agents: AgentSet::default(),
        };

        let manifest = render_chezmoi_manifest(&config);
        assert!(manifest.contains("managed_files"));
        assert!(manifest.contains("~/.bashrc"));
    }

    #[test]
    fn render_atuin_config_contains_sync_settings() {
        let config = PlatformConfig {
            core: CoreConfig {
                name: "demo".into(),
                phase: "phase-1".into(),
            },
            profiles: vec![],
            atuin: AtuinConfig {
                sync_frequency: "15m".into(),
                ..AtuinConfig::default()
            },
            managed_files: vec![],
            tools: ToolSet::default(),
            agents: AgentSet::default(),
        };

        let rendered = render_atuin_config(&config);
        assert!(rendered.contains("sync_frequency"));
        assert!(rendered.contains("15m"));
    }
}
