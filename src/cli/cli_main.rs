use clap::ArgMatches;
use eyre::{eyre, Result};

use crate::{cli::{command::Command, commands::{init_command::InitCommand, new_command::NewCommand}}, config::config_data::Config};

use super::command::CommandResult;

pub fn cli_main(matches: &ArgMatches, config: &Config) -> Result<CommandResult> {
    match matches.subcommand() {
        ("init", Some(sub_matches)) => InitCommand::run(config, sub_matches),
        ("new", Some(sub_matches)) => NewCommand::run(config, sub_matches),
        _ => Err(eyre!("Unknown command")),
    }
}
