use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    shell::operations::activate_operation::ActivateOperation,
    virtualenv::discovery::find_in_path,
};
use clap::ArgMatches;
use eyre::Result;

pub struct ActivateCommand;
impl Command for ActivateCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let venv_name = matches.value_of("name").unwrap();
        let parent_dir = config.venv_root.as_ref().unwrap();
        let venv = find_in_path(venv_name, parent_dir)?;

        Ok(CommandResult::new().operation(Box::new(ActivateOperation::new(venv))))
    }
}
