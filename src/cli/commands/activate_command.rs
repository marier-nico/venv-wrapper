use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
};
use clap::ArgMatches;
use eyre::Result;

pub struct ActivateCommand {}
impl Command for ActivateCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        todo!()
    }
}
