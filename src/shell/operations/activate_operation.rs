use crate::{shell::shell_operation::ShellOperation, virtualenv::virtualenv_info::Virtualenv};

pub struct ActivateOperation {
    venv: Virtualenv,
}

impl ActivateOperation {
    pub fn new(venv: Virtualenv) -> Self {
        ActivateOperation {
            venv,
        }
    }
}

// Taken from: https://github.com/python/cpython/blob/main/Lib/venv/scripts/posix/activate.fish
impl ShellOperation for ActivateOperation {
    fn bash_eval(&self) -> String {
        format!(
            r#"
deactivate () {{
    # reset old environment variables
    if [ -n "${{_OLD_VIRTUAL_PATH:-}}" ] ; then
        PATH="${{_OLD_VIRTUAL_PATH:-}}"
        export PATH
        unset _OLD_VIRTUAL_PATH
    fi
    if [ -n "${{_OLD_VIRTUAL_PYTHONHOME:-}}" ] ; then
        PYTHONHOME="${{_OLD_VIRTUAL_PYTHONHOME:-}}"
        export PYTHONHOME
        unset _OLD_VIRTUAL_PYTHONHOME
    fi

    # This should detect bash and zsh, which have a hash command that must
    # be called to get it to forget past commands.  Without forgetting
    # past commands the $PATH changes we made may not be respected
    if [ -n "${{BASH:-}}" -o -n "${{ZSH_VERSION:-}}" ] ; then
        hash -r 2> /dev/null
    fi

    if [ -n "${{_OLD_VIRTUAL_PS1:-}}" ] ; then
        PS1="${{_OLD_VIRTUAL_PS1:-}}"
        export PS1
        unset _OLD_VIRTUAL_PS1
    fi

    unset VIRTUAL_ENV
    if [ ! "${{1:-}}" = "nondestructive" ] ; then
    # Self destruct!
        unset -f deactivate
    fi
}}

# unset irrelevant variables
deactivate nondestructive

VIRTUAL_ENV="{}"
export VIRTUAL_ENV

_OLD_VIRTUAL_PATH="$PATH"
PATH="{}:$PATH"
export PATH

# unset PYTHONHOME if set
# this will fail if PYTHONHOME is set to the empty string (which is bad anyway)
# could use `if (set -u; : $PYTHONHOME) ;` in bash
if [ -n "${{PYTHONHOME:-}}" ] ; then
    _OLD_VIRTUAL_PYTHONHOME="${{PYTHONHOME:-}}"
    unset PYTHONHOME
fi

if [ -z "${{VIRTUAL_ENV_DISABLE_PROMPT:-}}" ] ; then
    _OLD_VIRTUAL_PS1="${{PS1:-}}"
    PS1="({}) ${{PS1:-}}"
    export PS1
fi

# This should detect bash and zsh, which have a hash command that must
# be called to get it to forget past commands.  Without forgetting
# past commands the $PATH changes we made may not be respected
if [ -n "${{BASH:-}}" -o -n "${{ZSH_VERSION:-}}" ] ; then
    hash -r 2> /dev/null
fi
            "#,
            self.venv.path().to_string_lossy(),
            self.venv.bin_path().to_string_lossy(),
            self.venv.name,
        )
    }

    fn fish_eval(&self) -> String {
        format!(
            r#"
function deactivate  -d "Exit virtual environment and return to normal shell environment"
    # reset old environment variables
    if test -n "$_OLD_VIRTUAL_PATH"
        set -gx PATH $_OLD_VIRTUAL_PATH
        set -e _OLD_VIRTUAL_PATH
    end
    if test -n "$_OLD_VIRTUAL_PYTHONHOME"
        set -gx PYTHONHOME $_OLD_VIRTUAL_PYTHONHOME
        set -e _OLD_VIRTUAL_PYTHONHOME
    end

    if test -n "$_OLD_FISH_PROMPT_OVERRIDE"
        functions -e fish_prompt
        set -e _OLD_FISH_PROMPT_OVERRIDE
        functions -c _old_fish_prompt fish_prompt
        functions -e _old_fish_prompt
    end

    set -e VIRTUAL_ENV
    if test "$argv[1]" != "nondestructive"
        # Self-destruct!
        functions -e deactivate
    end
end

# Unset irrelevant variables.
deactivate nondestructive

set -gx VIRTUAL_ENV "{}"

set -gx _OLD_VIRTUAL_PATH $PATH
set -gx PATH "{}" $PATH

# Unset PYTHONHOME if set.
if set -q PYTHONHOME
    set -gx _OLD_VIRTUAL_PYTHONHOME $PYTHONHOME
    set -e PYTHONHOME
end

if test -z "$VIRTUAL_ENV_DISABLE_PROMPT"
    # fish uses a function instead of an env var to generate the prompt.

    # Save the current fish_prompt function as the function _old_fish_prompt.
    functions -c fish_prompt _old_fish_prompt

    # With the original prompt function renamed, we can override with our own.
    function fish_prompt
        # Save the return status of the last command.
        set -l old_status $status

        # Output the venv prompt; color taken from the blue of the Python logo.
        printf "%s%s%s" (set_color 4B8BBE) "({}) " (set_color normal)

        # Restore the return status of the previous command.
        echo "exit $old_status" | .
        # Output the original/"old" prompt.
        _old_fish_prompt
    end

    set -gx _OLD_FISH_PROMPT_OVERRIDE "$VIRTUAL_ENV"
end
        "#,
            self.venv.path().to_string_lossy(),
            self.venv.bin_path().to_string_lossy(),
            self.venv.name
        )
    }

    fn zsh_eval(&self) -> String {
        self.bash_eval()
    }
}
