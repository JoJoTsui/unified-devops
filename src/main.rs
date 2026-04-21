use anyhow::Result;
use clap::Parser;
use unified_shell_platform::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => unified_shell_platform::cli::doctor()?,
        Commands::Plan => unified_shell_platform::cli::plan()?,
        Commands::Apply => unified_shell_platform::cli::apply()?,
        Commands::Sync => unified_shell_platform::cli::sync()?,
        Commands::Rollback => unified_shell_platform::cli::rollback()?,
        Commands::Integrate => unified_shell_platform::cli::integrate()?,
    }

    Ok(())
}
