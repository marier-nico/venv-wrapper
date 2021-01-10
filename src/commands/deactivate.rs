use std::env;

use eyre::{Context, Result};

use crate::settings::DeactivateSettings;

pub fn deactivate(settings: &DeactivateSettings) -> Result<()> {
    if let Ok(_env) = env::var("VIRTUAL_ENV") {
        std::fs::write(&settings.eval_file, "deactivate")
            .context("Unable to write to the eval file (cannot modify shell state)")?;
    }

    Ok(())
}
