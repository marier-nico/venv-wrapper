use std::path::PathBuf;

use super::creation::python_version::PythonVersion;

pub struct Virtualenv {
    pub name: String,
    pub parent_dir: PathBuf,
    pub python_version: PythonVersion,
}
