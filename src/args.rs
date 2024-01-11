use std::path::Path;

use clap:: {
    Parser,
    Subcommand
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

// Thinking of organizing commands like git commands 
// in order to be user familiar, needs some refactoring
#[derive(Subcommand)] 
pub enum Commands {
    /// Creates a new savesink data folder
    Create,
    /// Removes existing savesink data folder
    Delete,
    /// Add a new save directory to track
    Add {
        #[arg(short,long, value_name = "STRING")]
        name: String,
        #[arg(short,long, value_name = "FILE")]
        path: String
    },
    /// Remove tracked save directory
    Remove { 
        #[arg(short,long, value_name = "STRING")]
        name: String,
    },
    Pull,
    /// Sync local save data with remote save data
    Sync,
    /// Commit local save data differences from source folders to local save data
    Commit,
    /// Push local save data to remote
    Push,
    /// List local save data
    List {
        #[arg(short,long)]
        verbose: bool,
    },
    Server {
        #[command(subcommand)]
        command: Option<ServerCommands>
    }
}

#[derive(Subcommand)]
pub enum ServerCommands {
    /// Launch a new server instance
    Start,
    /// Shutdown existing server instancce
    Stop
}