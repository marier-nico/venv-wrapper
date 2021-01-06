use ansi_term::Colour::Green;
use clap::ArgMatches;
use eyre::{Context, Result};
use std::path::PathBuf;

use crate::settings::Settings;

pub fn unlink(settings: &Settings, args: &ArgMatches) -> Result<()> {
    let venv_name = args.value_of("venv_name").unwrap();
    let venv_path = PathBuf::from(&settings.venvs_dir).join(venv_name);
    let venv_project_file_path = venv_path.join("project_dir");

    std::fs::remove_file(&venv_project_file_path)
        .context("Could not remove the project association")?;

    println!(
        "\n  {} Successfully unlinked the project from the virtualenv `{}`.",
        Green.paint("✔"),
        venv_name,
    );

    Ok(())
}
