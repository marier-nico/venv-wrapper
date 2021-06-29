use std::path::PathBuf;

use crate::shell::shell_operation::ShellOperation;

pub struct CompletionOperation {
    venv_dir: PathBuf,
}

impl CompletionOperation {
    pub fn new(venv_dir: PathBuf) -> Self {
        CompletionOperation {
            venv_dir,
        }
    }
}

impl ShellOperation for CompletionOperation {
    fn bash_eval(&self) -> String {
        todo!()
    }

    fn fish_eval(&self) -> String {
        format!(
            r#"complete -e venv
               set -l venv_wrapper_commands activate help init ls new
               set venv_wrapper_split_path (string split : $PATH)

               complete -f -c venv
               complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a activate -d "activate a virtualenv"
               complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a ls -d "list available virtualenvs"
               complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a new -d "create a new virtualenv"
               complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -s r -l venv-root -a "(ls -ad */ .*/)"

               complete -f -c venv -n "__fish_seen_subcommand_from activate" -a "(ls {})"
               complete -f -c venv -n "__fish_seen_subcommand_from new" -s p -l python -a "(/bin/find $venv_wrapper_split_path -name 'python*' | /bin/grep -o '\(python\|python.*[0-9]\)\b' | sort -u)"

               set -e venv_wrapper_commands
               set -e venv_wrapper_split_path
        "#,
            self.venv_dir.to_string_lossy(),
        )
    }

    fn zsh_eval(&self) -> String {
        todo!()
    }
}
