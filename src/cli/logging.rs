use ansi_term::{
    ANSIString, ANSIStrings,
    Colour::{Blue, Green, Purple, Red, Yellow},
    Style,
};
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logging() {
    let mut builder = Builder::from_default_env();

    builder
        .format(|buf, record| {
            writeln!(buf, "{} {}", ANSIStrings(&format_level(record.level())), record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
}

fn format_level(level: Level) -> Vec<ANSIString<'static>> {
    match level {
        Level::Error => vec![Red.bold().paint("error"), Style::new().bold().paint(":")],
        Level::Warn => vec![Yellow.bold().paint("warn"), Style::new().bold().paint(":")],
        Level::Info => vec![Blue.bold().paint("info"), Style::new().bold().paint(":")],
        Level::Debug => vec![Green.bold().paint("debug"), Style::new().bold().paint(":")],
        Level::Trace => vec![Purple.bold().paint("trace"), Style::new().bold().paint(":")],
    }
}
