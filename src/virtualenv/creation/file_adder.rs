use eyre::{Result, WrapErr};
use std::path::Path;

pub trait FileAdder {
    fn add_file(&self, original: &Path, destination: &Path) -> Result<()>;
}

pub struct LinkingFileAdder;
impl FileAdder for LinkingFileAdder {
    #[cfg(target_family = "unix")]
    fn add_file(&self, original: &Path, link: &Path) -> Result<()> {
        use std::os::unix::fs;

        fs::symlink(original, link).wrap_err(format!("Could not link {:?} to {:?}", original, link))
    }

    #[cfg(target_family = "windows")]
    fn add_file(&self, original: &Path, link: &Path) -> Result<()> {
        use std::os::windows::fs;

        if original.is_dir() {
            fs::symlink_dir(original, link)
                .wrap_err(format!("Could not link {:?} to {:?}", original, link))
        } else {
            fs::symlink_file(original, link)
                .wrap_err(format!("Could not link {:?} to {:?}", original, link))
        }
    }
}

pub fn get_file_adder() -> Box<dyn FileAdder> {
    Box::new(LinkingFileAdder {})
}
