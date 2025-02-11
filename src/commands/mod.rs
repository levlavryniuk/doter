use clap::ArgMatches;
use clap::Command as ClapCommand;
use marker::MarkerCommand;
use origin_command::OriginCommand;
use sync_command::SyncCommand;

use crate::config;
use crate::marker_manager::MarkerManager;

pub mod marker;
pub mod origin_command;
pub mod sync_command;

pub struct Context<'a> {
    pub matches: &'a ArgMatches,
    pub mgr: &'a mut MarkerManager<'a>,
    pub cfg: &'a mut config::Config,
}

pub trait CommandHandler {
    fn handle(&self, ctx: Context);
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
            commands: vec![
                MarkerCommand::new(),
                OriginCommand::new(),
                SyncCommand::new(),
            ],
        }
    }

    pub fn handle<'a>(
        &self,
        matches: &'a ArgMatches,
        mgr: &'a mut MarkerManager<'a>,
        cfg: &'a mut config::Config,
    ) {
        if let Some(subcommand) = matches.subcommand_name() {
            if let Some(found_command) = self.commands.iter().find(|cmd| cmd.eq(subcommand)) {
                if let Some(subcommand_matches) = matches.subcommand_matches(subcommand) {
                    let ctx = Context {
                        mgr,
                        cfg,
                        matches: subcommand_matches,
                    };
                    found_command.handle(ctx);
                }
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
