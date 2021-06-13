use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    shell::operations::init_operation::InitOperation,
};
use clap::ArgMatches;
use eyre::Result;

pub struct InitCommand {}
impl Command for InitCommand {
    fn run(_config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let shell_to_init = matches.value_of("shell").unwrap();

        Ok(CommandResult::new()
            .with_shell(shell_to_init.to_owned())
            .operation(Box::new(InitOperation {})))
    }
}
