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

    serde_json::to_string_pretty(&json!({
        "phase": "phase-1",
        "agent": "vscode",
        "enabled": enabled,
        "terminal": {
            "integrated": {
                "env": {
                    "linux": env_object(env_pairs),
                    "windows": env_object(env_pairs)
                }
            }
        },
        "tools": tool_summary,
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

    serde_json::to_string_pretty(&json!({
        "phase": "phase-1",
        "agent": "claude_code",
        "enabled": enabled,
        "env": env_object(env_pairs),
        "tools": tool_summary,
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

    serde_json::to_string_pretty(&json!({
        "phase": "phase-1",
        "agent": "kiro",
        "enabled": enabled,
        "env": env_object(env_pairs),
        "tools": tool_summary,
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
    json!({
        "chezmoi": render_tool_spec("chezmoi", config.tools.chezmoi.as_ref()),
        "atuin": render_tool_spec("atuin", config.tools.atuin.as_ref()),
        "just": render_tool_spec("just", config.tools.just.as_ref()),
        "direnv": render_tool_spec("direnv", config.tools.direnv.as_ref()),
        "bun": render_tool_spec("bun", config.tools.bun.as_ref()),
        "npm": render_tool_spec("npm", config.tools.npm.as_ref())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AgentSet, CoreConfig, ManagedFile, PlatformConfig, ToolSet};

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
}
