use ansi_term::Style;
use indenter::indented;
use std::fmt::{Display, Write};

use crate::virtualenv::virtualenv_info::Virtualenv;

impl Display for Virtualenv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &str = self.name.as_ref();
        writeln!(f, "{}", Style::new().bold().paint(name))?;
        writeln!(
            indented(f),
            "Python Version: {}",
            Style::new().bold().paint(self.python_version.patch_prefix())
        )?;
        write!(indented(f), "Venv Directory: {}", self.path().to_string_lossy())?;

        Ok(())
    }
}
