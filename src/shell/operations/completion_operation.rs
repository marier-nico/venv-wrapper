use crate::shell::shell_operation::ShellOperation;

pub struct CompletionOperation;
impl ShellOperation for CompletionOperation {
    fn bash_eval(&self) -> String {
        String::from(
            r#"
            complete -r venv

            _venv_wrapper_completions() {
                if [ "${#COMP_WORDS[@]}" = "2" ]; then
                    COMPREPLY=($(compgen -W "activate help link ls new rm unlink use" -- "${COMP_WORDS[1]}"))
                fi

                if [ "${COMP_WORDS[1]}" = "activate" ]; then
                    COMPREPLY=($(compgen -W "$(venv-wrapper completions-helper-ls 2>&1)" -- "${COMP_WORDS[${COMP_CWORD}]}"))
                elif [ "${COMP_WORDS[1]}" = "link" ]; then
                    COMPREPLY=($(compgen -W "$(venv-wrapper completions-helper-ls 2>&1)" -- "${COMP_WORDS[${COMP_CWORD}]}"))
                elif [ "${COMP_WORDS[1]}" = "new" ]; then
                    if [ "${#COMP_WORDS[@]}" = "3" ]; then
                        COMPREPLY=($(compgen -W '-p --python' -- "${COMP_WORDS[${COMP_CWORD}]}"))
                    elif [ "${COMP_WORDS[2]}" = "-p" ] || [[ "${COMP_WORDS[2]}" == --python* ]]; then
                        if [ "${#COMP_WORDS[@]}" = "4" ]; then
                            COMPREPLY=($(compgen -W "$(venv-wrapper available-pythons-helper 2>&1)" -- "${COMP_WORDS[${COMP_CWORD}]}"))
                        fi
                    fi
                elif [ "${COMP_WORDS[1]}" = "rm" ]; then
                    COMPREPLY=($(compgen -W "$(venv-wrapper completions-helper-ls 2>&1)" -- "${COMP_WORDS[${COMP_CWORD}]}"))
                elif [ "${COMP_WORDS[1]}" = "unlink" ]; then
                    COMPREPLY=($(compgen -W "$(venv-wrapper completions-helper-ls 2>&1)" -- "${COMP_WORDS[${COMP_CWORD}]}"))
                elif [ "${COMP_WORDS[1]}" = "use" ]; then
                    COMPREPLY=($(compgen -W "$(venv-wrapper completions-helper-ls 2>&1)" -- "${COMP_WORDS[${COMP_CWORD}]}"))
                fi
            }

            complete -F _venv_wrapper_completions venv"#,
        )
    }

    fn fish_eval(&self) -> String {
        String::from(
            r#"
            complete -ec venv
            set -l venv_wrapper_commands activate help init link ls new rm unlink use
            set venv_wrapper_split_path (string split : $PATH)

            complete -f -c venv
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a activate -d "activate a virtualenv"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a help -d "show the help text"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a link -d "link a project directory with a virtualenv"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a ls -d "list available virtualenvs"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a new -d "create a new virtualenv"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a rm -d "remove an existing virtualenv"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a unlink -d "unlink a project directory from a virtualenv"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -a use -d "use a virtualenv (cd to the project and activate)"
            complete -f -c venv -n "not __fish_seen_subcommand_from $venv_wrapper_commands" -s r -l venv-root -a "(ls -ad */ .*/)" -d "set the virtualenv root"

            complete -f -c venv -n "__fish_seen_subcommand_from activate" -a "(string split ' ' (venv-wrapper completions-helper-ls 2>&1))"
            complete -f -c venv -n "__fish_seen_subcommand_from rm" -a "(string split ' ' (venv-wrapper completions-helper-ls 2>&1))"
            complete -f -c venv -n "__fish_seen_subcommand_from link" -a "(string split ' ' (venv-wrapper completions-helper-ls 2>&1))"
            complete -F -c venv -n "__fish_seen_subcommand_from link" -s p -l project -a "(__fish_complete_directories)"
            complete -f -c venv -n "__fish_seen_subcommand_from unlink" -a "(string split ' ' (venv-wrapper completions-helper-ls 2>&1))"
            complete -f -c venv -n "__fish_seen_subcommand_from use" -a "(string split ' ' (venv-wrapper completions-helper-ls 2>&1))"
            complete -f -c venv -n "__fish_seen_subcommand_from new" -s p -l python -a "(string split ' ' (venv-wrapper available-pythons-helper 2>&1))"

            set -e venv_wrapper_commands
            set -e venv_wrapper_split_path"#,
        )
    }

    fn zsh_eval(&self) -> String {
        String::from(
            r#"compdef 2>/dev/null || autoload -Uz compinit && compinit

            _venv_wrapper_completions () {{
                _arguments -C \
                    '-r[venv root]' '--venv-root[venv root]' \
                    '1: :((activate\:"activate a virtualenv"
                           help\:"show the help text"
                           link\:"link a project directory with a virtualenv"
                           ls\:"list available virtualenvs"
                           new\:"create a new virtualenv"
                           rm\:"remove an existing virtualenv"
                           unlink\:"unlink a project directory from a virtualenv"
                           use\:"use a virtualenv (cd to the project and activate)"))' \
                    '*::arg:->args'

                case "$line[1]" in
                    activate)
                        _describe 'available virtualenvs' "($(venv-wrapper completions-helper-ls 2>&1))"
                    ;;
                    help)
                        return 0
                    ;;
                    link)
                        _arguments \
                            '-p[project directory]:project directory:_path_files -/' \
                            '--python[project cirectory]:project directory:_path_files -/' \
                            ':virtualenv name:($(venv-wrapper completions-helper-ls 2>&1))'
                    ;;
                    ls)
                        return 0
                    ;;
                    new)
                        _arguments \
                            '-p[source python executable]:source python executable:($(venv-wrapper available-pythons-helper 2>&1))' \
                            '--python[source python executable]:source python executable:($(venv-wrapper available-pythons-helper 2>&1))' \
                            ':virtualenv name:()'
                    ;;
                    rm)
                        _describe 'available virtualenvs' "($(venv-wrapper completions-helper-ls 2>&1))"
                    ;;
                    unlink)
                        _describe 'available virtualenvs' "($(venv-wrapper completions-helper-ls 2>&1))"
                    ;;
                    use)
                        _describe 'available virtualenvs' "($(venv-wrapper completions-helper-ls 2>&1))"
                    ;;
                esac
            }}

            compdef _venv_wrapper_completions venv"#,
        )
    }
}
