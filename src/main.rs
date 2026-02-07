mod data;
mod helpers;

use helpers::config::{get_config_file, read_or_init_config};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    // let args = Args::parse();
    let mut config_file = get_config_file(Default::default());
    let mut config = read_or_init_config(&mut config_file);
}


