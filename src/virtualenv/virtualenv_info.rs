use ansi_term::Style;
use eyre::{eyre, Report, Result, WrapErr};
use indenter::indented;
use ini::Ini;
use std::convert::TryFrom;
use std::fmt::Write;
use std::fs;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use super::creation::{link_path::link_path, python_version::PythonVersion};

pub struct Virtualenv {
    pub name: String,
    parent_dir: PathBuf,
    python_version: PythonVersion,
    linked_project: Option<PathBuf>,
    config: Ini,
}

impl Virtualenv {
    pub fn new(
        name: String,
        parent_dir: PathBuf,
        python_version: PythonVersion,
        linked_project: Option<PathBuf>,
        source_interpreter: &Path,
    ) -> Result<Self> {
        let config = Virtualenv::generate_config(&python_version, source_interpreter)?;

        Ok(Virtualenv {
            name,
            parent_dir,
            python_version,
            linked_project,
            config,
        })
    }

    fn generate_config(python_version: &PythonVersion, interpreter: &Path) -> Result<Ini> {
        let mut config = Ini::new();

        let home = interpreter
            .parent()
            .ok_or_else(|| eyre!("Interpreter does not have a parent directory"))?;

        config
            .with_general_section()
            .set("home", home.to_string_lossy())
            .set("include-system-site-packages", "false")
            .set("version", python_version.to_string());

        Ok(config)
    }

    pub fn update_config(&mut self, key: &str, value: &str) {
        self.config.with_general_section().set(key, value);
    }

    pub fn delete_config_key(&mut self, key: &str) {
        self.config.with_general_section().delete(&key);
    }

    pub fn write_config(&self) -> Result<()> {
        self.config
            .write_to_file(self.config_path())
            .wrap_err("Could not write the virtualenv config")?;

        Ok(())
    }

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

    pub fn config_path(&self) -> PathBuf {
        self.path().join("pyvenv.cfg")
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

    pub fn add_python(&self, interpreter: &Path) -> Result<()> {
        for name in self.python_version.executable_names() {
            link_path(interpreter, &self.bin_path().join(name))
                .wrap_err("Could not add python executable to venv")?;
        }

        Ok(())
    }

    pub fn find_all_in_path(path: &Path) -> Vec<Self> {
        let mut found_venvs = vec![];

        let dirs = fs::read_dir(path);
        if dirs.is_err() {
            return found_venvs;
        }

        for entry in dirs.unwrap().flatten() {
            let venv_path = entry.path();
            match Self::try_from(venv_path.as_ref()) {
                Ok(venv) => found_venvs.push(venv),
                Err(_) => continue,
            }
        }

        found_venvs
    }
}

impl Display for Virtualenv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &str = self.name.as_ref();

        writeln!(f, "{}", Style::new().bold().paint(name))?;
        writeln!(
            indented(f),
            "Python Version: {}",
            Style::new().bold().paint(self.python_version.patch_prefix())
        )?;
        writeln!(indented(f), "Venv Directory: {}", self.path().to_string_lossy())?;

        write!(indented(f), "Linked Project: ")?;
        if self.linked_project.is_some() {
            write!(f, "{}", self.linked_project.as_ref().unwrap().to_string_lossy())?;
        } else {
            write!(f, "None")?;
        }

        Ok(())
    }
}

impl TryFrom<&Path> for Virtualenv {
    type Error = Report;

    fn try_from(venv_path: &Path) -> Result<Self, Self::Error> {
        if !venv_path.is_dir() {
            return Err(eyre!("The path to the virtualenv should point to a directory"));
        }

        let venv_config_file = venv_path.join("pyvenv.cfg");
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

        let project_path = config.get_from::<&str>(None, "project").map(PathBuf::from);

        let name = venv_path
            .file_name()
            .ok_or_else(|| eyre!("Could not find the venv name"))?
            .to_string_lossy();

        let parent_dir = venv_path
            .parent()
            .ok_or_else(|| eyre!("Could not find the venv's parent directory"))?;

        Ok(Virtualenv {
            name: String::from(name),
            parent_dir: parent_dir.to_owned(),
            python_version: version,
            linked_project: project_path,
            config,
        })
    }
}
