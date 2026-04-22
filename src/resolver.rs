use crate::model::{EnvVar, PlatformConfig, Profile};

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub os: String,
    pub shell: String,
    pub agent_ide: Option<String>,
    pub host_profile: Option<String>,
    pub interactive: bool,
}

pub fn resolve_profile<'a>(config: &'a PlatformConfig, ctx: &Context) -> Vec<&'a Profile> {
    config
        .profiles
        .iter()
        .filter(|profile| matches_context(profile, ctx))
        .collect()
}

fn matches_context(profile: &Profile, ctx: &Context) -> bool {
    let os_ok = profile
        .os
        .as_ref()
        .map(|value| value == &ctx.os)
        .unwrap_or(true);
    let shell_ok = profile
        .shell
        .as_ref()
        .map(|value| value == &ctx.shell)
        .unwrap_or(true);
    let agent_ok = profile
        .agent_ide
        .as_ref()
        .map(|value| ctx.agent_ide.as_deref() == Some(value.as_str()))
        .unwrap_or(true);
    let host_ok = profile
        .host_profile
        .as_ref()
        .map(|value| ctx.host_profile.as_deref() == Some(value.as_str()))
        .unwrap_or(true);
    let interactive_ok = profile
        .interactive
        .map(|value| value == ctx.interactive)
        .unwrap_or(true);

    os_ok && shell_ok && agent_ok && host_ok && interactive_ok
}

pub fn merged_env(profiles: &[&Profile]) -> Vec<EnvVar> {
    let mut merged: Vec<EnvVar> = Vec::new();

    for profile in profiles {
        for var in &profile.vars {
            if let Some(existing) = merged.iter_mut().find(|item| item.key == var.key) {
                existing.value = var.value.clone();
            } else {
                merged.push(var.clone());
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AgentSet, AtuinConfig, CoreConfig, PlatformConfig, ToolSet};

    fn base_config(profiles: Vec<Profile>) -> PlatformConfig {
        PlatformConfig {
            core: CoreConfig {
                name: "test".into(),
                phase: "phase-1".into(),
            },
            profiles,
            atuin: AtuinConfig::default(),
            managed_files: vec![],
            tools: ToolSet::default(),
            agents: AgentSet::default(),
        }
    }

    #[test]
    fn resolve_profile_filters_by_host_and_interactive() {
        let config = base_config(vec![
            Profile {
                name: "base".into(),
                vars: vec![EnvVar {
                    key: "A".into(),
                    value: "1".into(),
                }],
                ..Default::default()
            },
            Profile {
                name: "host-specific".into(),
                host_profile: Some("server-a".into()),
                vars: vec![EnvVar {
                    key: "HOST".into(),
                    value: "server-a".into(),
                }],
                ..Default::default()
            },
            Profile {
                name: "non-interactive".into(),
                interactive: Some(false),
                vars: vec![EnvVar {
                    key: "MODE".into(),
                    value: "batch".into(),
                }],
                ..Default::default()
            },
        ]);

        let ctx = Context {
            os: std::env::consts::OS.to_string(),
            shell: "bash".into(),
            agent_ide: None,
            host_profile: Some("server-a".into()),
            interactive: true,
        };

        let resolved = resolve_profile(&config, &ctx);
        assert_eq!(resolved.len(), 2);
        assert_eq!(resolved[0].name, "base");
        assert_eq!(resolved[1].name, "host-specific");

        let merged = merged_env(&resolved);
        assert!(merged.iter().any(|item| item.key == "HOST"));
        assert!(!merged.iter().any(|item| item.key == "MODE"));
    }

    #[test]
    fn merged_env_uses_last_matching_profile_value() {
        let config = base_config(vec![
            Profile {
                name: "base".into(),
                vars: vec![EnvVar {
                    key: "EDITOR".into(),
                    value: "vim".into(),
                }],
                ..Default::default()
            },
            Profile {
                name: "shell-overlay".into(),
                shell: Some("bash".into()),
                vars: vec![EnvVar {
                    key: "EDITOR".into(),
                    value: "hx".into(),
                }],
                ..Default::default()
            },
            Profile {
                name: "agent-overlay".into(),
                shell: Some("bash".into()),
                agent_ide: Some("kiro".into()),
                vars: vec![EnvVar {
                    key: "EDITOR".into(),
                    value: "nvim".into(),
                }],
                ..Default::default()
            },
        ]);

        let ctx = Context {
            os: std::env::consts::OS.to_string(),
            shell: "bash".into(),
            agent_ide: Some("kiro".into()),
            host_profile: Some("default".into()),
            interactive: true,
        };

        let resolved = resolve_profile(&config, &ctx);
        let merged = merged_env(&resolved);

        let editor = merged
            .iter()
            .find(|item| item.key == "EDITOR")
            .map(|item| item.value.as_str());
        assert_eq!(editor, Some("nvim"));
    }
}
