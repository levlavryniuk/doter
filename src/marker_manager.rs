use std::fs;

use rusqlite::Connection;

use crate::config::Config;

#[derive(Debug, PartialEq)]
pub struct Marker {
    pub source_location: String,
    pub copy_location: String,
    pub alias: String,
}

pub struct MarkerManager<'a> {
    conn: &'a Connection,
    cfg: &'a Config,
}

// nvim|/home/username/.config/nvim

impl<'a> MarkerManager<'a> {
    pub fn new(conn: &'a Connection, cfg: &'a Config) -> MarkerManager<'a> {
        MarkerManager { conn, cfg }
    }

    pub fn add_marker(&self, source_marker: String) {
        let mrk = self.create_marker(source_marker);

        let query = "insert into markers (alias, source_location, copy_location) values (?, ?, ?)";

        let mut stmt = self
            .conn
            .prepare(query)
            .expect("BAD QUERY: total skill issue from devs");

        match stmt.execute([&mrk.alias, &mrk.source_location, &mrk.copy_location]) {
            Ok(_) => {
                println!("Added marker: {}", mrk.alias);
            }
            Err(e) => {
                println!("Error adding marker: {}", e);
            }
        }
    }

    fn create_marker(&self, source_marker: String) -> Marker {
        let path = fs::canonicalize(&source_marker).expect("Invalid path");

        let name = path
            .file_name()
            .expect("Invalid path")
            .to_string_lossy()
            .into_owned();

        let copy_location = format!("{}/{}", self.cfg.doter_saves_dir_path.clone(), &name);

        Marker {
            source_location: source_marker.to_string(),
            copy_location,
            alias: name.to_string(),
        }
    }

    pub fn get_markers(&self) -> Vec<Marker> {
        let mut stmt = self
            .conn
            .prepare("select * from markers")
            .expect("BAD QUERY: total skill issue from devs");

        let mut markers = Vec::new();
        for row in stmt
            .query_map([], |row| {
                Ok(Marker {
                    alias: row.get(0)?,
                    source_location: row.get(1)?,
                    copy_location: row.get(2)?,
                })
            })
            .expect("BAD QUERY: total skill issue from devs")
        {
            if let Ok(mrk) = row {
                markers.push(mrk);
            }
        }
        markers
    }

    pub fn remove_all_markers(&self) {
        let mut stmt = self
            .conn
            .prepare("delete from markers")
            .expect("BAD QUERY: total skill issue from devs");

        stmt.execute([])
            .expect("BAD QUERY: total skill issue from devs");
    }
    pub fn remove_marker(&self, alias: &str) {
        let mut stmt = self
            .conn
            .prepare("delete from markers where alias = ?")
            .expect("BAD QUERY: total skill issue from devs");

        stmt.execute([&alias])
            .expect("BAD QUERY: total skill issue from devs");
    }
}
