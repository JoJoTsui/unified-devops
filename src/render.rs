use crate::model::{PlatformConfig, ToolSpec};

pub fn render_vscode(config: &PlatformConfig) -> String {
    let enabled = config
        .agents
        .vscode
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    format!(
        "{{\n  \"phase\": \"phase-1\",\n  \"agent\": \"vscode\",\n  \"enabled\": {enabled}\n}}\n"
    )
}

pub fn render_claude_code(config: &PlatformConfig) -> String {
    let enabled = config
        .agents
        .claude_code
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    format!(
        "{{\n  \"phase\": \"phase-1\",\n  \"agent\": \"claude_code\",\n  \"enabled\": {enabled}\n}}\n"
    )
}

pub fn render_kiro(config: &PlatformConfig) -> String {
    let enabled = config
        .agents
        .kiro
        .as_ref()
        .map(|tool| tool.enabled)
        .unwrap_or(false);

    format!("{{\n  \"phase\": \"phase-1\",\n  \"agent\": \"kiro\",\n  \"enabled\": {enabled}\n}}\n")
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
