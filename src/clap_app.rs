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
            (@arg venv_name: +required +takes_value "The name of the virtualenv to delete")
        )
        (@subcommand project =>
            (about: "Manage projects associated with virtualenvs")
            (setting: AppSettings::ArgRequiredElseHelp)
            (@subcommand link =>
                (about: "Link a project (directory) to a given virtualenv")
                (@arg project_dir: !required +takes_value -p --("project-dir") "The path to the project's directory [default: ./]")
                (@arg venv_name: +required +takes_value "The name of the virtualenv to link to a project")
            )
            (@subcommand unlink =>
                (about: "Remove an existing link to a project for a given virtualenv")
                (@arg venv_name: +required +takes_value "The name of the virtualenv to unlink with a project")
            )
        )
    )
}
