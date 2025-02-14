use crate::{config::DOTER_BOILERPLATE_REPO_URL, git};

use super::{CommandHandler, Context};

pub struct InitCommand;

impl CommandHandler for InitCommand {
    fn handle(&self, ctx: Context) {
        if ctx.cfg.has_initialised {
            eprintln!("You have already initialized, why would you do it again?")
        }

        git::clone(DOTER_BOILERPLATE_REPO_URL, &ctx.cfg.doter_dir_path)
            .expect("Failed to clone boilerplate code, check ~/.doter");
        git::init(&ctx.cfg.doter_remote_dir_path).expect("Failed to init git repo");

        ctx.cfg.set_initialized();
    }

    fn new() -> Box<Self> {
        Box::new(Self)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("init")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("init")
    }
}
