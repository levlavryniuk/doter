// 0. Auto detect .config/doter or creat it
// 1. Auto detect .config/doter/index.toml or create it
// 2. Add entries using clap

use std::fs::{create_dir_all, read_dir, File};
use std::io::{ErrorKind, Read, Write};

use clap::{Arg, ArgMatches, Command as ClapCommand};

const DOTER_DIR: &str = ".config/doter";
const DOTER_FILE: &str = "index.toml";
const DOTER_FILE_PATH: &str = ".config/doter/index.toml";

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

fn read_or_create_index() -> Vec<String> {
    let file = File::open(DOTER_FILE_PATH);
    let mut buf = String::new();

    match file {
        Ok(mut file) => {
            file.read_to_string(&mut buf)
                .expect("Unable to read contents of ");
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                match File::create(DOTER_FILE_PATH) {
                    Ok(_) => {
                        println!("Created .config/doter/index.toml")
                    }
                    Err(_) => {
                        println!("Failed to create .config/doter/index.toml")
                    }
                }
            }
        }
    }

    buf.split_whitespace()
        .map(|e| {
            println!("{e}");
            e.to_string()
        })
        .collect()
}

struct AddCommand;

trait CommandHandler {
    fn handle(&self, matches: &ArgMatches, mgr: &mut MarkerManager);
    fn eq(&self, other: &str) -> bool;
    fn new() -> Box<Self>
    where
        Self: Sized;
}

struct MarkerManager {
    markers: Vec<String>,
}

impl MarkerManager {
    fn from_config() -> MarkerManager {
        let entries = read_or_create_index();
        MarkerManager { markers: entries }
    }

    fn add_marker(&mut self, marker: String) {
        self.markers.push(marker);
        self.update_index();
    }

    fn remove_marker(&mut self, marker: String) {
        self.markers.retain(|e| e != &marker);
        self.update_index();
    }

    fn update_index(&self) {
        let mut file = File::create(DOTER_FILE_PATH).unwrap();
        for marker in &self.markers {
            match file.write_all(marker.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error writing to file: {}", e);
                }
            };
        }
    }
}

impl CommandHandler for AddCommand {
    fn handle(&self, matches: &ArgMatches, mgr: &mut MarkerManager) {
        let path = matches.get_one::<String>("path");
        if let Some(path) = path {
            mgr.add_marker(path.into());
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
        if command.eq(matches.subcommand_name().unwrap()) {
            command.handle(&matches, mgr);
        }
    }
}

fn main() {
    detect_or_create_dir();
    let command = create_command();
    let mut mgr = MarkerManager::from_config();
    handle_command(command, &mut mgr);
}
