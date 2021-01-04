use directories::UserDirs;
use eyre::Result;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub venvs_dir: String,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let mut settings = config::Config::new();

        // Set the default venvs directory
        if let Some(dir) = UserDirs::new() {
            let mut venvs_default_dir = dir.home_dir().to_owned();
            venvs_default_dir.push(".virtualenvs");

            settings
                .set_default(
                    "venvs_dir",
                    venvs_default_dir
                        .to_str()
                        .ok_or_else(|| eyre!("Path to home directory is not valid UTF-8"))?,
                )
                .ok();
        } else {
            return Err(eyre!("Could not determine the path to your home directory"));
        }

        // It's fine to unwrap here because if no home directory path is found, an Err() will be
        // returned when setting the default value just above.
        let dir = directories::ProjectDirs::from("com", "venv-wrapper", "venv-wrapper").unwrap();
        let mut config_file = dir.preference_dir().to_owned();
        config_file.push("config.toml");
        settings.merge(config::File::from(config_file)).ok();

        settings.merge(config::Environment::with_prefix("VENVWRAPPER")).ok();

        Ok(settings
            .try_into()
            .map_err(|_| eyre!("The provided configuration is in an invalid format"))?)
    }
}
