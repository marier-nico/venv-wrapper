use eyre::Result;
use std::convert::TryInto;

use crate::settings::GlobalSettings;

use super::link::link;
use super::unlink::unlink;

pub fn project_main(settings: &GlobalSettings) -> Result<()> {
    match settings.args.subcommand() {
        ("link", Some(_sub_matches)) => link(&settings.try_into()?)?,
        ("unlink", Some(_sub_matches)) => unlink(&settings.into())?,
        _ => return Err(eyre!("Unhandled subcommand")),
    }

    Ok(())
}
