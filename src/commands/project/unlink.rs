use ansi_term::Colour::Green;
use eyre::{Context, Result};

use super::project_settings::ProjectUnlinkSettings;

pub fn unlink(settings: &ProjectUnlinkSettings) -> Result<()> {
    let venv_project_file_path = settings.venv.path.join("project_dir");

    std::fs::remove_file(&venv_project_file_path)
        .context("Could not remove the project association")?;

    println!(
        "\n  {} Successfully unlinked the project from the virtualenv `{}`.",
        Green.paint("✔"),
        &settings.venv.name,
    );

    Ok(())
}
