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
        format!(
            r#"
            complete -r venv

            _venv_wrapper_completions() {{
                if [ "${{#COMP_WORDS[@]}}" = "2" ]; then
                    COMPREPLY=($(compgen -W "activate help link ls new rm unlink use" -- "${{COMP_WORDS[1]}}"))
                fi

                if [ "${{COMP_WORDS[1]}}" = "activate" ]; then
                    COMPREPLY=($(compgen -W "$(ls {0})" -- "${{COMP_WORDS[${{COMP_CWORD}}]}}"))
                elif [ "${{COMP_WORDS[1]}}" = "link" ]; then
                    COMPREPLY=($(compgen -W "$(ls {0})" -- "${{COMP_WORDS[${{COMP_CWORD}}]}}"))
                elif [ "${{COMP_WORDS[1]}}" = "new" ]; then
                    if [ "${{#COMP_WORDS[@]}}" = "3" ]; then
                        COMPREPLY=($(compgen -W '-p --python' -- "${{COMP_WORDS[${{COMP_CWORD}}]}}"))
                    elif [ "${{COMP_WORDS[2]}}" = "-p" ] || [[ "${{COMP_WORDS[2]}}" == --python* ]]; then
                        if [ "${{#COMP_WORDS[@]}}" = "4" ]; then
                            local venv_wrapper_split_path venv_wrapper_pythons
                            IFS=':' read -ra venv_wrapper_split_path <<< "$PATH"
                            venv_wrapper_pythons="$(\find ${{venv_wrapper_split_path[@]}} -executable -name 'python*' 2>/dev/null | \grep -o 'python\([0-9]\|[0-9]\.[0-9]\|[0-9]\.[0-9]\.[0-9]\)\?$' | sort -u)"

                            COMPREPLY=($(compgen -W "$venv_wrapper_pythons" -- "${{COMP_WORDS[${{COMP_CWORD}}]}}"))
                        fi
                    fi
                elif [ "${{COMP_WORDS[1]}}" = "rm" ]; then
                    COMPREPLY=($(compgen -W "$(ls {0})" -- "${{COMP_WORDS[${{COMP_CWORD}}]}}"))
                elif [ "${{COMP_WORDS[1]}}" = "unlink" ]; then
                    COMPREPLY=($(compgen -W "$(ls {0})" -- "${{COMP_WORDS[${{COMP_CWORD}}]}}"))
                elif [ "${{COMP_WORDS[1]}}" = "use" ]; then
                    COMPREPLY=($(compgen -W "$(ls {0})" -- "${{COMP_WORDS[${{COMP_CWORD}}]}}"))
                fi
            }}

            complete -F _venv_wrapper_completions venv"#,
            self.venv_dir.to_string_lossy()
        )
    }

    fn fish_eval(&self) -> String {
        format!(
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

            complete -f -c venv -n "__fish_seen_subcommand_from activate" -a "(ls {0})"
            complete -f -c venv -n "__fish_seen_subcommand_from rm" -a "(ls {0})"
            complete -f -c venv -n "__fish_seen_subcommand_from link" -a "(ls {0})"
            complete -F -c venv -n "__fish_seen_subcommand_from link" -s p -l project -a "(__fish_complete_directories)"
            complete -f -c venv -n "__fish_seen_subcommand_from unlink" -a "(ls {0})"
            complete -f -c venv -n "__fish_seen_subcommand_from use" -a "(ls {0})"
            complete -f -c venv -n "__fish_seen_subcommand_from new" -s p -l python -a "(command find $venv_wrapper_split_path -executable -name 'python*' 2>/dev/null | command grep -o 'python\([0-9]\|[0-9]\.[0-9]\|[0-9]\.[0-9]\.[0-9]\)\?\$' | sort -u)"

            set -e venv_wrapper_commands
            set -e venv_wrapper_split_path"#,
            self.venv_dir.to_string_lossy(),
        )
    }

    fn zsh_eval(&self) -> String {
        format!(
            r#"compdef 2>/dev/null || autoload -Uz compinit && compinit

            _venv_wrapper_python_versions () {{
                local venv_wrapper_split_path venv_wrapper_path_str
                venv_wrapper_path_str="$PATH"
                venv_wrapper_split_path=(${{(s[:])venv_wrapper_path_str}})

                _describe 'available python versions' "($(\find ${{venv_wrapper_split_path[@]}} -executable -name 'python*' 2>/dev/null | \grep -o 'python\([0-9]\|[0-9]\.[0-9]\|[0-9]\.[0-9]\.[0-9]\)\?$' | sort -u))"
            }}

            _venv_wrapper_available_virtualenvs () {{
                \find {} -mindepth 1 -maxdepth 1 -printf "%f "
            }}

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
                        _describe 'available virtualenvs' "($(_venv_wrapper_available_virtualenvs))"
                    ;;
                    help)
                        return 0
                    ;;
                    link)
                        _arguments \
                            '-p[project directory]:project directory:_path_files -/' \
                            '--python[project cirectory]:project directory:_path_files -/' \
                            ':virtualenv name:($(_venv_wrapper_available_virtualenvs))'
                    ;;
                    ls)
                        return 0
                    ;;
                    new)
                        _arguments \
                            '-p[source python executable]:source python executable:_venv_wrapper_python_versions' \
                            '--python[source python executable]:source python executable:_venv_wrapper_python_versions' \
                            ':virtualenv name:()'
                    ;;
                    rm)
                        _describe 'available virtualenvs' "($(_venv_wrapper_available_virtualenvs))"
                    ;;
                    unlink)
                        _describe 'available virtualenvs' "($(_venv_wrapper_available_virtualenvs))"
                    ;;
                    use)
                        _describe 'available virtualenvs' "($(_venv_wrapper_available_virtualenvs))"
                    ;;
                esac
            }}

            compdef _venv_wrapper_completions venv"#,
            self.venv_dir.to_string_lossy(),
        )
    }
}
