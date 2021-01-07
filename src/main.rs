#[macro_use]
extern crate clap;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate serde;

use std::path::PathBuf;

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
    let settings = settings::Settings::new(&matches)?;
    match matches.subcommand() {
        ("init", Some(_sub_matches)) => commands::init(&eval_file)?,
        ("ls", Some(_sub_matches)) => commands::ls(&settings)?,
        ("new", Some(sub_matches)) => commands::new(&settings, sub_matches, &eval_file)?,
        ("activate", Some(sub_matches)) => {
            commands::activate_cli(&settings, sub_matches, &eval_file)?
        }
        ("deactivate", Some(_sub_matches)) => commands::deactivate(&eval_file)?,
        ("rm", Some(sub_matches)) => commands::rm(&settings, sub_matches)?,
        ("project", Some(sub_matches)) => commands::project_main(&settings, sub_matches)?,
        ("use", Some(sub_matches)) => commands::use_command(&settings, sub_matches, &eval_file)?,
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
