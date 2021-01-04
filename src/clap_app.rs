use clap::AppSettings;

pub fn get_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(venv =>
        (name: "venv-wrapper")
        (bin_name: "venv")
        (version: "0.1.0")
        (author: "Nicolas Marier <software@nmarier.com>")
        (about: "Virtualenvwrapper, but in rust!")

        (@subcommand init =>
            (about: "Prepare the shell for use")
        )
        (@subcommand ls =>
            (about: "Show a list of available virtualenvs")
            (@arg venvs_dir: !required +takes_value -d --("venvs-dir") "Sets where to search for virtualenvs [default: ~/.virtualenvs]")
        )
        (@subcommand new =>
            (about: "Create a new virtual environment")
            (@arg venvs_dir: !required +takes_value -d --("venvs-dir") "Sets where to create the virtualenv [default: ~/.virtualenvs]")
            (@arg venv_name: +required +takes_value "The name for the new virtualenv")
            (@arg python_executable: !required +takes_value default_value("python3") -p --python "Sets the python executable name for the virtualenv")
        )
    )
    .setting(AppSettings::ArgRequiredElseHelp)
}
