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
    Delete
}