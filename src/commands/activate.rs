use std::path::Path;

use crate::settings::ActivateSettings;
use eyre::{Context, Result};

pub fn activate_cli(settings: &ActivateSettings) -> Result<()> {
    activate(&settings.venvs_dir, &settings.venv_name, &settings.eval_file)?;

    Ok(())
}

pub fn activate(venvs_dir: &Path, venv_name: &str, eval_file: &Path) -> Result<()> {
    let activate_script_path = venvs_dir.join(venv_name).join("bin/activate");
    let activate_script_path_str = &activate_script_path.to_str().ok_or_else(|| {
        eyre!(
            "The path to the activation script for the virtualenv {} contained invalid UTF-8",
            venv_name
        )
    })?;

    std::fs::write(eval_file, format!("source {}", activate_script_path_str))
        .context("Unable to write to the eval file (cannot modify shell state)")?;

    Ok(())
}
