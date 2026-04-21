use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeployState {
    pub version: String,
    pub applied_profiles: Vec<String>,
    pub rendered_targets: Vec<String>,
}
