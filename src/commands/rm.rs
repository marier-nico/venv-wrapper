use clap::ArgMatches;
use eyre::{Context, Result};
use std::path::PathBuf;
use std::{env, path::Path};

use crate::settings::Settings;

pub fn rm(settings: &Settings, args: &ArgMatches) -> Result<()> {
    let venv_name = args.value_of("venv_name").unwrap();

    if let Ok(var) = env::var("VIRTUAL_ENV") {
        let active_venv_path = PathBuf::from(var);
        let active_venv_name = active_venv_path.file_name().unwrap().to_str().unwrap();
        if venv_name == active_venv_name {
            return Err(eyre!("Cannot delete the currently active virtualenv"));
        }
    }

    let venv_path = Path::new(&settings.venvs_dir).join(venv_name);
    if venv_path.join("bin/activate").is_file() {
        std::fs::remove_dir_all(venv_path)
            .context(format!("Could not delete the virtualenv `{}`", venv_name))?;
    } else {
        return Err(eyre!(
            "Cannot delete the directory `{}`, not a virtualenv",
            venv_path.to_string_lossy()
        ));
    }

    Ok(())
}
