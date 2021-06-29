use clap::{clap_app, AppSettings};

pub fn get_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(venv =>
        (name: "venv-wrapper")
        (bin_name: "venv")
        (version: "1.0.0")
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

        (@subcommand ls =>
            (about: "Show a list of available virtualenvs")
        )

        (@subcommand new =>
            (about: "Create a new virtual environment")
            (@arg name: +required +takes_value "The virtualenv's name")
            (@arg python_executable: !required +takes_value default_value("python3") -p --python "Set the python interpreter to use for the virtualenv (either provide a full path or ensure the interpreter is in your PATH)")
        )
    )
}
