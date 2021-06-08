use eyre::{eyre, Result};

use crate::config::config_data::Config;

use super::{
    file_adder::get_file_adder, install_pip::install_pip,
    interpreter_locator::get_interpreter_locator, python_adder::add_python,
    python_version::PythonVersion, venv_directory_creator::get_directory_creator,
    write_venv_config::write_venv_config,
};

pub fn create_virtualenv(config: &Config) -> Result<()> {
    let interpreter_info = config.interpreter.as_ref().unwrap();
    let interpreter_locator = get_interpreter_locator(interpreter_info);
    let interpreter = interpreter_locator.locate_interpreter(interpreter_info);
    let interpreter = match interpreter {
        Some(interpreter) => interpreter,
        None => return Err(eyre!("Could not find the specified python interpreter")),
    };

    let python_version = PythonVersion::try_from_interpreter(&interpreter)?;
    let directory_creator = get_directory_creator()?;
    let file_adder = get_file_adder();

    let venv_root = config.venv_root.as_ref().unwrap();
    let venv_path = venv_root.join(config.venv_name.as_ref().unwrap());

    directory_creator.create_directories(&*file_adder, &python_version, &venv_path)?;
    add_python(&*file_adder, &interpreter, &python_version, &venv_path)?;
    write_venv_config(&interpreter, &python_version, &venv_path)?;
    install_pip(&venv_path)?;

    Ok(())
}
