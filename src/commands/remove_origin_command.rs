use super::{CommandHandler, Context};

pub struct RemoveOriginCommand;

impl CommandHandler for RemoveOriginCommand {
    fn handle(&self, ctx: Context) {
        ctx.cfg.remove_origin();
    }

    fn new() -> Box<Self> {
        Box::new(Self)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("bob")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("bob")
    }
}
