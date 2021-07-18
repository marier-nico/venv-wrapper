use clap::ArgMatches;
use eyre::{eyre, Result};

use crate::{
    cli::{
        command::Command,
        commands::{
            activate_command::ActivateCommand,
            completions_command::CompletionsCommand,
            completions_helpers::{
                available_pythons_helper::AvailablePythonsHelper,
                ls_completion_helper::LsCompletionHelper,
            },
            init_command::InitCommand,
            link_command::LinkCommand,
            ls_command::LsCommand,
            new_command::NewCommand,
            rm_command::RmCommand,
            unlink_command::UnlinkCommand,
            use_command::UseCommand,
        },
    },
    config::config_data::Config,
};

use super::command::CommandResult;

pub fn cli_main(matches: &ArgMatches, config: &Config) -> Result<CommandResult> {
    match matches.subcommand() {
        ("activate", Some(sub_matches)) => ActivateCommand::run(config, sub_matches),
        ("completions", Some(sub_matches)) => CompletionsCommand::run(config, sub_matches),
        ("init", Some(sub_matches)) => InitCommand::run(config, sub_matches),
        ("link", Some(sub_matches)) => LinkCommand::run(config, sub_matches),
        ("ls", Some(sub_matches)) => LsCommand::run(config, sub_matches),
        ("new", Some(sub_matches)) => NewCommand::run(config, sub_matches),
        ("rm", Some(sub_matches)) => RmCommand::run(config, sub_matches),
        ("unlink", Some(sub_matches)) => UnlinkCommand::run(config, sub_matches),
        ("use", Some(sub_matches)) => UseCommand::run(config, sub_matches),
        ("available-pythons-helper", Some(sub_matches)) => {
            AvailablePythonsHelper::run(config, sub_matches)
        }
        ("completions-helper-ls", Some(sub_matches)) => {
            LsCompletionHelper::run(config, sub_matches)
        }
        _ => Err(eyre!("Unknown command")),
    }
}
