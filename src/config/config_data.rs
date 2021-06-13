use clap::ArgMatches;
use directories::BaseDirs;
use std::path::PathBuf;

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
    pub fn merge(configs: Vec<Self>) -> Self {
        let mut merged = Config::new();

        for config in configs.into_iter() {
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
