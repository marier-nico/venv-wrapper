use std::{env::current_exe, path::PathBuf};

use eyre::Context;

pub fn init(eval_file: PathBuf) -> eyre::Result<()> {
    let current_exe = current_exe().context("Could not determine current executable path")?;
    let s = format!(
        "function venv () {{
            {bin} $@
            if [ -f '{eval}' ]; then
                source {eval}
                rm {eval}
            fi
        }}",
        bin = current_exe.to_str().ok_or_else(|| eyre!("Path to executable is not valid UTF-8"))?,
        eval = eval_file.to_str().ok_or_else(|| eyre!("Eval file path was not valid UTF-8"))?
    );

    print!("{}", s);
    Ok(())
}
