use clap::ArgMatches;
use eyre::{Context, Result};
use std::io::Write;
use std::path::Path;
use std::{fs::OpenOptions, path::PathBuf};

use crate::commands::activate;
use crate::settings::Settings;

pub fn use_command(settings: &Settings, args: &ArgMatches, eval_file: &Path) -> Result<()> {
    let venvs_dir = PathBuf::from(&settings.venvs_dir);
    let venv_name = args.value_of("venv_name").unwrap(); // `venv_name` is a required arg

    activate(&venvs_dir, venv_name, eval_file)?;

    let project_file_path = venvs_dir.join(venv_name).join("project_dir");
    if project_file_path.exists() {
        let project_path = std::fs::read_to_string(project_file_path)
            .context("Could not find the project location")?;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(eval_file)
            .context("Unable to open the eval file (cannot modify shell state)")?;

        writeln!(file, "\ncd {}", project_path.trim())
            .context("Unable to write to the eval file (cannot modify shell state)")?;
    }

    Ok(())
}
