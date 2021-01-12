use std::{convert::TryFrom, env};

use crate::settings::{GlobalSettings, Virtualenv};

use eyre::{Context, Result};

pub struct ProjectLinkSettings {
    pub venv: Virtualenv,
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

        let virtualenv = Virtualenv::from_existing(
            sub_matches.value_of("venv_name").unwrap(),
            &settings.config.venvs_dir,
        )?;

        let current_dir = env::current_dir().context("Could not access the current directory")?;
        let current_dir_str = current_dir
            .to_str()
            .ok_or_else(|| eyre!("The path to the current directory contains inavlid UTF-8"))?;
        let project_dir = settings.args.value_of("project_dir").unwrap_or(&current_dir_str);

        Ok(ProjectLinkSettings {
            venv: virtualenv,
            project_dir: project_dir.to_owned(),
        })
    }
}

pub struct ProjectUnlinkSettings {
    pub venv: Virtualenv,
}

impl TryFrom<&GlobalSettings<'_>> for ProjectUnlinkSettings {
    type Error = eyre::Report;

    fn try_from(settings: &GlobalSettings) -> Result<Self> {
        let sub_matches = settings
            .args
            .subcommand_matches("project")
            .unwrap()
            .subcommand_matches("unlink")
            .unwrap();

        let virtualenv = Virtualenv::from_existing(
            sub_matches.value_of("venv_name").unwrap(),
            &settings.config.venvs_dir,
        )?;

        Ok(ProjectUnlinkSettings {
            venv: virtualenv,
        })
    }
}
