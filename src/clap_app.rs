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
    )
    .setting(AppSettings::ArgRequiredElseHelp)
}
