use std::{env, fs, path::PathBuf};

use clap::ArgMatches;
use eyre::{Context, Result};
use regex::Regex;

use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    utils::intersperse,
};

pub struct AvailablePythonsHelper;
impl Command for AvailablePythonsHelper {
    fn run(_config: &Config, _matches: &ArgMatches) -> Result<CommandResult> {
        let re = r"^python\d?(?:\.\d+(?:\.\d+)?)?$";

        let mut python_interpreters: Vec<String> =
            which_re(re)?.map(|p| p.file_name().unwrap().to_string_lossy().to_string()).collect();
        python_interpreters.sort();
        python_interpreters.dedup();

        Ok(CommandResult::new().output(Box::new(intersperse(python_interpreters.iter(), ' '))))
    }
}

fn which_re(re: &str) -> Result<impl Iterator<Item = PathBuf>> {
    let path = env::var("PATH").wrap_err("No PATH environment variable defined")?;
    let re = Regex::new(re).wrap_err("Invalid regex supplied")?;

    let valid_paths = path.split(':').map(PathBuf::from).filter(|p| p.exists() && p.is_dir());

    let mut results = Vec::new();
    for bin_dir in valid_paths {
        let entries = match fs::read_dir(bin_dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if re.is_match(&entry.file_name().to_string_lossy()) {
                results.push(entry.path());
            }
        }
    }

    Ok(results.into_iter())
}
