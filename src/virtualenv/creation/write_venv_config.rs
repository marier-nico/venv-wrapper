use eyre::{eyre, Context, Result};
use std::{fs::File, io::Write, path::Path};

use crate::virtualenv::virtualenv_info::Virtualenv;

use os_str_bytes::OsStrBytes;

pub fn write_venv_config(venv: &Virtualenv, interpreter: &Path) -> Result<()> {
    let mut config_file = File::create(venv.path().join("pyvenv.cfg"))
        .wrap_err("Could not create venv config file")?;

    let mut buf: Vec<u8> = vec![];
    buf.extend(b"home = ");
    buf.extend(&*OsStrBytes::to_raw_bytes(
        interpreter.parent().ok_or_else(|| eyre!("Interpreter path did not have a parent"))?,
    ));
    buf.extend(b"\ninclude-system-site-packages = false");
    buf.extend(b"\nversion = ");
    buf.extend(venv.python_version.to_string().as_bytes());

    config_file.write_all(&buf).wrap_err("Could not write to venv config file")?;

    Ok(())
}
