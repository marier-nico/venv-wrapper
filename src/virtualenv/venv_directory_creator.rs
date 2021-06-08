use std::{fs, path::Path};

use eyre::{eyre, Result, WrapErr};

use super::{file_adder::FileAdder, python_version::PythonVersion};

pub trait VenvDirectoryCreator {
    fn create_directories(
        &self,
        file_adder: &dyn FileAdder,
        python_version: &PythonVersion,
        venv_path: &Path,
    ) -> Result<()>;
}

pub struct LinuxVenvDirectoryCreator {}
impl VenvDirectoryCreator for LinuxVenvDirectoryCreator {
    fn create_directories(
        &self,
        file_adder: &dyn FileAdder,
        python_version: &PythonVersion,
        venv_path: &Path,
    ) -> Result<()> {
        let sub_dirs =
            ["bin", "include", &format!("lib/{}/site-packages", python_version.minor_prefix())];

        for dir in sub_dirs.iter() {
            fs::create_dir_all(venv_path.join(dir))
                .wrap_err(format!("Could not create venv directory {}", dir))?;
        }

        file_adder
            .add_file(&venv_path.join("lib"), &venv_path.join("lib64"))
            .wrap_err("Could not link lib64 to lib")?;

        Ok(())
    }
}

pub fn get_directory_creator() -> Result<Box<dyn VenvDirectoryCreator>> {
    if cfg!(target_os = "linux") {
        Ok(Box::new(LinuxVenvDirectoryCreator {}))
    } else {
        Err(eyre!("Your OS is not yet supported"))
    }
}
