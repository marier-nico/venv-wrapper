use super::virtualenv_info::Virtualenv;
use eyre::{Context, Result};
use log::info;

pub fn delete_virtualenv(venv: &Virtualenv) -> Result<()> {
    std::fs::remove_dir_all(venv.path()).wrap_err("Could not delete the virtualenv")?;

    info!("Successfully removed the virtualenv '{}' 🗑️", venv.name);
    Ok(())
}
