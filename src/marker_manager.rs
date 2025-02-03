use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
};

use crate::consts::DOTER_FILE_PATH;

pub struct MarkerManager {
    markers: Vec<String>,
}

impl MarkerManager {
    pub fn from_config() -> MarkerManager {
        let entries = read_or_create_index();
        MarkerManager { markers: entries }
    }

    pub fn add_marker(&mut self, marker: String) {
        self.markers.push(marker);
        self.update_index();
    }

    pub fn remove_marker(&mut self, marker: String) {
        self.markers.retain(|e| e != &marker);
        self.update_index();
    }

    fn update_index(&self) {
        let create = File::create(DOTER_FILE_PATH);
        let mut file = create.unwrap();
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
