use std::convert::TryFrom;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    virtualenv::{deletion::delete_virtualenv, virtualenv_info::Virtualenv},
};
use clap::ArgMatches;
use eyre::Result;

pub struct RmCommand;
impl Command for RmCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let venv_name = matches.value_of("name").unwrap();
        let parent_dir = config.venv_root.as_ref().unwrap();
        let venv = Virtualenv::try_from(parent_dir.join(venv_name).as_ref())?;

        delete_virtualenv(&venv)?;

        Ok(CommandResult::new())
    }
}
