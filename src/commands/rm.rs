use ansi_term::Colour::Green;
use eyre::{Context, Result};
use std::env;
use std::path::PathBuf;

use crate::settings::RmSettings;

pub fn rm(settings: &RmSettings) -> Result<()> {
    if let Ok(var) = env::var("VIRTUAL_ENV") {
        let active_venv_path = PathBuf::from(var);
        let active_venv_name = active_venv_path.file_name().unwrap().to_str().unwrap();
        if settings.venv.name == active_venv_name {
            return Err(eyre!("Cannot delete the currently active virtualenv"));
        }
    }

    if settings.venv.path.join("bin/activate").is_file() {
        std::fs::remove_dir_all(&settings.venv.path)
            .context(format!("Could not delete the virtualenv `{}`", &settings.venv.name))?;
    } else {
        return Err(eyre!(
            "Cannot delete the directory `{}`, not a virtualenv",
            settings.venv.path_str()?
        ));
    }

    println!(
        "\n {}  Successfully removed the virtualenv `{}`.",
        Green.paint("✔"),
        &settings.venv.name
    );

    Ok(())
}
