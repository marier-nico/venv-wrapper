use std::env;
use std::path::Path;

use eyre::{Context, Result};

pub fn deactivate(eval_file: &Path) -> Result<()> {
    if let Ok(_env) = env::var("VIRTUAL_ENV") {
        std::fs::write(eval_file, "deactivate")
            .context("Unable to write to the eval file (cannot modify shell state)")?;
    }

    Ok(())
}
