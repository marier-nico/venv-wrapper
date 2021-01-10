use ansi_term::Colour::Green;
use eyre::{Context, Result};

use super::project_settings::ProjectLinkSettings;

pub fn link(settings: &ProjectLinkSettings) -> Result<()> {
    let venv_path = &settings.venvs_dir.join(&settings.venv_name);
    let venv_project_file_path = venv_path.join("project_dir");

    std::fs::write(&venv_project_file_path, &settings.project_dir)
        .context("Could not link the specified virtualenv to a project")?;

    println!(
        "\n  {} Successfully linked the virtualenv `{}` to `{}`.",
        Green.paint("✔"),
        &settings.venv_name,
        &settings.project_dir
    );

    Ok(())
}
