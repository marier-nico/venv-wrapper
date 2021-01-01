#[macro_use]
extern crate clap;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate serde;

use eyre::Result;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod clap_app;
mod commands;
mod settings;

fn main() {
    let result = cli_main();
    if let Err(e) = result {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .ok();
        writeln!(&mut stdout, "\nError: {:?}", e).ok();
    }
}

fn cli_main() -> Result<()> {
    simple_eyre::install()?;
    let settings = settings::Settings::new()?;

    let matches = clap_app::get_app().get_matches();
    match matches.subcommand() {
        ("init", Some(_sub_matches)) => commands::init()?,
        _ => return Err(eyre!("Unhandled subcommand")),
    }

    Ok(())
}
