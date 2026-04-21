pub fn supported_compiled_tools() -> [&'static str; 5] {
    ["chezmoi", "atuin", "just", "direnv", "bun"]
}

pub fn supported_fallback_tools() -> [&'static str; 1] {
    ["npm"]
}
