use clap::AppSettings;

pub fn get_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(venv =>
        (name: "venv-wrapper")
        (bin_name: "venv")
        (version: "0.1.0")
        (author: "Nicolas Marier <software@nmarier.com>")
        (about: "Virtualenvwrapper, but in rust!")
        (setting: AppSettings::ArgRequiredElseHelp)

        (@arg venvs_dir: !required +takes_value -d --("venvs-dir") "Sets where to look for the virtualenv to activate [default: ~/.virtualenvs]")

        (@subcommand init =>
            (about: "Prepare the shell for use")
            (long_about: 
                "Output a shell function to STDOUT to allow venv-wrapper to modify the shell state. \
                The function that is output and evaluated essentially runs the venv-wrapper \
                binary and evals the contents of a file that is written to by venv-wrapper itself (to modify the shell's state). \
                Feel free to audit the function by running the `init` command without evaluating the result."
            )
        )
        (@subcommand ls =>
            (about: "Show a list of available virtualenvs")
        )
        (@subcommand new =>
            (about: "Create a new virtual environment")
            (@arg venv_name: +required +takes_value "The name for the new virtualenv")
            (@arg python_executable: !required +takes_value default_value("python3") -p --python "Sets the python executable name for the virtualenv (provided value must be in your path)")
        )
        (@subcommand activate =>
            (about: "Activate a virtualenv")
            (@arg venv_name: +required +takes_value "The name of the virtualenv to activate")
        )
        (@subcommand deactivate =>
            (about: "Deactivate the current virtualenv")
        )
        (@subcommand rm =>
            (about: "Remove a virtualenv")
            (long_about: 
                "Remove a virtualenv. The currently active virtualenv cannot be removed with this method. \
                You should deactivate the current virtualenv first, if that is the one you would like to remove. \
                Also, to make accidental data loss more difficult, only a directory containg a `bin` \
                folder, which itself contains an `activate` file will be removed by this command."
            )
            (@arg venv_name: +required +takes_value "The name of the virtualenv to delete")
        )
        (@subcommand project =>
            (about: "Manage projects associated with virtualenvs")
            (long_about:
                "Manage what working directory is associated with a virtualenv to quickly switch \
                between which project you are currently working on."
            )
            (setting: AppSettings::ArgRequiredElseHelp)
            (@subcommand link =>
                (about: "Link a project (directory) to a given virtualenv")
                (long_about:
                    "Associate a working directory with a virtualenv. Multiple runs of this command \
                    will overwrite previously set values."
                )
                (@arg project_dir: !required +takes_value -p --("project-dir") "The path to the project's directory [default: ./]")
                (@arg venv_name: +required +takes_value "The name of the virtualenv to link to a project")
            )
            (@subcommand unlink =>
                (about: "Remove an existing link to a project for a given virtualenv")
                (@arg venv_name: +required +takes_value "The name of the virtualenv to unlink with a project")
            )
        )
        (@subcommand use_command =>
            (name: "use")
            (about: "Use a virtualenv (activate it, and move to its directory)")
            (long_about: 
                "Use a virtualenv (begin working on the project associated with it, if any). \
                If no project is associated with the virtualenv, it is only activated. \
                If a project is associated with it, the current shell will move to the project's \
                directory and the virtualenv will be activated."
            )
            (@arg venv_name: +required +takes_value "The name of the virtualenv to use")
        )
    )
}
