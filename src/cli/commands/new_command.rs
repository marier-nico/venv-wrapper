use crate::{
    cli::command::{Command, CommandResult},
    config::config_data::Config,
    virtualenv::{
        creation::{
            create_virtualenv, locate_interpreter::locate_interpreter,
            python_version::PythonVersion,
        },
        virtualenv_info::Virtualenv,
    },
};
use clap::ArgMatches;
use eyre::{eyre, Result};

pub struct NewCommand;
impl Command for NewCommand {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult> {
        let venv_name = matches.value_of("name").unwrap();

        let interpreter_info = matches.value_of("python_executable").unwrap();
        let interpreter = match locate_interpreter(interpreter_info) {
            Some(interpreter) => interpreter,
            None => return Err(eyre!("Could not find the specified python interpreter")),
        };

        let python_version = PythonVersion::try_from_interpreter(&interpreter)?;

        let venv = Virtualenv {
            name: venv_name.to_string(),
            parent_dir: config.venv_root.as_ref().unwrap().clone(),
            python_version,
        };

        create_virtualenv(&venv, &interpreter)?;

        Ok(CommandResult::new().output(Box::new(String::from("🚀️ Virtualenv created"))))
    }
}
