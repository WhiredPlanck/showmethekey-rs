use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommands: Option<SubCommands>,
}

#[derive(Parser, Debug)]
pub enum SubCommands {
    /// Launch the command line backend of Show Me The Key.
    Cli,
    /// Launch the GTK GUI frontend of Show Me The Key.
    Gtk,
}
