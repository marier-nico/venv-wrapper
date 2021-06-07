use eyre::Result;

use super::get_app::get_app;

pub fn cli_main() -> Result<()> {
    let matches = get_app().get_matches();

    panic!("Not Implemented")
}
