use std::env::current_exe;

use eyre::Context;

pub fn init() -> eyre::Result<()> {
    let current_exe = current_exe().context("Could not determine current executable path")?;
    let s = format!(
        "function venv () {{
        result=\"$({} $@)\"

        if [[ \"$result\" == SHELLEVAL* ]]; then
            eval $(echo \"$result\" | cut -c10-)
        else
            echo -e $result
        fi
    }}",
        current_exe
            .to_str()
            .ok_or_else(|| eyre!("Path to executable is not valid UTF-8"))?
    );

    print!("{}", s);
    Ok(())
}
