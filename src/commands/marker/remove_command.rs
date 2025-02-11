use super::add_command::copy_dir_recursive;
use super::{CommandHandler, Context};
use clap::Arg;
use std::fs;
use std::path::Path;

pub struct RemoveCommand;

impl CommandHandler for RemoveCommand {
    fn handle(&self, ctx: Context) {
        if ctx.matches.get_flag("all") {
            let markers = ctx.mgr.get_markers();
            for marker in markers {
                remove_single_marker(&marker, ctx.matches.get_flag("keep-files"));
            }
            ctx.mgr.remove_all_markers();
            println!("Removed all markers");
            return;
        }

        let alias = match ctx.matches.get_one::<String>("alias") {
            Some(alias) => alias,
            None => return,
        };

        let markers = ctx.mgr.get_markers();
        if let Some(marker) = markers.iter().find(|m| m.alias == *alias) {
            remove_single_marker(marker, ctx.matches.get_flag("keep-files"));
            ctx.mgr.remove_marker(alias);
            println!("Removed marker: {}", alias);
        } else {
            println!("Marker not found: {}", alias);
        }
    }

    fn new() -> Box<Self> {
        Box::new(Self)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("rm")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("rm")
            .about("Remove a marker")
            .arg_required_else_help(true)
            .arg(
                Arg::new("alias")
                    .required(true)
                    .help("The marker to remove"),
            )
            .arg(
                Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Remove all markers")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("alias"),
            )
            .arg(
                Arg::new("keep-files")
                    .short('k')
                    .long("keep-files")
                    .help("Keep files in doter directory")
                    .action(clap::ArgAction::SetTrue),
            )
    }
}

fn remove_single_marker(marker: &crate::marker_manager::Marker, keep_files: bool) {
    let source_path = Path::new(&marker.source_location);
    let doter_path = Path::new(&marker.copy_location);

    if source_path.exists() && source_path.is_symlink() {
        if let Err(e) = fs::remove_file(source_path) {
            eprintln!(
                "Failed to remove symlink at {}: {}",
                source_path.display(),
                e
            );
            return;
        }
    }

    if doter_path.exists() && !keep_files {
        if doter_path.is_dir() {
            match copy_dir_recursive(doter_path, source_path) {
                Ok(_) => {
                    let _ = fs::remove_dir_all(doter_path);
                }
                Err(e) => eprintln!("Failed to move directory back: {}", e),
            }
        } else {
            match fs::copy(doter_path, source_path) {
                Ok(_) => {
                    let _ = fs::remove_file(doter_path);
                }
                Err(e) => eprintln!("Failed to move file back: {}", e),
            }
        }
    }
}
