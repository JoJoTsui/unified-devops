use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeployState {
    pub version: String,
    pub timestamp_unix: u64,
    pub managed_backups: Vec<ManagedBackup>,
    pub generated_paths: Vec<String>,
    pub chezmoi_apply_attempted: bool,
    pub chezmoi_apply_succeeded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedBackup {
    pub target: String,
    pub backup_path: String,
    pub existed: bool,
}
