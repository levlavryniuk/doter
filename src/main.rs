// 0. Auto detect .config/doter or creat it

// 1. Auto detect .config/doter/index.toml or create it
// 2. Add entries using clap

mod commands;
mod config;
mod git_manager;
mod marker_manager;
use std::fs::{create_dir_all, read_dir};
use std::io::ErrorKind;

use marker_manager::MarkerManager;

fn detect_or_create_dir(dir: &str) {
    match read_dir(dir) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("Creating .config/doter");

                match create_dir_all(dir) {
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

fn main() {
    let cfg = config::Config::load();
    detect_or_create_dir(&cfg.doter_dir_path);

    let mut mgr = MarkerManager::from_config(&cfg.doter_file_path);

    let command_mgr = commands::CommandManager::new();
    let command = command_mgr.create_command();

    let matches = command.get_matches();
    command_mgr.handle(&matches, &mut mgr);
}
