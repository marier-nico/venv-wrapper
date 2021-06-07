use std::path::Path;

use eyre::{eyre, Context, Result};

use super::{file_adder::FileAdder, python_version::PythonVersion};

pub trait PythonAdder {
    fn add_python(
        &self,
        file_adder: &dyn FileAdder,
        interpreter: &Path,
        python_version: &PythonVersion,
        venv_root: &Path,
    ) -> Result<()>;
}

pub struct UnixPythonAdder {}
impl PythonAdder for UnixPythonAdder {
    fn add_python(
        &self,
        file_adder: &dyn FileAdder,
        interpreter: &Path,
        python_version: &PythonVersion,
        venv_root: &Path,
    ) -> Result<()> {
        for name in python_version.executable_names() {
            file_adder
                .add_file(interpreter, &venv_root.join(format!("bin/{}", name)))
                .wrap_err("Could not add python executable to venv")?;
        }

        Ok(())
    }
}

pub fn get_python_adder() -> Result<Box<dyn PythonAdder>> {
    if cfg!(target_family = "unix") {
        Ok(Box::new(UnixPythonAdder {}))
    } else {
        Err(eyre!("Your OS is not yet supported"))
    }
}
