use std::path::{Path, PathBuf};

use clap::ArgMatches;

use crate::settings::Settings;
use eyre::{Context, Result};

pub fn activate_cli(settings: &Settings, args: &ArgMatches, eval_file: &Path) -> Result<()> {
    let venvs_dir = PathBuf::from(&settings.venvs_dir);
    let venv_name = args.value_of("venv_name").unwrap(); // `venv_name` is a required arg

    activate(&venvs_dir, venv_name, eval_file)?;

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
