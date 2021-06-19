use std::path::PathBuf;

use super::creation::python_version::PythonVersion;

pub struct Virtualenv {
    pub name: String,
    pub parent_dir: PathBuf,
    pub python_version: PythonVersion,
}

impl Virtualenv {
    pub fn path(&self) -> PathBuf {
        let name: &str = self.name.as_ref();
        self.parent_dir.join(name)
    }

    pub fn bin_path(&self) -> PathBuf {
        if cfg!(target_family = "unix") {
            self.path().join("bin")
        } else {
            self.path().join("Scripts")
        }
    }

    pub fn python_path(&self) -> PathBuf {
        if cfg!(target_family = "unix") {
            self.bin_path().join("python")
        } else {
            self.bin_path().join("python.exe")
        }
    }

    pub fn include_path(&self) -> PathBuf {
        if cfg!(target_family = "unix") {
            self.path().join("include")
        } else {
            self.path().join("Include")
        }
    }

    pub fn lib_path(&self) -> PathBuf {
        if cfg!(target_family = "unix") {
            self.path().join("lib")
        } else {
            self.path().join("Lib")
        }
    }

    pub fn lib64_path(&self) -> Option<PathBuf> {
        if cfg!(any(target_os = "windows", target_os = "darwin")) {
            None
        } else {
            Some(self.path().join("lib64"))
        }
    }

    pub fn site_packages_path(&self) -> PathBuf {
        let lib_path = self.lib_path();
        if cfg!(target_family = "unix") {
            lib_path.join(self.python_version.minor_prefix()).join("site-packages")
        } else {
            lib_path.join("site-packages")
        }
    }
}
