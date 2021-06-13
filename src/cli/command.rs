use std::fmt::Display;

use crate::{config::config_data::Config, shell::shell_operation::ShellOperation};
use clap::ArgMatches;
use eyre::Result;

pub trait Command {
    fn run(config: &Config, matches: &ArgMatches) -> Result<CommandResult>;
}

pub struct CommandResult {
    pub shell_name: Option<String>,
    pub shell_operation: Option<Box<dyn ShellOperation>>,
    pub visible_output: Option<Box<dyn Display>>,
}

impl CommandResult {
    pub fn new() -> Self {
        CommandResult {
            shell_name: None,
            shell_operation: None,
            visible_output: None,
        }
    }

    pub fn with_shell(mut self, shell_name: String) -> Self {
        self.shell_name = Some(shell_name);
        self
    }

    pub fn operation(mut self, operation: Box<dyn ShellOperation>) -> Self {
        self.shell_operation = Some(operation);
        self
    }

    pub fn output(mut self, output: Box<dyn Display>) -> Self {
        self.visible_output = Some(output);
        self
    }
}
