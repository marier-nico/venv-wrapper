use eyre::{Result, WrapErr};
use std::path::Path;

#[cfg(target_family = "unix")]
pub fn link_path(original: &Path, link: &Path) -> Result<()> {
    use std::os::unix::fs;

    fs::symlink(original, link).wrap_err(format!("Could not link {:?} to {:?}", original, link))
}

#[cfg(target_family = "windows")]
pub fn link_path(original: &Path, link: &Path) -> Result<()> {
    use std::os::windows::fs;

    if original.is_dir() {
        fs::symlink_dir(original, link)
            .wrap_err(format!("Could not link {:?} to {:?}", original, link))
    } else {
        fs::symlink_file(original, link)
            .wrap_err(format!("Could not link {:?} to {:?}", original, link))
    }
}
