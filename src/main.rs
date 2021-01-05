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
    let settings = settings::Settings::new()?;
    let eval_file = create_eval_dir()?;

    let matches = clap_app::get_app().get_matches();
    match matches.subcommand() {
        ("init", Some(_sub_matches)) => commands::init(eval_file)?,
        ("ls", Some(sub_matches)) => commands::ls(&settings, sub_matches)?,
        ("new", Some(sub_matches)) => commands::new(&settings, sub_matches)?,
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
