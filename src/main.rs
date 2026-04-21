mod model;
mod cli;
mod resolver;
mod render;
mod integrations;
mod state;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => cli::doctor()?,
        Commands::Plan => cli::plan()?,
        Commands::Apply => cli::apply()?,
        Commands::Sync => cli::sync()?,
        Commands::Rollback => cli::rollback()?,
        Commands::Integrate => cli::integrate()?,
    }

    Ok(())
}
