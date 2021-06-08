use std::{path::Path, process::Command};

use eyre::{Context, Result};

#[cfg(target_family = "unix")]
pub fn install_pip(venv_path: &Path) -> Result<()> {
    Command::new(venv_path.join("bin/python"))
        .arg("-Im")
        .arg("ensurepip")
        .arg("--upgrade")
        .arg("--default-pip")
        .output()
        .wrap_err("Could not install pip in the venv")?;

    Ok(())
}
