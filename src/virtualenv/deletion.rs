use super::virtualenv_info::Virtualenv;
use eyre::{Context, Result};
use log::info;

pub fn delete_virtualenv(venv: &Virtualenv) -> Result<()> {
    let venv_path = venv.parent_dir.join(&venv.name);
    std::fs::remove_dir_all(venv_path).wrap_err("Could not delete the virtualenv")?;

    info!("Successfully removed the virtualenv '{}' 🗑️", venv.name);
    Ok(())
}
