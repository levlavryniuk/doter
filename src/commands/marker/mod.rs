use add_command::AddCommand;
use ls_command::LsCommand;
use remove_command::RemoveCommand;

use super::{CommandHandler, Context};

pub struct MarkerCommand {
    commands: Vec<Box<dyn CommandHandler>>,
}
mod add_command;
mod ls_command;
mod remove_command;

impl CommandHandler for MarkerCommand {
    fn handle(&self, ctx: Context) {
        if let Some(subcommand) = ctx.matches.subcommand_name() {
            if let Some(found_command) = self.commands.iter().find(|cmd| cmd.eq(subcommand)) {
                if let Some(subcommand_matches) = ctx.matches.subcommand_matches(subcommand) {
                    let ctx = Context {
                        cfg: ctx.cfg,
                        mgr: ctx.mgr,
                        matches: subcommand_matches,
                    };
                    found_command.handle(ctx);
                }
            }
        }
    }
    fn new() -> Box<Self> {
        Box::new(Self {
            commands: vec![LsCommand::new(), AddCommand::new(), RemoveCommand::new()],
        })
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("marker")
    }

    fn to_clap(&self) -> clap::Command {
        let commands: Vec<clap::Command> = self.commands.iter().map(|cmd| cmd.to_clap()).collect();
        clap::Command::new("marker").subcommands(commands)
    }
}
