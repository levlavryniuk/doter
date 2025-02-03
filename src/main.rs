// 0. Auto detect .config/doter or creat it

// 1. Auto detect .config/doter/index.toml or create it
// 2. Add entries using clap

mod consts;
mod marker_manager;
use std::fs::{create_dir_all, read_dir};
use std::io::ErrorKind;

use clap::{Arg, ArgMatches, Command as ClapCommand};
use consts::DOTER_DIR;
use marker_manager::MarkerManager;

fn detect_or_create_dir() {
    match read_dir(DOTER_DIR) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("Creating .config/doter");

                match create_dir_all(DOTER_DIR) {
                    Ok(_) => {
                        println!("Created .config/doter");
                    }
                    Err(e) => {
                        println!("Error creating .config/doter: {}", e);
                    }
                }
            }
        }
    }
}

fn create_command() -> ClapCommand {
    ClapCommand::new("doter")
        .version("0.1")
        .about("Dot file handler")
        .subcommand(ClapCommand::new("add").arg(Arg::new("path")))
}

struct AddCommand;

trait CommandHandler {
    fn handle(&self, matches: Option<&ArgMatches>, mgr: &mut MarkerManager);
    fn eq(&self, other: &str) -> bool;
    fn new() -> Box<Self>
    where
        Self: Sized;
}

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
}

fn handle_command(command: ClapCommand, mgr: &mut MarkerManager) {
    let matches = command.get_matches();
    let commands: Vec<Box<dyn CommandHandler>> = vec![AddCommand::new()];

    for command in commands {
        let subcommand = matches.subcommand_name().expect("Command not found");
        if command.eq(subcommand) {
            command.handle(matches.subcommand_matches(subcommand), mgr);
        }
    }
}

fn main() {
    detect_or_create_dir();
    let command = create_command();
    let mut mgr = MarkerManager::from_config();
    handle_command(command, &mut mgr);
}
