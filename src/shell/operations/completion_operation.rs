use crate::shell::shell_operation::ShellOperation;

pub struct CompletionOperation;
impl ShellOperation for CompletionOperation {
    fn bash_eval(&self) -> String {
        todo!()
    }

    // TODO: Add completions for
    // - CLI options (-r at the root)
    // - python executable option in the new command (maybe try common paths for the python executable and `which python`)
    //   would be nice if a `which` returning all paths existed
    // TODO: Use the configured value for the venv home by default, also be aware of the root `-r` option
    fn fish_eval(&self) -> String {
        String::from(
            r#"set -l commands activate help init ls new
               complete -c venv -f
               complete -c venv -n "not __fish_seen_subcommand_from $commands" -a "$commands"
               complete -c venv -n "__fish_seen_subcommand_from activate" -a "(ls $HOME/.virtualenvs)"
        "#,
        )
    }

    fn zsh_eval(&self) -> String {
        todo!()
    }
}
