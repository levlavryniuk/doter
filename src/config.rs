use std::{
    default, env,
    fs::{create_dir_all, read_dir, File},
    io::ErrorKind,
};

use rusqlite::Connection;

#[derive(Clone, Debug)]
pub struct Config {
    pub doter_dir_path: String,
    pub doter_file_path: String,
    pub github_repo_url: Option<String>,
    pub doter_saves_dir_path: String,
}

impl Config {
    pub fn new() -> Self {
        let doter_dir_path = env::var("HOME").expect("HOME not found").to_string() + "/.doter";
        let doter_file_path = format!("{}/{}", doter_dir_path, "index.db");

        let doter_saves_path = format!("{}/{}", doter_dir_path, "saves");
        let cfg = Self {
            doter_dir_path,
            doter_file_path,
            doter_saves_dir_path: doter_saves_path,
            github_repo_url: None,
        };
        cfg.detect_or_create_dir();
        let _ = File::create_new(&cfg.doter_file_path);

        cfg
    }

    pub fn set_origin(&mut self, url: String) {
        self.github_repo_url = Some(url.clone());

        let conn = Connection::open(&self.doter_file_path).unwrap();
        conn.execute("delete from variables where name = 'github_repo_url'", [])
            .expect("BAD QUERY: total skill issue from devs");
        conn.execute(
            "insert into variables (name, value) values ('github_repo_url', ?)",
            [&url],
        )
        .expect("BAD QUERY: total skill issue from devs");
    }

    pub fn load_vars(&mut self, conn: &Connection) {
        let query = "select name, value from variables";
        let mut stmt = conn.prepare(query).unwrap();
        let mut vars: Vec<(String, String)> = Vec::new();
        for row in stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
        {
            if let Ok((name, value)) = row {
                vars.push((name, value));
            }
        }
        for (name, value) in vars {
            match name.as_str() {
                "github_repo_url" => {
                    self.github_repo_url = Some(value);
                }
                _ => {}
            }
        }
    }

    pub fn detect_or_create_dir(&self) {
        match read_dir(&self.doter_saves_dir_path) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    if let Err(e) = create_dir_all(&self.doter_dir_path) {
                        eprintln!("Error creating {}: {}", self.doter_dir_path, e);
                    }
                    if let Err(e) = create_dir_all(&self.doter_saves_dir_path) {
                        eprintln!("Error creating {}: {}", self.doter_saves_dir_path, e);
                    }
                }
            }
        }
    }
}
