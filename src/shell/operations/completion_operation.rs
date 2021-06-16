use std::path::PathBuf;

use crate::shell::shell_operation::ShellOperation;

pub struct CompletionOperation {
    venv_dir: PathBuf
}

impl ShellOperation for CompletionOperation {
    fn bash_eval(&self) -> String {
        todo!()
    }

    // TODO: Use the configured value for the venv home by default, also be aware of the root `-r` option
    fn fish_eval(&self) -> String {
        String::from(
            r#"set -l subcommands activate help init ls new
               complete -c venv -f

               # Handle completions for the venv-root option
               complete -c venv -n "not __fish_seen_subcommand_from $subcommands" -s r -l venv-root -a "(ls -d */ .*/)"

               # Suggest subcommands
               complete -c venv -n "not __fish_seen_subcommand_from $subcommands" -a "$subcommands"

               # Suggest available virtualenvs for activation
               complete -c venv -n "__fish_seen_subcommand_from activate" -a "(ls $HOME/.virtualenvs)"

               # Suggest python interpreters for new virtualenvs
               set split_path (string split : $PATH)
               complete -c venv -n "__fish_seen_subcommand_from new" -s p -l python -a "(/bin/find $split_path -name 'python*' | /bin/grep -o '\(python\|python.*[0-9]\)\b' | sort -u)"
        "#,
        )
    }

    fn zsh_eval(&self) -> String {
        todo!()
    }
}
