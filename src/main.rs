use clap::ArgMatches;
use cli::{error_handler::install, get_app::get_app};
use config::config_data::Config;
use shell::get_eval::get_eval_from_env;

use crate::{
    cli::{cli_main::cli_main, logging::init_logging},
    shell::get_eval::get_eval_fron_shell_name,
};
use eyre::Result;

mod cli;
mod config;
mod constants;
mod presentation;
mod shell;
mod utils;
mod virtualenv;

fn main() {
    init_logging();
    let matches = match get_app().get_matches_safe() {
        Ok(matches) => matches,
        Err(e) => return eprintln!("{}", e.message),
    };

    let cli_config = Config::from(&matches);
    let config =
        Config::merge(&[Config::default(), Config::from_file(), Config::from_env(), cli_config]);

    let result = run_and_process_result(&matches, &config);

    if let Err(e) = result {
        return eprintln!("{:?}", e);
    }
}

fn run_and_process_result(matches: &ArgMatches, config: &Config) -> Result<()> {
    install()?;
    let command_result = cli_main(&matches, &config)?;

    if let Some(shell_operation) = command_result.shell_operation {
        let eval = match command_result.shell_name {
            Some(name) => get_eval_fron_shell_name(&*shell_operation, &name)?,
            None => get_eval_from_env(&*shell_operation)?,
        };

        print!("{}", eval);
    }

    if let Some(visible_output) = command_result.visible_output {
        eprintln!("{}", visible_output);
    }

    Ok(())
}
