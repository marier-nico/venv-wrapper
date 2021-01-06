use clap::ArgMatches;
use eyre::Result;

use crate::settings::Settings;

use super::link::link;
use super::unlink::unlink;

pub fn project_main(settings: &Settings, args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("link", Some(sub_matches)) => link(&settings, sub_matches)?,
        ("unlink", Some(sub_matches)) => unlink(&settings, sub_matches)?,
        _ => return Err(eyre!("Unhandled subcommand")),
    }

    Ok(())
}
