use clap::ArgMatches;
use eyre::Result;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    shell::operations::completion_operation::CompletionOperation,
};

pub struct CompletionsCommand;
impl Command for CompletionsCommand {
    fn run(_config: &Config, _matches: &ArgMatches) -> Result<CommandResult> {
        Ok(CommandResult::new().operation(Box::new(CompletionOperation {})))
    }
}
