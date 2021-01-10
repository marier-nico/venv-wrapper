use std::path::{Path, PathBuf};

use clap::ArgMatches;
use config::Config;
use directories::UserDirs;
use eyre::{Context, Result};

#[derive(Debug)]
pub struct GlobalSettings<'a> {
    pub config: ConfigSettings,
    pub args: &'a ArgMatches<'a>,
    pub eval_file: &'a Path,
}

impl<'a> GlobalSettings<'a> {
    pub fn new(config: ConfigSettings, args: &'a ArgMatches, eval_file: &'a Path) -> Self {
        GlobalSettings {
            config,
            args,
            eval_file,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigSettings {
    pub venvs_dir: String,
}

impl ConfigSettings {
    pub fn new(matches: &ArgMatches) -> Result<Self> {
        let mut settings = Config::new();
        set_defaults(&mut settings)?;

        // It's fine to unwrap here because if no home directory path is found, an Err() will be
        // returned when setting the default value just above.
        let dir = directories::ProjectDirs::from("com", "venv-wrapper", "venv-wrapper").unwrap();
        let mut config_file = dir.preference_dir().to_owned();
        config_file.push("config.toml");

        settings.merge(config::File::from(config_file)).ok();
        settings.merge(config::Environment::with_prefix("VENVWRAPPER")).ok();
        let mut settings =
            settings.try_into().context("The provided configuration is in an invalid format")?;
        set_cli_overrides(&mut settings, matches)?;

        Ok(settings)
    }
}

fn set_defaults(settings: &mut Config) -> Result<()> {
    let dirs = UserDirs::new()
        .ok_or_else(|| eyre!("Could not determine the path to your home directory"))?;

    let default_venvs_dir = dirs.home_dir().join(".virtualenvs");
    settings
        .set_default(
            "venvs_dir",
            default_venvs_dir
                .to_str()
                .ok_or_else(|| eyre!("Path to home directory is not valid UTF-8"))?,
        )
        .ok();

    Ok(())
}

fn set_cli_overrides(settings: &mut ConfigSettings, matches: &ArgMatches) -> Result<()> {
    if let Some(venvs_dir) = matches.value_of("venvs_dir") {
        settings.venvs_dir = venvs_dir.to_string();
    }

    Ok(())
}

pub struct InitSettings {
    pub eval_file: PathBuf,
}

impl From<GlobalSettings<'_>> for InitSettings {
    fn from(settings: GlobalSettings) -> Self {
        InitSettings {
            eval_file: settings.eval_file.to_owned(),
        }
    }
}

pub struct LsSettings {
    pub venvs_dir: PathBuf,
}

impl From<GlobalSettings<'_>> for LsSettings {
    fn from(settings: GlobalSettings) -> Self {
        LsSettings {
            venvs_dir: PathBuf::from(&settings.config.venvs_dir),
        }
    }
}

pub struct NewSettings {
    pub venvs_dir: PathBuf,
    pub venv_name: String,
    pub python_executable: String,
    pub eval_file: PathBuf,
}

impl From<GlobalSettings<'_>> for NewSettings {
    fn from(settings: GlobalSettings) -> Self {
        NewSettings {
            venvs_dir: PathBuf::from(&settings.config.venvs_dir),
            venv_name: settings.args.value_of("venv_name").unwrap().to_owned(),
            python_executable: settings.args.value_of("python_executable").unwrap().to_owned(),
            eval_file: settings.eval_file.to_owned(),
        }
    }
}

pub struct ActivateSettings {
    pub venvs_dir: PathBuf,
    pub venv_name: String,
    pub eval_file: PathBuf,
}

impl From<GlobalSettings<'_>> for ActivateSettings {
    fn from(settings: GlobalSettings) -> Self {
        ActivateSettings {
            venvs_dir: PathBuf::from(&settings.config.venvs_dir),
            venv_name: settings.args.value_of("venv_name").unwrap().to_owned(),
            eval_file: settings.eval_file.to_owned(),
        }
    }
}

pub struct DeactivateSettings {
    pub eval_file: PathBuf,
}

impl From<GlobalSettings<'_>> for DeactivateSettings {
    fn from(settings: GlobalSettings) -> Self {
        DeactivateSettings {
            eval_file: PathBuf::from(settings.eval_file),
        }
    }
}

pub struct RmSettings {
    pub venvs_dir: PathBuf,
    pub venv_name: String,
}

impl From<GlobalSettings<'_>> for RmSettings {
    fn from(settings: GlobalSettings) -> Self {
        RmSettings {
            venvs_dir: PathBuf::from(&settings.config.venvs_dir),
            venv_name: settings.args.value_of("venv_name").unwrap().to_owned(),
        }
    }
}

pub struct UseSettings {
    pub venvs_dir: PathBuf,
    pub venv_name: String,
    pub eval_file: PathBuf,
}

impl From<GlobalSettings<'_>> for UseSettings {
    fn from(settings: GlobalSettings) -> Self {
        UseSettings {
            venvs_dir: PathBuf::from(&settings.config.venvs_dir),
            venv_name: settings.args.value_of("venv_name").unwrap().to_owned(),
            eval_file: settings.eval_file.to_owned(),
        }
    }
}
