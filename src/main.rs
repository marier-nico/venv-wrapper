#[macro_use]
extern crate clap;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate serde;

use ansi_term::Colour::Red;
use eyre::Result;

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

    let matches = clap_app::get_app().get_matches();
    match matches.subcommand() {
        ("init", Some(_sub_matches)) => commands::init()?,
        ("ls", Some(sub_matches)) => commands::ls(&settings, sub_matches)?,
        _ => return Err(eyre!("Unhandled subcommand")),
    }

    Ok(())
}
