use clap::ArgMatches;
use config::Config;
use directories::UserDirs;
use eyre::{Context, Result};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub venvs_dir: String,
}

impl Settings {
    pub fn new(matches: &ArgMatches) -> Result<Self> {
        let mut settings = Config::new();
        set_defaults(&mut settings)?;

        // It's fine to unwrap here because if no home directory path is found, an Err() will be
        // returned when setting the default value just above.
        let dir = directories::ProjectDirs::from("com", "venv-wrapper", "venv-wrapper").unwrap();
        let mut config_file = dir.preference_dir().to_owned();
        config_file.push("config.toml");

        settings.merge(config::File::from(config_file)).ok();
        settings.merge(config::Environment::with_prefix("VENVWRAPPER")).ok();
        set_cli_overrides(&mut settings, matches)?;

        Ok(settings.try_into().context("The provided configuration is in an invalid format")?)
    }
}

fn set_defaults(settings: &mut Config) -> Result<()> {
    let dirs = UserDirs::new()
        .ok_or_else(|| eyre!("Could not determine the path to your home directory"))?;

    let default_venvs_dir = dirs.home_dir().join(".virtualenvs");
    settings
        .set_default(
            "venvs_dir",
            default_venvs_dir
                .to_str()
                .ok_or_else(|| eyre!("Path to home directory is not valid UTF-8"))?,
        )
        .ok();

    Ok(())
}

fn set_cli_overrides(settings: &mut Config, matches: &ArgMatches) -> Result<()> {
    let venvs_dir = match matches.value_of("venvs_dir") {
        Some(dir) => Some(dir),
        None => None,
    };

    settings.set("venvs_dir", venvs_dir).context("Could not set the venv-dir value")?;

    Ok(())
}
