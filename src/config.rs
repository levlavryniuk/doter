use std::env;

pub struct Config {
    pub doter_dir_path: String,
    pub doter_file_path: String,
}

impl Config {
    pub fn load() -> Self {
        let doter_dir_path =
            env::var("HOME").expect("HOME not found").to_string() + "/.config/doter";
        let doter_file_path = format!("{}/{}", doter_dir_path, "index.toml");

        Self {
            doter_dir_path,
            doter_file_path,
        }
    }
}
