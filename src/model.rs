use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PlatformConfig {
    pub core: CoreConfig,
    pub profiles: Vec<Profile>,
    pub atuin: AtuinConfig,
    pub managed_files: Vec<ManagedFile>,
    pub tools: ToolSet,
    pub agents: AgentSet,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CoreConfig {
    pub name: String,
    pub phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub os: Option<String>,
    pub shell: Option<String>,
    pub agent_ide: Option<String>,
    pub host_profile: Option<String>,
    pub interactive: Option<bool>,
    pub vars: Vec<EnvVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagedFile {
    pub name: String,
    pub source: String,
    pub target: String,
    pub template: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AtuinConfig {
    pub enabled: bool,
    pub auto_sync: bool,
    pub sync_frequency: String,
    pub db_path: String,
    pub key_path: String,
    pub records: bool,
    pub filter_mode: String,
    pub workspaces: bool,
    pub timezone: String,
}

impl Default for AtuinConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_sync: true,
            sync_frequency: "10m".to_string(),
            db_path: "~/.local/share/atuin/history.db".to_string(),
            key_path: "~/.local/share/atuin/key".to_string(),
            records: true,
            filter_mode: "global".to_string(),
            workspaces: true,
            timezone: "local".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ToolSet {
    pub chezmoi: Option<ToolSpec>,
    pub atuin: Option<ToolSpec>,
    pub just: Option<ToolSpec>,
    pub direnv: Option<ToolSpec>,
    pub bun: Option<ToolSpec>,
    pub npm: Option<ToolSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AgentSet {
    pub vscode: Option<ToolSpec>,
    pub claude_code: Option<ToolSpec>,
    pub kiro: Option<ToolSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ToolSpec {
    pub enabled: bool,
    pub source: Option<String>,
    pub notes: Option<String>,
}
