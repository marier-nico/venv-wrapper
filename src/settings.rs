use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

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
    pub venvs_dir: PathBuf,
}

impl ConfigSettings {
    pub fn new(matches: &ArgMatches) -> Result<Self> {
        let mut settings = Config::new();
        set_defaults(&mut settings)?;

        // It's fine to unwrap here because if no home directory path is found, an Err() will be
        // returned when setting the default value just above.
        let dir = directories::ProjectDirs::from("com", "venv-wrapper", "venv-wrapper").unwrap();
        let config_file = dir.preference_dir().join("config.toml");

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
        settings.venvs_dir = PathBuf::from(venvs_dir);
    }

    Ok(())
}

pub struct Virtualenv {
    pub name: String,
    pub path: PathBuf,
}

impl Virtualenv {
    /// Create a new Virtualenv struct from an existing virtualenv
    ///
    /// Use this function to create a new Virtualenv struct based on the base directory for all
    /// virtualenvs and the individual virtualenv name. This function may fail if the virtualenv
    /// doesn't exist.
    pub fn from_existing(name: &str, venvs_dir: &Path) -> Result<Self> {
        let full_path = venvs_dir.join(name);

        if !full_path.exists() {
            return Err(eyre!("The virtualenv `{}` does not exist", name));
        }
        if !full_path.join("bin/activate").exists() {
            return Err(eyre!(
                "The directory `{}` exists, but it doesn't look like a virtualenv",
                full_path.to_string_lossy()
            ));
        }

        Ok(Virtualenv {
            name: name.to_owned(),
            path: full_path,
        })
    }

    /// Create a new Virtualenv struct
    ///
    /// Use this function to create a new Virtualenv struct without checking that the environment
    /// exists beforehand. If the virtualenv needs to exist, use the `from_existing` function.
    pub fn new(name: &str, venvs_dir: &Path) -> Self {
        let full_path = venvs_dir.join(name);

        Virtualenv {
            name: name.to_owned(),
            path: full_path,
        }
    }

    pub fn path_str(&self) -> Result<&str> {
        self.path.to_str().ok_or_else(|| eyre!("Path to virtualenv contained invalid UTF-8"))
    }
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
            venvs_dir: settings.config.venvs_dir,
        }
    }
}

pub struct NewSettings {
    pub venv: Virtualenv,
    pub python_executable: String,
    pub eval_file: PathBuf,
}

impl TryFrom<GlobalSettings<'_>> for NewSettings {
    type Error = eyre::Report;

    fn try_from(settings: GlobalSettings) -> Result<Self> {
        let sub_matches = settings.args.subcommand_matches("new").unwrap();

        let virtualenv =
            Virtualenv::new(sub_matches.value_of("venv_name").unwrap(), &settings.config.venvs_dir);

        Ok(NewSettings {
            venv: virtualenv,
            python_executable: sub_matches.value_of("python_executable").unwrap().to_owned(),
            eval_file: settings.eval_file.to_owned(),
        })
    }
}

pub struct ActivateSettings {
    pub venv: Virtualenv,
    pub eval_file: PathBuf,
}

impl TryFrom<GlobalSettings<'_>> for ActivateSettings {
    type Error = eyre::Report;

    fn try_from(settings: GlobalSettings) -> Result<Self> {
        let sub_matches = settings.args.subcommand_matches("activate").unwrap();

        let virtualenv = Virtualenv::from_existing(
            sub_matches.value_of("venv_name").unwrap(),
            &settings.config.venvs_dir,
        )?;

        Ok(ActivateSettings {
            venv: virtualenv,
            eval_file: settings.eval_file.to_owned(),
        })
    }
}

pub struct DeactivateSettings {
    pub eval_file: PathBuf,
}

impl From<GlobalSettings<'_>> for DeactivateSettings {
    fn from(settings: GlobalSettings) -> Self {
        DeactivateSettings {
            eval_file: settings.eval_file.to_owned(),
        }
    }
}

pub struct RmSettings {
    pub venv: Virtualenv,
}

impl TryFrom<GlobalSettings<'_>> for RmSettings {
    type Error = eyre::Report;

    fn try_from(settings: GlobalSettings) -> Result<Self> {
        let sub_matches = settings.args.subcommand_matches("rm").unwrap();

        let virtualenv = Virtualenv::from_existing(
            sub_matches.value_of("venv_name").unwrap(),
            &settings.config.venvs_dir,
        )?;

        Ok(RmSettings {
            venv: virtualenv,
        })
    }
}

pub struct UseSettings {
    pub venv: Virtualenv,
    pub eval_file: PathBuf,
}

impl TryFrom<GlobalSettings<'_>> for UseSettings {
    type Error = eyre::Report;

    fn try_from(settings: GlobalSettings) -> Result<Self> {
        let sub_matches = settings.args.subcommand_matches("use").unwrap();

        let virtualenv = Virtualenv::from_existing(
            sub_matches.value_of("venv_name").unwrap(),
            &settings.config.venvs_dir,
        )?;

        Ok(UseSettings {
            venv: virtualenv,
            eval_file: settings.eval_file.to_owned(),
        })
    }
}
