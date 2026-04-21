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
    let os_ok = profile.os.as_ref().map(|value| value == &ctx.os).unwrap_or(true);
    let shell_ok = profile.shell.as_ref().map(|value| value == &ctx.shell).unwrap_or(true);
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
