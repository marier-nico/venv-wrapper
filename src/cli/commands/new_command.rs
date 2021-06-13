use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    virtualenv::create_virtualenv,
};
use clap::ArgMatches;
use eyre::Result;

pub struct NewCommand {}
impl Command for NewCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        create_virtualenv(
            config,
            matches.value_of("python_executable").unwrap(),
            matches.value_of("name").unwrap(),
        )?;

        Ok(CommandResult::new()
            .output(Box::new(String::from("Success, you now have a new virtualenv!"))))
    }
}
