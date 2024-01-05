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
    /// Sync local save data with remote save data
    Sync
}