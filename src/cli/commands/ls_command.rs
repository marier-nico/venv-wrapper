use clap::ArgMatches;
use eyre::Result;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    presentation::collection_display::CollectionDisplay,
    virtualenv::discovery::find_all_in_path,
};

pub struct LsCommand;
impl Command for LsCommand {
    fn run(config: &Config, _matches: &ArgMatches) -> Result<CommandResult> {
        let parent_dir = config.venv_root.as_ref().unwrap();

        let venvs = find_all_in_path(parent_dir);
        Ok(CommandResult::new().output(Box::new(CollectionDisplay::new(venvs))))
    }
}
