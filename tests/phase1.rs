use unified_shell_platform::model::{
    AgentSet, AtuinConfig, CoreConfig, EnvVar, PlatformConfig, Profile, ToolSet,
};
use unified_shell_platform::render;
use unified_shell_platform::resolver::{merged_env, resolve_profile, Context};

#[test]
fn profile_resolution_prefers_specific_overlay() {
    let config = PlatformConfig {
        core: CoreConfig {
            name: "demo".into(),
            phase: "phase-1".into(),
        },
        profiles: vec![
            Profile {
                name: "base".into(),
                host_profile: Some("default".into()),
                interactive: Some(true),
                vars: vec![EnvVar {
                    key: "A".into(),
                    value: "1".into(),
                }],
                ..Default::default()
            },
            Profile {
                name: "bash".into(),
                shell: Some("bash".into()),
                host_profile: Some("default".into()),
                interactive: Some(true),
                vars: vec![EnvVar {
                    key: "A".into(),
                    value: "2".into(),
                }],
                ..Default::default()
            },
        ],
        atuin: AtuinConfig::default(),
        managed_files: vec![],
        tools: ToolSet::default(),
        agents: AgentSet::default(),
    };

    let ctx = Context {
        os: std::env::consts::OS.to_string(),
        shell: "bash".into(),
        agent_ide: None,
        host_profile: Some("default".into()),
        interactive: true,
    };

    let profiles = resolve_profile(&config, &ctx);
    let merged = merged_env(&profiles);

    assert_eq!(profiles.len(), 2);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].value, "2");
}

#[test]
fn renders_agent_outputs_with_env_and_tool_summary() {
    let config = PlatformConfig {
        core: CoreConfig {
            name: "demo".into(),
            phase: "phase-1".into(),
        },
        profiles: vec![],
        atuin: AtuinConfig::default(),
        managed_files: vec![],
        tools: ToolSet::default(),
        agents: AgentSet::default(),
    };

    let env_pairs = vec![("X".into(), "1".into())];
    let vscode = render::render_vscode(&config, &env_pairs);
    let claude = render::render_claude_code(&config, &env_pairs);
    let kiro = render::render_kiro(&config, &env_pairs);

    assert!(vscode.contains("\"agent\": \"vscode\""));
    assert!(vscode.contains("\"X\": \"1\""));
    assert!(claude.contains("\"agent\": \"claude_code\""));
    assert!(kiro.contains("\"agent\": \"kiro\""));
}
