use clap::ArgMatches;
use cli::get_app::get_app;
use config::config_data::Config;
use shell::get_eval::get_eval_from_env;

use crate::{cli::cli_main::cli_main, shell::get_eval::get_eval_fron_shell_name};
use eyre::Result;

mod cli;
mod config;
mod constants;
mod shell;
mod virtualenv;

fn main() {
    let matches = get_app().get_matches();
    let cli_config = Config::from(&matches);
    let config = Config::merge(vec![Config::default(), cli_config]);

    let result = run_and_process_result(&matches, &config);

    if let Err(e) = result {
        return print!("{:?}", e);
    }
}

fn run_and_process_result(matches: &ArgMatches, config: &Config) -> Result<()> {
    let command_result = cli_main(&matches, &config)?;

    if let Some(shell_operation) = command_result.shell_operation {
        let eval = match command_result.shell_name {
            Some(name) => get_eval_fron_shell_name(&*shell_operation, &name)?,
            None => get_eval_from_env(&*shell_operation)?
        }; 

        print!("{}", eval);
    }

    if let Some(visible_output) = command_result.visible_output {
        eprint!("{}", visible_output);
    }

    Ok(())
}
