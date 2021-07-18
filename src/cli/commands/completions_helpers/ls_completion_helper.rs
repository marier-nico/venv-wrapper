use clap::ArgMatches;
use eyre::Result;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    utils::intersperse,
    virtualenv::virtualenv_info::Virtualenv,
};

pub struct LsCompletionHelper;
impl Command for LsCompletionHelper {
    fn run(config: &Config, _matches: &ArgMatches) -> Result<CommandResult> {
        let parent_dir = config.venv_root.as_ref().unwrap();

        let venvs = Virtualenv::find_all_in_path(parent_dir);
        let names = venvs.iter().map(|v| &v.name);

        Ok(CommandResult::new().output(Box::new(intersperse(names, ' '))))
    }
}
