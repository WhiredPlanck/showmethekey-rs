mod cli;

use std::process;

use anyhow::Result;
use clap::Parser;
use cli::SubCommands;

#[inline]
fn is_root() -> bool {
    return nix::unistd::geteuid().is_root();
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.subcommands {
        Some(s) => match s {
            SubCommands::Cli => {
                if !is_root() {
                    println!("Please run me as root!");
                    process::exit(1);
                }
            }, 
            SubCommands::Gtk => {}
        },
        None => unreachable!(),
    }

    Ok(())
}
