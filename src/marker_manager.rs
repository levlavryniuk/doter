use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
};

pub struct MarkerManager {
    pub markers: Vec<String>,
    file_path: String,
}

impl MarkerManager {
    pub fn from_config(doter_file_path: &str) -> MarkerManager {
        let entries = read_or_create_index(doter_file_path);
        MarkerManager {
            markers: entries,
            file_path: doter_file_path.to_string(),
        }
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
        let create = File::create(&self.file_path);
        let mut file = create.unwrap();
        for marker in &self.markers {
            let marker = format!("{}\n", marker);
            match file.write_all(marker.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error writing to file: {}", e);
                }
            };
        }
    }
}
fn read_or_create_index(path: &str) -> Vec<String> {
    let file = File::open(path);
    let mut buf = String::new();

    match file {
        Ok(mut file) => {
            file.read_to_string(&mut buf)
                .expect("Unable to read contents of ");
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                match File::create(path) {
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
