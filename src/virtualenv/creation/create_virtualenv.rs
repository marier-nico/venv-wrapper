use std::path::Path;

use eyre::Result;
use log::info;

use crate::virtualenv::virtualenv_info::Virtualenv;

use super::{
    file_adder::get_file_adder, install_pip::install_pip, python_adder::add_python,
    venv_directory_creator::get_directory_creator, write_venv_config::write_venv_config,
};

pub fn create_virtualenv(venv: &Virtualenv, source_interpreter: &Path) -> Result<()> {
    let directory_creator = get_directory_creator()?;
    let file_adder = get_file_adder();

    let venv_path = venv.parent_dir.join(&venv.name);

    info!("Creating venv directories");
    directory_creator.create_directories(&*file_adder, &venv.python_version, &venv_path)?;
    info!("Adding python to the venv");
    add_python(&*file_adder, source_interpreter, &venv.python_version, &venv_path)?;
    info!("Writing venv configuration");
    write_venv_config(source_interpreter, &venv.python_version, &venv_path)?;
    info!("Installing pip");
    install_pip(&venv_path)?;

    Ok(())
}
