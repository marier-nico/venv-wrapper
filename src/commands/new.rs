use ansi_term::Colour::Green;
use quale::which;
use std::process::{Command, Stdio};

use eyre::{Context, Result};
use spinners::{Spinner, Spinners};

use crate::{commands::activate, settings::NewSettings};

pub fn new(settings: &NewSettings) -> Result<()> {
    let venv_path = &settings.venvs_dir.join(&settings.venv_name);

    if venv_path.exists() {
        return Err(eyre!("A directory named `{}` already exists", &settings.venv_name));
    }

    let python_exec_path = which(&settings.python_executable).ok_or_else(|| {
        eyre!(
            "Could not determine the path to the python executable ({})",
            &settings.python_executable
        )
    })?;

    println!();
    let spinner = Spinner::new(Spinners::Dots, String::from("Creating your virtual environment"));
    let cmd = Command::new(python_exec_path)
        .args(&[
            "-m",
            "venv",
            venv_path
                .to_str()
                .ok_or_else(|| eyre!("Path to virtualenv contained invalid UTF-8"))?,
        ])
        .stdout(Stdio::null())
        .output()
        .context("Unable to call the python binary to create a virtualenv")?;
    spinner.stop();

    if cmd.status.success() {
        println!(
            "\n\n {}  Successfully created the virtualenv `{}`.",
            Green.paint("✔"),
            &settings.venv_name
        );
    } else {
        let stderr = String::from_utf8(cmd.stderr)?.trim().to_owned();
        return Err(eyre!(stderr))
            .context("Could not create the virtualenv with the specified python executable");
    }

    activate(&settings.venvs_dir, &settings.venv_name, &settings.eval_file)?;

    Ok(())
}
