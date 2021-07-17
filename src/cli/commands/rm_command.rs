use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    virtualenv::{deletion::delete_virtualenv, discovery::find_in_path},
};
use clap::ArgMatches;
use eyre::Result;

pub struct RmCommand;
impl Command for RmCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let venv_name = matches.value_of("name").unwrap();
        let parent_dir = config.venv_root.as_ref().unwrap();
        let venv = find_in_path(venv_name, parent_dir)?;

        delete_virtualenv(&venv)?;

        Ok(CommandResult::new())
    }
}
