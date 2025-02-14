use super::{CommandHandler, Context};
use std::fs;
use std::path::Path;

pub struct AddCommand;

impl CommandHandler for AddCommand {
    fn handle(&self, ctx: Context) {
        let path = ctx
            .matches
            .get_one::<String>("path")
            .expect("Path is required by clap");

        ctx.mgr.add_marker(path.to_string());

        let markers = ctx.mgr.get_markers();
        let marker = markers
            .iter()
            .find(|m| m.source_location == *path)
            .expect("Marker should exist after adding");

        let source_path = Path::new(&marker.source_location);
        let dest_path = Path::new(&marker.copy_location);

        println!("Source path: {}", source_path.display());
        println!("Destination path: {}", dest_path.display());

        if source_path.exists() {
            if source_path.is_dir() {
                copy_dir_recursive(&source_path, &dest_path).expect("Failed to copy directory");
                fs::remove_dir_all(&source_path).expect("Failed to remove original directory");
            } else {
                fs::copy(&source_path, &dest_path).expect("Failed to copy file");
                fs::remove_file(&source_path).expect("Failed to remove original file");
            }
        } else {
            println!("Warning: Source path does not exist, creating new at target location");
            if source_path.to_string_lossy().ends_with('/') {
                fs::create_dir_all(&dest_path).expect("Failed to create directory");
            } else {
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent).expect("Failed to create parent directories");
                }
                fs::write(&dest_path, "").expect("Failed to create empty file");
            }
        }

        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directories");
        }

        std::os::unix::fs::symlink(&dest_path, &source_path).expect("Failed to create symlink");

        println!("Added {} to doter", marker.alias);
        println!(
            "Created symlink from {} to {}",
            source_path.display(),
            dest_path.display()
        );
    }

    fn new() -> Box<Self> {
        Box::new(AddCommand)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("add")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("add")
            .about("Add a marker")
            .arg_required_else_help(true)
            .arg(
                clap::Arg::new("path")
                    .required(true)
                    .help("The marker to add"),
            )
    }
}
pub fn copy_dir_recursive(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && path.file_name().map_or(false, |name| name == ".git") {
            continue;
        }

        let dest_path = destination.join(path.file_name().unwrap());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}
