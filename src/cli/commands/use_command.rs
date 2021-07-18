use std::convert::TryFrom;

use clap::ArgMatches;
use eyre::Result;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    shell::{
        operations::{
            activate_operation::ActivateOperation, cd_operation::CdOperation,
            combined_operation::CombinedOperation,
        },
        shell_operation::ShellOperation,
    },
    virtualenv::virtualenv_info::Virtualenv,
};

pub struct UseCommand;
impl Command for UseCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let venv_name = matches.value_of("name").unwrap();
        let parent_dir = config.venv_root.as_ref().unwrap();
        let venv = Virtualenv::try_from(parent_dir.join(venv_name).as_ref())?;

        let mut shell_operations: Vec<Box<dyn ShellOperation>> = Vec::new();

        if let Some(project) = venv.project_path() {
            shell_operations.push(Box::new(CdOperation::to(&project)));
        }

        shell_operations.push(Box::new(ActivateOperation::new(venv)));

        Ok(CommandResult::new().operation(Box::new(CombinedOperation::new(shell_operations))))
    }
}
