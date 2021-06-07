use std::{path::Path, process::Command};

use eyre::Result;

pub fn install_pip(venv_root: &Path) -> Result<()> {
    Command::new(venv_root.join("bin/python"))
        .arg("-Im")
        .arg("ensurepip")
        .arg("--upgrade")
        .arg("--default-pip")
        .output()?;
    
    Ok(())
}
