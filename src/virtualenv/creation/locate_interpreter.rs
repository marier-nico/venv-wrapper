use is_executable::IsExecutable;
use std::{path::PathBuf, process::Command};
use which::which;

pub fn locate_interpreter(interpreter_info: &str) -> Option<PathBuf> {
    let explicit_path_result = locate_explicit_path(interpreter_info);

    if explicit_path_result.is_some() {
        explicit_path_result
    } else {
        match which("pyenv") {
            Ok(_) => locate_pyenv_interpreter(interpreter_info),
            Err(_) => locate_named_interpreter(interpreter_info),
        }
    }
}

fn locate_explicit_path(interpreter_path: &str) -> Option<PathBuf> {
    let interpreter_path = PathBuf::from(interpreter_path);

    if interpreter_path.exists() && interpreter_path.is_executable() {
        Some(interpreter_path)
    } else {
        None
    }
}

fn locate_named_interpreter(interpreter_name: &str) -> Option<PathBuf> {
    let interpreter_path = which(interpreter_name);
    match interpreter_path {
        Ok(interpreter) => Some(interpreter),
        Err(_) => None,
    }
}

fn locate_pyenv_interpreter(interpreter_name: &str) -> Option<PathBuf> {
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
