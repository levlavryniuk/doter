use clap::ArgMatches;

use crate::marker_manager::MarkerManager;

use super::CommandHandler;

pub struct AddCommand;

impl CommandHandler for AddCommand {
    fn handle(&self, matches: Option<&ArgMatches>, mgr: &mut MarkerManager) {
        match matches {
            None => {
                println!("No matches found");
            }
            Some(matches) => {
                let path = matches.get_one::<String>("path");
                if let Some(path) = path {
                    mgr.add_marker(path.into());
                }
            }
        }
    }

    fn new() -> Box<Self> {
        Box::new(AddCommand)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("add")
    }
    fn to_clap(&self) -> clap::Command {
        clap::Command::new("add")
            .about("Add a file to the doter file")
            .arg(clap::Arg::new("path").required(true))
    }
}
