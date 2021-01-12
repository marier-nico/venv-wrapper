#[macro_use]
extern crate clap;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate serde;

use std::{convert::TryInto, path::PathBuf};

use ansi_term::Colour::Red;
use eyre::{Context, Result};

mod clap_app;
mod commands;
mod settings;

fn main() {
    let result = cli_main();
    if let Err(e) = result {
        println!("{}", Red.paint(format!("\nError: {:?}\n", e)));
    }
}

fn cli_main() -> Result<()> {
    simple_eyre::install()?;
    let eval_file = create_eval_dir()?;

    let matches = clap_app::get_app().get_matches();
    let config = settings::ConfigSettings::new(&matches)?;
    let settings = settings::GlobalSettings::new(config, &matches, &eval_file);
    match matches.subcommand() {
        ("init", Some(_sub_matches)) => commands::init(&settings.into())?,
        ("ls", Some(_sub_matches)) => commands::ls(&settings.into())?,
        ("new", Some(_sub_matches)) => commands::new(&settings.try_into()?)?,
        ("activate", Some(_sub_matches)) => commands::activate_cli(&settings.try_into()?)?,
        ("deactivate", Some(_sub_matches)) => commands::deactivate(&settings.into())?,
        ("rm", Some(_sub_matches)) => commands::rm(&settings.try_into()?)?,
        ("project", Some(_sub_matches)) => commands::project_main(&settings)?,
        ("use", Some(_sub_matches)) => commands::use_command(&settings.try_into()?)?,
        _ => return Err(eyre!("Unhandled subcommand")),
    }

    Ok(())
}

fn create_eval_dir() -> Result<PathBuf> {
    let eval_file = directories::BaseDirs::new().unwrap().cache_dir().join("venv-wrapper/eval");
    std::fs::create_dir_all(eval_file.parent().unwrap())
        .context("Could not create cache directory")?;

    Ok(eval_file)
}
