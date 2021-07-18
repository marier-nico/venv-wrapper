use ansi_term::{Colour::Red, Style};
use eyre::{EyreHandler, Result};
use indenter::indented;

#[derive(Debug)]
pub struct FriendlyHandler;
impl EyreHandler for FriendlyHandler {
    fn debug(
        &self,
        error: &(dyn std::error::Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        use core::fmt::Write as _;

        if f.alternate() {
            return core::fmt::Debug::fmt(error, f);
        }

        write!(f, "{}{} {}", Red.bold().paint("error"), Style::new().bold().paint(":"), error)?;

        if let Some(cause) = error.source() {
            let errors = std::iter::successors(Some(cause), |e| (*e).source());

            for error in errors {
                writeln!(f)?;
                write!(indented(f), "{} {}", Style::new().bold().paint("cause:"), error)?;
            }
        }

        Ok(())
    }
}

pub fn install() -> Result<()> {
    eyre::set_hook(Box::new(move |_| Box::new(FriendlyHandler)))?;

    Ok(())
}
