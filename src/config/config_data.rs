use clap::ArgMatches;
use directories::BaseDirs;
use ini::Ini;
use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub venv_root: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            venv_root: None,
        }
    }
}

impl Config {
    pub fn merge(configs: &[Self]) -> Self {
        let mut merged = Config::new();

        for config in configs.iter() {
            if let Some(venv_root) = config.venv_root.as_ref() {
                merged.venv_root = Some(venv_root.to_owned());
            }
        }

        merged
    }

    pub fn from_file() -> Self {
        let mut config = Config::new();

        if let Some(dirs) = BaseDirs::new() {
            let config_file = dirs.config_dir().join("venv-wrapper/config.ini");
            if let Ok(config_file) = Ini::load_from_file(config_file) {
                config.venv_root =
                    config_file.general_section().get("venv_root").map(PathBuf::from);
            }
        }

        config
    }

    pub fn from_env() -> Self {
        let mut config = Config::new();
        config.venv_root = env::var("VENVWRAPPER_VENV_ROOT").ok().map(PathBuf::from);

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        let base_dirs = BaseDirs::new().expect("Could not find base user directories");

        Config {
            venv_root: Some(base_dirs.home_dir().join(".virtualenvs")),
        }
    }
}

impl From<&ArgMatches<'_>> for Config {
    fn from(matches: &ArgMatches) -> Self {
        let mut config = Config::new();

        if let Some(value) = matches.value_of("venv_root") {
            config.venv_root = Some(PathBuf::from(value));
        }

        config
    }
}
