use std::path::PathBuf;

use ansi_term::{Colour::Blue, Colour::Green, Colour::Yellow};
use clap::ArgMatches;
use eyre::{Context, Result};

use crate::settings::Settings;

pub fn ls(settings: &Settings, args: &ArgMatches) -> Result<()> {
    let venvs_dir = PathBuf::from(args.value_of("venvs_dir").unwrap_or(&settings.venvs_dir));

    let contents = venvs_dir.read_dir();
    if let Err(read_err) = contents {
        match read_err.kind() {
            std::io::ErrorKind::NotFound => {
                return {
                    print_no_venvs_found(&venvs_dir.to_string_lossy());
                    Ok(())
                }
            }
            _ => return Err(read_err).context("Could not access the virtualenv directory"),
        }
    }

    let mut invalid_utf_8 = false;
    let venv_names: Vec<String> = contents
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter(|dir| {
            let mut path = dir.path();
            path.push("bin/activate");
            path.exists() && path.is_file()
        })
        .filter_map(|dir| {
            let dir_name = dir.file_name().into_string();
            if dir_name.is_err() {
                invalid_utf_8 = true;
            }
            dir_name.ok()
        })
        .map(|dir_name| format!("{:>sp$}{} {}", "", Green.paint("•"), dir_name, sp = 4))
        .collect();

    if invalid_utf_8 {
        println!(
            "\n {} Some virtual environment names were not valid UTF-8.",
            Yellow.paint("w")
        );
    }

    if venv_names.is_empty() {
        print_no_venvs_found(&venvs_dir.to_string_lossy());
    } else {
        println!("\n {}  Found the following virtualenvs.", Green.paint("✔"));
        venv_names.iter().for_each(|venv| println!("{}", venv));
    }

    Ok(())
}

fn print_no_venvs_found(venvs_home: &str) {
    println!(
        "\n {}  No virtual environments in `{}`.\n\n \
        {} Consider creating a virtualenv with `venv new`.",
        Green.paint("✔"),
        venvs_home,
        Blue.paint("Tip:")
    );
}
