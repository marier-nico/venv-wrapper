use std::env::current_exe;

use eyre::Context;

use crate::settings::InitSettings;

pub fn init(settings: &InitSettings) -> eyre::Result<()> {
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
        eval = settings
            .eval_file
            .to_str()
            .ok_or_else(|| eyre!("Eval file path was not valid UTF-8"))?
    );

    print!("{}", s);
    Ok(())
}
