mod cli;

use cli::Cli;
use helpers::config::{get_config_file, read_or_init_config};

fn main() {
    let args = Cli::get_cli();

    // if there's no custom config path
    let mut config_file = get_config_file(Default::default());
    let mut config = read_or_init_config(&mut config_file);
    
    if let Err(e) = std::env::set_current_dir(config.get_root()) {
        eprintln!("Unable to move into notes directory \"{}\": {e:#}", config.get_root().to_string_lossy())
    };
    
    config.print_available_classes();

    // manually add paths to config for immediate open from anywhere?
    // add professors etc to init
    // implement copying original times to next day on empty enter
    // add option to navigate to path manually during config initialization?
    
}


