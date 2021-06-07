use eyre::Result;
use std::path::Path;

use super::{
    file_adder::FileAdder, install_pip::install_pip, python_adder::PythonAdder,
    python_version::PythonVersion, venv_directory_creator::VenvDirectoryCreator,
    write_venv_config::write_venv_config,
};

pub fn create_virtualenv(
    directory_creator: &dyn VenvDirectoryCreator,
    file_adder: &dyn FileAdder,
    interpreter: &Path,
    python_adder: &dyn PythonAdder,
    python_version: &PythonVersion,
    venv_root: &Path,
) -> Result<()> {
    directory_creator.create_directories(&*file_adder, &python_version, venv_root)?;
    python_adder.add_python(&*file_adder, interpreter, &python_version, venv_root)?;
    write_venv_config(interpreter, &python_version, venv_root)?;
    install_pip(venv_root)?;

    Ok(())
}
