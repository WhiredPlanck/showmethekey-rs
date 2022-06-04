mod cli;

use anyhow::Result;
use clap::Parser;
use cli::SubCommands;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.subcommands {
        Some(s) => match s {
            SubCommands::Cli => {}, 
            SubCommands::Gtk => {}
        },
        None => unreachable!(),
    }

    Ok(())
}
