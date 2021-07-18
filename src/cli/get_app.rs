use clap::{clap_app, AppSettings};

pub fn get_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(venv =>
        (name: "venv-wrapper")
        (bin_name: "venv")
        (version: "2.0.0")
        (author: "Nicolas Marier <software@nmarier.com>")
        (about: "Python virtualenv management")
        (setting: AppSettings::ArgRequiredElseHelp)

        (@arg venv_root: !required +takes_value -r --("venv-root") "Set the location to store virtualenvs in [default: ~/.virtualenvs]")

        (@subcommand activate =>
            (about: "Activate a virtualenv")
            (@arg name: +required +takes_value "The name of the virtualenv to activate")
        )

        (@subcommand completions =>
            (about: "Output completions for your shell")
        )

        (@subcommand init =>
            (about: "Initialize your shell to use venv-wrapper")
            (@arg shell: +required +takes_value possible_values(&["bash", "fish", "zsh"]) "The name of the shell you're using")
        )

        (@subcommand link =>
            (about: "Link a virtualenv with a project directory")
            (@arg name: +required +takes_value "The virtualenv's name")
            (@arg project: -p --project +takes_value "The path to the project you want to link [default: current dir]")
        )

        (@subcommand ls =>
            (about: "Show a list of available virtualenvs")
        )

        (@subcommand new =>
            (about: "Create a new virtual environment")
            (@arg name: +required +takes_value "The virtualenv's name")
            (@arg python_executable: !required +takes_value default_value("python3") -p --python "Set the python interpreter to use for the virtualenv (either provide a full path or ensure the interpreter is in your PATH)")
        )

        (@subcommand rm =>
            (about: "Delete a virtual environment and its contents")
            (@arg name: +required +takes_value "The virtualenv's name")
        )

        (@subcommand unlink =>
            (about: "Unlink a virtualenv from a project directory")
            (@arg name: +required +takes_value "The virtualenv's name")
        )

        (@subcommand use_command =>
            (name: "use")
            (about: "Use a virtualenv (activate it, and cd to the linked project if there is one)")
            (@arg name: +required +takes_value "The virtualenv's name")
        )

        (@subcommand available_pythons_helper =>
            (name: "available-pythons-helper")
            (about: "Return the list of available python versions in an easily parsable format")
            (setting: AppSettings::Hidden)
        )

        (@subcommand completions_helper_ls =>
            (name: "completions-helper-ls")
            (about: "Return the list of virtualenvs in an easily parsable format")
            (setting: AppSettings::Hidden)
        )

    )
}
