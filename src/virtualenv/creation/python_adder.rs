use std::path::Path;

use eyre::{Context, Result};

use super::{file_adder::FileAdder, python_version::PythonVersion};

#[cfg(target_family = "unix")]
pub fn add_python(
    file_adder: &dyn FileAdder,
    interpreter: &Path,
    python_version: &PythonVersion,
    venv_path: &Path,
) -> Result<()> {
    for name in python_version.executable_names() {
        file_adder
            .add_file(interpreter, &venv_path.join(format!("bin/{}", name)))
            .wrap_err("Could not add python executable to venv")?;
    }

    Ok(())
}
