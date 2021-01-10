use ansi_term::Colour::Green;
use eyre::{Context, Result};

use super::project_settings::ProjectUnlinkSettings;

pub fn unlink(settings: &ProjectUnlinkSettings) -> Result<()> {
    let venv_path = &settings.venvs_dir.join(&settings.venv_name);
    let venv_project_file_path = venv_path.join("project_dir");

    std::fs::remove_file(&venv_project_file_path)
        .context("Could not remove the project association")?;

    println!(
        "\n  {} Successfully unlinked the project from the virtualenv `{}`.",
        Green.paint("✔"),
        &settings.venv_name,
    );

    Ok(())
}
