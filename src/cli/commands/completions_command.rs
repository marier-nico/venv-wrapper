use clap::ArgMatches;
use eyre::Result;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    shell::operations::completion_operation::CompletionOperation,
};

pub struct CompletionsCommand;
impl Command for CompletionsCommand {
    fn run(config: &Config, _matches: &ArgMatches) -> Result<CommandResult> {
        let parent_dir = config.venv_root.as_ref().unwrap();

        Ok(CommandResult::new()
            .operation(Box::new(CompletionOperation::new(parent_dir.to_owned()))))
    }
}
