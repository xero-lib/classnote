pub use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name 
    #[arg(short, long)]
    pub name: Option<String>,
}

impl Cli {
    pub fn get_cli() -> Cli {
        return Cli::parse();
    }
}