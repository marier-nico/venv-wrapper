use std::{convert::TryFrom, env, path::PathBuf};

use crate::settings::GlobalSettings;

use eyre::{Context, Result};

pub struct ProjectLinkSettings {
    pub venvs_dir: PathBuf,
    pub venv_name: String,
    pub project_dir: String,
}

impl TryFrom<&GlobalSettings<'_>> for ProjectLinkSettings {
    type Error = eyre::Report;

    fn try_from(settings: &GlobalSettings) -> Result<ProjectLinkSettings> {
        let sub_matches = settings
            .args
            .subcommand_matches("project")
            .unwrap()
            .subcommand_matches("link")
            .unwrap();

        let current_dir = env::current_dir().context("Could not access the current directory")?;
        let current_dir_str = current_dir
            .to_str()
            .ok_or_else(|| eyre!("The path to the current directory contains inavlid UTF-8"))?;
        let project_dir = settings.args.value_of("project_dir").unwrap_or(&current_dir_str);

        Ok(ProjectLinkSettings {
            venvs_dir: PathBuf::from(&settings.config.venvs_dir),
            venv_name: sub_matches.value_of("venv_name").unwrap().to_owned(),
            project_dir: project_dir.to_owned(),
        })
    }
}

pub struct ProjectUnlinkSettings {
    pub venvs_dir: PathBuf,
    pub venv_name: String,
}

impl From<&GlobalSettings<'_>> for ProjectUnlinkSettings {
    fn from(settings: &GlobalSettings) -> Self {
        let sub_matches = settings
            .args
            .subcommand_matches("project")
            .unwrap()
            .subcommand_matches("unlink")
            .unwrap();

        ProjectUnlinkSettings {
            venvs_dir: PathBuf::from(&settings.config.venvs_dir),
            venv_name: sub_matches.value_of("venv_name").unwrap().to_owned(),
        }
    }
}
