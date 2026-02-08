pub use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name of class to create/edit a note of
    #[arg(short, long)]
    pub name: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn get_cli() -> Cli {
        return Cli::parse();
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lists available classes
    List,
    /// Updates existing class information
    Update,

    /// Adds a new class
    Add {
        /// Class name
        name: String,
    },

    /// Removes a class from the config (does not delete any files)
    Remove {
        /// Class name
        name: String
    }
}