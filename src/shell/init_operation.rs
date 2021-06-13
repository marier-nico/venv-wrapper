use crate::constants::VENV_WRAPPER_SHELL_ENV_VAR_NAME;

use super::shell_operation::ShellOperation;

pub struct InitOperation {}
impl ShellOperation for InitOperation {
    fn bash_eval(&self) -> String {
        format!(
            r#"export {}=bash
               venv() {{
                    local result
                    result="$(venv-wrapper $@)"

                    if test -n "$result"; then
                        eval "$result"
                    fi
               }}"#,
            VENV_WRAPPER_SHELL_ENV_VAR_NAME
        )
    }

    fn fish_eval(&self) -> String {
        format!(
            r#"set -gx {} fish
               function venv
                    set result (cargo run -- $argv)

                    if test -n "$result"
                        echo "$result" | source
                    end
               end"#,
            VENV_WRAPPER_SHELL_ENV_VAR_NAME
        )
    }

    fn zsh_eval(&self) -> String {
        format!(
            r#"export {}=zsh
               venv() {{
                    local result
                    result="$(venv-wrapper $@)"

                    if test -n "$result"; then
                        eval "$result"
                    fi
               }}"#,
            VENV_WRAPPER_SHELL_ENV_VAR_NAME
        )
    }
}
