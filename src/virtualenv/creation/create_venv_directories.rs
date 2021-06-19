use std::fs;

use crate::virtualenv::virtualenv_info::Virtualenv;
use eyre::{Context, Result};

use super::link_path::link_path;

pub fn create_venv_directories(venv: &Virtualenv) -> Result<()> {
    let base_dirs = [venv.bin_path(), venv.include_path(), venv.site_packages_path()];
    for dir in base_dirs.iter() {
        fs::create_dir_all(dir)
            .wrap_err(format!("Could not create venv directory {}", dir.to_string_lossy()))?;
    }

    if let Some(lib64_path) = venv.lib64_path() {
        link_path(&venv.lib_path(), &lib64_path).wrap_err("Could not link lib64 to lib")?;
    }

    Ok(())
}
