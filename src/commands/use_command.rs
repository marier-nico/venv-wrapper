use eyre::{Context, Result};
use std::fs::OpenOptions;
use std::io::Write;

use crate::{commands::activate, settings::UseSettings};

pub fn use_command(settings: &UseSettings) -> Result<()> {
    activate(&settings.venv, &settings.eval_file)?;

    let project_file_path = &settings.venv.path.join("project_dir");
    if project_file_path.exists() {
        let project_path = std::fs::read_to_string(project_file_path)
            .context("Could not find the project location")?;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&settings.eval_file)
            .context("Unable to open the eval file (cannot modify shell state)")?;

        writeln!(file, "\ncd {}", project_path.trim())
            .context("Unable to write to the eval file (cannot modify shell state)")?;
    }

    Ok(())
}
