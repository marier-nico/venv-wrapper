use std::convert::TryFrom;

use clap::ArgMatches;
use eyre::Result;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    virtualenv::virtualenv_info::Virtualenv,
};

pub struct LinkCommand;
impl Command for LinkCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let venv_name = matches.value_of("name").unwrap();
        let parent_dir = config.venv_root.as_ref().unwrap();
        let _venv = Virtualenv::try_from(parent_dir.join(venv_name).as_ref())?;

        todo!()
    }
}
