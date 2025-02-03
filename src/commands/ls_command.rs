use clap::ArgMatches;

use crate::marker_manager::MarkerManager;

use super::CommandHandler;

pub struct LsCommand;

impl CommandHandler for LsCommand {
    fn handle(&self, _matches: Option<&ArgMatches>, mgr: &mut MarkerManager) {
        for marker in &mgr.markers {
            println!("{}", marker);
        }
    }

    fn new() -> Box<Self> {
        Box::new(Self)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("add")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("ls")
    }
}
