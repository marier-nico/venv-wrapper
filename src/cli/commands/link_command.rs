use std::{convert::TryFrom, env};

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
        let mut venv = Virtualenv::try_from(parent_dir.join(venv_name).as_ref())?;

        let current_dir = env::current_dir()?;
        let current_dir_str = current_dir.to_string_lossy();
        let project = matches.value_of("project").unwrap_or(&current_dir_str);

        venv.update_config("project", project);
        venv.write_config()?;

        Ok(CommandResult::new().output(Box::new("Successfully linked the project!")))
    }
}
