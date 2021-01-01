#[macro_use]
extern crate clap;
#[macro_use]
extern crate eyre;

use std::io::Write;
use simple_eyre::eyre::Result;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod clap_app;
mod commands;

fn main() {
    let result = cli_main();
    if let Err(e) = result {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).ok();
        writeln!(&mut stdout, "\nError: {:?}", e).ok();
    }
}

fn cli_main() -> Result<()> {
    simple_eyre::install()?;

    let matches = clap_app::get_app().get_matches();
    match matches.subcommand() {
        ("init", Some(_sub_matches)) => commands::init()?,
        _ => return Err(eyre!("Unhandled subcommand")),
    }

    Ok(())
}
