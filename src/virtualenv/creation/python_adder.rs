use std::path::Path;

use eyre::{Context, Result};

use crate::virtualenv::{creation::link_path::link_path, virtualenv_info::Virtualenv};

#[cfg(target_family = "unix")]
pub fn add_python(venv: &Virtualenv, interpreter: &Path) -> Result<()> {
    for name in venv.python_version.executable_names() {
        link_path(interpreter, &venv.bin_path().join(name))
            .wrap_err("Could not add python executable to venv")?;
    }

    Ok(())
}
