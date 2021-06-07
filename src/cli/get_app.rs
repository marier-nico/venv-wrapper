use clap::{clap_app, AppSettings};

pub fn get_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(venv =>
        (name: "venv-wrapper")
        (bin_name: "venv")
        (version: "1.0.0")
        (author: "Nicolas Marier <software@nmarier.com>")
        (about: "Python virtualenv management")
        (setting: AppSettings::ArgRequiredElseHelp)

        (@arg venvs_dir: !required +takes_value -d --("venvs-dir") "Set the location to store virtualenvs in [default: ~/.virtualenvs]")

        (@subcommand new =>
            (about: "Create a new virtual environment")
            (@arg name: +required +takes_value "The virtualenv's name")
            (@arg python_executable: !required +takes_value default_value("python3") -p --python "Set the python interpreter to use for the virtualenv (either provide a full path or ensure the interpreter is in your PATH)")
        )
    )
}
