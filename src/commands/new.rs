use ansi_term::Colour::Green;
use quale::which;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::ArgMatches;
use eyre::{Context, Result};
use spinners::{Spinner, Spinners};

use crate::settings::Settings;

pub fn new(settings: &Settings, args: &ArgMatches) -> Result<()> {
    let mut venv_dir = PathBuf::from(args.value_of("venvs_dir").unwrap_or(&settings.venvs_dir));
    let venv_name = args.value_of("venv_name").unwrap();
    venv_dir.push(venv_name); // `venv_name` is a required arg

    if venv_dir.exists() {
        return Err(eyre!("A directory named `{}` already exists", venv_name));
    }

    let python_exec_name = args.value_of("python_executable").unwrap(); // There is a default value for this arg
    let python_exec_path = which(python_exec_name).ok_or_else(|| {
        eyre!("Could not determine the path to the python executable ({})", python_exec_name)
    })?;

    // TODO: Python2 does not work because the module `venv` doesn't exist
    //       Check the output of the command to make sure it was successful.
    println!();
    let spinner = Spinner::new(Spinners::Dots, String::from("Creating your virtual environment"));
    let _cmd = Command::new(python_exec_path)
        .args(&[
            "-m",
            "venv",
            venv_dir.to_str().ok_or_else(|| eyre!("Path to virtualenv contained invalid UTF-8"))?,
        ])
        .stdout(Stdio::null())
        .output()
        .context("Unable to call the python binary to create a virtualenv")?;
    spinner.stop();

    println!("\n\n {}  Successfully created the virtualenv `{}`.", Green.paint("✔"), venv_name);

    Ok(())
}
