use std::process::{Command, Output};

use crate::config::Config;

fn git() -> Command {
    Command::new("git")
}
pub fn clone(url: &str, dest: &str) -> Result<(), std::io::Error> {
    let output = git().args(["clone", url, dest]).output();
    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to clone: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )),
        Err(e) => Err(e),
    }
}
pub fn init(path: &str) -> Result<(), std::io::Error> {
    let output = git().current_dir(path).args(["init"]).output();
    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to initialize git repository: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )),
        Err(e) => Err(e),
    }
}

pub fn add(cfg: &Config, entry: &str) -> Result<(), std::io::Error> {
    let output = git()
        .current_dir(&cfg.doter_local_dir_path)
        .args(["add", entry])
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to add files to git repository: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )),
        Err(e) => Err(e),
    }
}

pub fn commit(cfg: &Config, message: &str) -> Result<(), std::io::Error> {
    let output = git()
        .current_dir(&cfg.doter_local_dir_path)
        .args(["commit", "-am", message])
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to commit in git repository: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )),
        Err(e) => Err(e),
    }
}
