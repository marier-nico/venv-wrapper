use eyre::{eyre, Context, Result};
use std::{
    convert::TryFrom,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use crate::virtualenv::creation::python_version::PythonVersion;

use super::virtualenv_info::Virtualenv;

pub fn find_in_path(name: &str, parent_dir: &Path) -> Result<Virtualenv> {
    let venv_config_file = parent_dir.join(format!("{}/pyvenv.cfg", name));
    if venv_config_file.exists() {
        let file =
            File::open(venv_config_file).wrap_err("Could not open the venv's config file")?;

        for line in BufReader::new(file).lines().flatten() {
            let (key, value) = match line.split_once('=') {
                Some(cfg) => cfg,
                None => continue,
            };

            if key.trim() == "version" || key.trim() == "version_info" {
                let version = PythonVersion::try_from(value)
                    .wrap_err("Could not determine the python version of the venv")?;

                return Ok(Virtualenv {
                    name: name.to_string(),
                    parent_dir: parent_dir.to_owned(),
                    python_version: version,
                });
            }
        }
    }

    Err(eyre!("Could not find a virtual environment named '{}' in {:?}", name, parent_dir))
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
