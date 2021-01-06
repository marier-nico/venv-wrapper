use ansi_term::Colour::Green;
use clap::ArgMatches;
use eyre::{Context, Result};
use std::{env, path::PathBuf};

use crate::settings::Settings;

pub fn link(settings: &Settings, args: &ArgMatches) -> Result<()> {
    let current_dir = env::current_dir().context("Could not access the current directory")?;
    let current_dir_str = current_dir
        .to_str()
        .ok_or_else(|| eyre!("The path to the current directory contains inavlid UTF-8"))?;
    let project_dir = args.value_of("project_dir").unwrap_or(current_dir_str);

    let venv_name = args.value_of("venv_name").unwrap();
    let venv_path = PathBuf::from(&settings.venvs_dir).join(venv_name);
    let venv_project_file_path = venv_path.join("project_dir");

    std::fs::write(&venv_project_file_path, project_dir)
        .context("Could not link the specified virtualenv to a project")?;

    println!(
        "\n  {} Successfully linked the virtualenv `{}` to `{}`.",
        Green.paint("✔"),
        venv_name,
        project_dir
    );

    Ok(())
}
