use std::convert::TryFrom;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    shell::operations::activate_operation::ActivateOperation,
    virtualenv::virtualenv_info::Virtualenv,
};
use clap::ArgMatches;
use eyre::Result;

pub struct ActivateCommand;
impl Command for ActivateCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let venv_name = matches.value_of("name").unwrap();
        let parent_dir = config.venv_root.as_ref().unwrap();
        let venv = Virtualenv::try_from(parent_dir.join(venv_name).as_ref())?;

        Ok(CommandResult::new().operation(Box::new(ActivateOperation::new(venv))))
    }
}
