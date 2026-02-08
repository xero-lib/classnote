mod cli;

use cli::{ Cli, Commands };
use helpers::{
    config::{get_config_file, read_or_init_config},
    note::create_note
};

fn main() {
    let args = Cli::get_cli();

    // if there's no custom config path
    let mut config_file = get_config_file(Default::default());
    let mut config = read_or_init_config(&mut config_file);
    
    if let Err(e) = std::env::set_current_dir(config.get_root()) {
        eprintln!("Unable to move into notes directory \"{}\": {e:#}", config.get_root().to_string_lossy())
    };
    
    match args.command {
        Some(Commands::List) => config.print_available_classes(),
        Some(Commands::Update) => todo!(),
        Some(Commands::Add { name }) => todo!(),
        Some(Commands::Remove { name }) => todo!(),
        None => create_note()
    }
}


