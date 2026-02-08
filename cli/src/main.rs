mod cli;

use cli::Cli;
use helpers::config::{get_config_file, read_or_init_config};

fn main() {
    let args = Cli::get_cli();

    // if there's no custom config path
    let mut config_file = get_config_file(Default::default());
    let mut config = read_or_init_config(&mut config_file);

    // manually add paths to config for immediate open from anywhere?
    // add professors etc to init
    // implement copying original times to next day on empty enter
    // add option to navigate to path manually during config initialization?
    
}


