use std::path::Path;

use eyre::Result;
use log::info;

use crate::virtualenv::{
    creation::create_venv_directories::create_venv_directories, virtualenv_info::Virtualenv,
};

use super::{
    install_pip::install_pip, python_adder::add_python, write_venv_config::write_venv_config,
};

pub fn create_virtualenv(venv: &Virtualenv, source_interpreter: &Path) -> Result<()> {
    info!("Creating venv directories");
    create_venv_directories(&venv)?;

    info!("Adding python to the venv");
    add_python(&venv, source_interpreter)?;

    info!("Writing venv configuration");
    write_venv_config(&venv, source_interpreter)?;

    info!("Installing pip");
    install_pip(&venv)?;

    Ok(())
}
