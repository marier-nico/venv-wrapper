use clap::ArgMatches;
use eyre::{eyre, Result};

use crate::{config::config_data::Config, virtualenv::create_virtualenv};

pub fn cli_main(matches: &ArgMatches, config: &Config) -> Result<()> {
    match matches.subcommand() {
        ("new", Some(_sub_matches)) => create_virtualenv(config),
        _ => Err(eyre!("Unknown command")),
    }
}
