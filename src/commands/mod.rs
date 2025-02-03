use add_command::AddCommand;
use clap::ArgMatches;
use clap::Command as ClapCommand;
use ls_command::LsCommand;

use crate::marker_manager::MarkerManager;

pub mod add_command;
pub mod ls_command;

pub trait CommandHandler {
    fn handle(&self, matches: Option<&ArgMatches>, mgr: &mut MarkerManager);
    fn eq(&self, other: &str) -> bool;
    fn to_clap(&self) -> ClapCommand;
    fn new() -> Box<Self>
    where
        Self: Sized;
}

pub struct CommandManager {
    commands: Vec<Box<dyn CommandHandler>>,
}

impl CommandManager {
    pub fn new() -> Self {
        CommandManager {
            commands: vec![AddCommand::new(), LsCommand::new()],
        }
    }

    pub fn handle(&self, matches: &ArgMatches, mgr: &mut MarkerManager) {
        for command in &self.commands {
            let subcommand = matches.subcommand_name().expect("Command not found");
            if command.eq(subcommand) {
                command.handle(matches.subcommand_matches(subcommand), mgr);
            }
        }
    }

    pub fn create_command(&self) -> ClapCommand {
        let commands: Vec<ClapCommand> = self.commands.iter().map(|c| c.to_clap()).collect();
        ClapCommand::new("doter")
            .version("0.1")
            .about("Dot file handler")
            .subcommands(commands)
    }
}
