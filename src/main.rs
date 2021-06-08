use std::convert::TryFrom;

use cli::get_app::get_app;
use config::config_data::Config;

use crate::cli::cli_main::cli_main;

mod cli;
mod config;
mod virtualenv;

fn main() {
    let matches = get_app().get_matches();
    let cli_config = Config::try_from(&matches).unwrap();
    let config = Config::merge(vec![Config::default(), cli_config]);

    let result = cli_main(&matches, &config);
    if let Err(e) = result {
        print!("{:?}", e);
    }
}
