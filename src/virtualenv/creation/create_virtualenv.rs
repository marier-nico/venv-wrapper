use std::path::Path;

use eyre::{eyre, Result};
use log::info;

use crate::virtualenv::{
    creation::create_venv_directories::create_venv_directories, virtualenv_info::Virtualenv,
};

use super::install_pip::install_pip;

pub fn create_virtualenv(venv: &Virtualenv, source_interpreter: &Path) -> Result<()> {
    if venv.path().exists() {
        return Err(eyre!("The path '{}' already exists", venv.path().to_string_lossy()));
    }

    info!("Creating venv directories");
    create_venv_directories(&venv)?;

    info!("Adding python to the venv");
    venv.add_python(source_interpreter)?;

    info!("Writing venv configuration");
    venv.write_config()?;

    info!("Installing pip");
    install_pip(&venv)?;

    info!("Virtualenv created 🚀");
    Ok(())
}
