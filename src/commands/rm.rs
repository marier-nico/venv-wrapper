use eyre::{Context, Result};
use std::path::PathBuf;
use std::env;

use crate::settings::RmSettings;

pub fn rm(settings: &RmSettings) -> Result<()> {
    if let Ok(var) = env::var("VIRTUAL_ENV") {
        let active_venv_path = PathBuf::from(var);
        let active_venv_name = active_venv_path.file_name().unwrap().to_str().unwrap();
        if settings.venv_name == active_venv_name {
            return Err(eyre!("Cannot delete the currently active virtualenv"));
        }
    }

    let venv_path = &settings.venvs_dir.join(&settings.venv_name);
    if venv_path.join("bin/activate").is_file() {
        std::fs::remove_dir_all(venv_path)
            .context(format!("Could not delete the virtualenv `{}`", &settings.venv_name))?;
    } else {
        return Err(eyre!(
            "Cannot delete the directory `{}`, not a virtualenv",
            venv_path.to_string_lossy()
        ));
    }

    Ok(())
}
