use std::process::Command;

use eyre::{Context, Result};

use crate::virtualenv::virtualenv_info::Virtualenv;

#[cfg(target_family = "unix")]
pub fn install_pip(venv: &Virtualenv) -> Result<()> {
    Command::new(venv.python_path())
        .arg("-Im")
        .arg("ensurepip")
        .arg("--upgrade")
        .arg("--default-pip")
        .output()
        .wrap_err("Could not install pip in the venv")?;

    Ok(())
}
