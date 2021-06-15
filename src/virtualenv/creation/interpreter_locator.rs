use is_executable::IsExecutable;
use std::{path::PathBuf, process::Command};
use which::which;

pub trait InterpreterLocator {
    fn locate_interpreter(&self, interpreter_name: &str) -> Option<PathBuf>;
}

pub struct ExplicitPathInterpreterLocator;
impl InterpreterLocator for ExplicitPathInterpreterLocator {
    fn locate_interpreter(&self, interpreter_path: &str) -> Option<PathBuf> {
        let interpreter_path = PathBuf::from(interpreter_path);

        if interpreter_path.is_executable() {
            Some(interpreter_path)
        } else {
            None
        }
    }
}

pub struct NameInterpreterLocator;
impl InterpreterLocator for NameInterpreterLocator {
    fn locate_interpreter(&self, interpreter_name: &str) -> Option<PathBuf> {
        let interpreter_path = which(interpreter_name);
        match interpreter_path {
            Ok(interpreter) => Some(interpreter),
            Err(_) => None,
        }
    }
}

pub struct PyenvNameInterpreterLocator;
impl InterpreterLocator for PyenvNameInterpreterLocator {
    fn locate_interpreter(&self, interpreter_name: &str) -> Option<PathBuf> {
        let output = Command::new("pyenv").arg("which").arg(interpreter_name).output();
        let output = match output {
            Ok(o) => o,
            Err(_) => return None,
        };

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Some(PathBuf::from(stdout.trim()))
    }
}

pub fn get_interpreter_locator(interpreter_info: &str) -> Box<dyn InterpreterLocator> {
    let provided_path = PathBuf::from(interpreter_info);

    if provided_path.exists() {
        return Box::new(ExplicitPathInterpreterLocator {});
    }

    match which("pyenv") {
        Ok(_) => Box::new(PyenvNameInterpreterLocator {}),
        Err(_) => Box::new(NameInterpreterLocator {}),
    }
}
