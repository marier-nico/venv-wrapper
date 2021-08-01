use clap::ArgMatches;
use eyre::Result;
use regex::Regex;
use which::which_re;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    utils::intersperse,
};

pub struct AvailablePythonsHelper;
impl Command for AvailablePythonsHelper {
    fn run(_config: &Config, _matches: &ArgMatches) -> Result<CommandResult> {
        let re = Regex::new(r"^python\d?(?:\.\d+(?:\.\d+)?)?$")?;

        let mut python_interpreters: Vec<String> =
            which_re(re)?.map(|p| p.file_name().unwrap().to_string_lossy().to_string()).collect();
        python_interpreters.sort();
        python_interpreters.dedup();

        Ok(CommandResult::new().output(Box::new(intersperse(python_interpreters.iter(), ' '))))
    }
}
