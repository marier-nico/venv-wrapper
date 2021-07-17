use eyre::{eyre, Context, Result};
use ini::Ini;
use std::{convert::TryFrom, fs, path::Path};

use crate::virtualenv::creation::python_version::PythonVersion;

use super::virtualenv_info::Virtualenv;

pub fn find_in_path(name: &str, parent_dir: &Path) -> Result<Virtualenv> {
    let venv_config_file = parent_dir.join(format!("{}/pyvenv.cfg", name));
    let config =
        Ini::load_from_file(venv_config_file).wrap_err("Could not load the venv config")?;

    let venv_version = config.get_from::<&str>(None, "version");
    let virtualenv_version = config.get_from::<&str>(None, "version_info");

    let version = match (venv_version, virtualenv_version) {
        (Some(v), _) => v,
        (None, Some(v)) => v,
        (None, None) => return Err(eyre!("Python version not present in venv config")),
    };

    let version =
        PythonVersion::try_from(version).wrap_err("Could not parse the python version")?;

    Ok(Virtualenv {
        name: name.to_string(),
        parent_dir: parent_dir.to_owned(),
        python_version: version,
    })
}

pub fn find_all_in_path(parent_dir: &Path) -> Vec<Virtualenv> {
    let mut found_venvs = vec![];

    let dirs = fs::read_dir(parent_dir);
    if dirs.is_err() {
        return found_venvs;
    }

    for entry in dirs.unwrap().flatten() {
        let venv_path = entry.path();
        let venv_name = venv_path.file_name().unwrap().to_string_lossy();
        match find_in_path(&venv_name, parent_dir) {
            Ok(venv) => found_venvs.push(venv),
            Err(_) => continue,
        }
    }

    found_venvs
}
