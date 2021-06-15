use eyre::{eyre, Context, Error as EyreError, Result};
use std::{convert::TryFrom, num::ParseIntError, path::Path, process::Command};

pub struct PythonVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl PythonVersion {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        PythonVersion {
            major,
            minor,
            patch,
        }
    }

    pub fn prefix() -> String {
        String::from("python")
    }

    pub fn major_prefix(&self) -> String {
        format!("python{}", self.major)
    }

    pub fn minor_prefix(&self) -> String {
        format!("python{}.{}", self.major, self.minor)
    }

    pub fn patch_prefix(&self) -> String {
        format!("python{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn executable_names(&self) -> Vec<String> {
        return vec![self.minor_prefix(), self.major_prefix(), PythonVersion::prefix()];
    }

    pub fn try_from_interpreter(interpreter: &Path) -> Result<Self> {
        let result = Command::new(interpreter)
            .arg("-c")
            .arg("import sys; print(f'{sys.version_info[0]}.{sys.version_info[1]}.{sys.version_info[2]}')")
            .output()
            .wrap_err("Could not determine the full python version")?;

        let stdout = String::from_utf8_lossy(&result.stdout);
        let trimmed = stdout.trim();
        let parsed = Self::try_from(trimmed)?;

        Ok(parsed)
    }
}

impl ToString for PythonVersion {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&str> for PythonVersion {
    type Error = EyreError;

    /// Convert a python version string into a PythonVersion.
    ///
    /// Example of a python version string :
    /// - `3.9.5`
    ///
    /// # Errors
    ///
    /// An error is returned if the version string does not include all the required version
    /// information or if it isn't in theh correct format.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let parsed: Vec<Result<u8, ParseIntError>> =
            value.split('.').map(|x| x.parse::<u8>()).collect();

        if parsed.len() != 3 {
            return Err(eyre!("Python version string '{}' must be in the x.y.z format", value));
        }

        let mut parts = vec![];
        for result in parsed {
            match result {
                Err(_) => {
                    return Err(eyre!("Python version parts must be numbers, got '{}'", value))
                }
                Ok(part) => parts.push(part),
            }
        }

        Ok(PythonVersion::new(parts[0], parts[1], parts[2]))
    }
}
