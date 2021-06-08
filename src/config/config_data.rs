use clap::ArgMatches;
use directories::BaseDirs;
use eyre::{eyre, Error as EyreError, Result};
use std::{convert::TryFrom, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub interpreter: Option<String>,
    pub venv_name: Option<String>,
    pub venv_root: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            interpreter: None,
            venv_name: None,
            venv_root: None,
        }
    }
}

impl Config {
    pub fn merge(configs: Vec<Self>) -> Self {
        let mut merged = Config::new();

        for config in configs.into_iter() {
            if let Some(interpreter) = config.interpreter {
                merged.interpreter = Some(interpreter);
            }
            if let Some(venv_name) = config.venv_name {
                merged.venv_name = Some(venv_name);
            }
            if let Some(venv_root) = config.venv_root {
                merged.venv_root = Some(venv_root);
            }
        }

        merged
    }
}

impl Default for Config {
    fn default() -> Self {
        let base_dirs = BaseDirs::new().expect("Could not find base user directories");

        Config {
            interpreter: None,
            venv_name: None,
            venv_root: Some(base_dirs.home_dir().join(".virtualenvs")),
        }
    }
}

impl TryFrom<&ArgMatches<'_>> for Config {
    type Error = EyreError;

    fn try_from(matches: &ArgMatches) -> Result<Self> {
        let mut config = Config::new();

        add_root_cli_values(&mut config, &matches);

        match matches.subcommand() {
            ("new", Some(sub)) => add_new_command_cli_values(&mut config, sub),
            _ => return Err(eyre!("Unhandled command")),
        }

        Ok(config)
    }
}

fn add_root_cli_values(config: &mut Config, matches: &ArgMatches) {
    if let Some(value) = matches.value_of("venv_root") {
        config.venv_root = Some(PathBuf::from(value));
    }
}

fn add_new_command_cli_values(config: &mut Config, sub_matches: &ArgMatches) {
    config.interpreter = Some(sub_matches.value_of("python_executable").unwrap().to_string());
    config.venv_name = Some(sub_matches.value_of("name").unwrap().to_string());
}
