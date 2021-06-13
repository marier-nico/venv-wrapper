use std::env;

use crate::constants::VENV_WRAPPER_SHELL_ENV_VAR_NAME;

use super::shell_operation::ShellOperation;
use eyre::{eyre, Result};

pub fn get_eval_from_env(operation: &dyn ShellOperation) -> Result<String> {
    let shell_name = env::var(VENV_WRAPPER_SHELL_ENV_VAR_NAME)
        .map_err(|_| eyre!("It looks like venv-wrapper is not initialized, please initialize it before running any commands!"))?;

    get_eval_fron_shell_name(operation, &shell_name)
}

pub fn get_eval_fron_shell_name(operation: &dyn ShellOperation, shell_name: &str) -> Result<String> {
    match shell_name {
        "bash" => Ok(operation.bash_eval()),
        "fish" => Ok(operation.fish_eval()),
        "zsh" => Ok(operation.zsh_eval()),
        _ => Err(eyre!("Sorry, your shell isn't supported, feel free to open an issue on GitHub!"))
    }

}
