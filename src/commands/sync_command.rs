use crate::marker_manager::Marker;

use super::{CommandHandler, Context};
use std::process::Command;

pub struct SyncCommand;

impl CommandHandler for SyncCommand {
    fn handle(&self, ctx: Context) {
        let github_url = match &ctx.cfg.github_repo_url {
            Some(url) => url,
            None => {
                eprintln!("GitHub repository URL not set. Use 'doter origin' to set it.");
                return;
            }
        };

        let force = ctx.matches.get_flag("force");

        let markers = ctx.mgr.get_markers();
        if markers.is_empty() {
            println!("No markers found to sync");
            return;
        }

        let init_result = Command::new("git")
            .current_dir(&ctx.cfg.doter_dir_path)
            .args(["init"])
            .output();

        if let Err(e) = init_result {
            eprintln!("Failed to initialize git repository: {}", e);
            return;
        }

        let _ = Command::new("git")
            .current_dir(&ctx.cfg.doter_dir_path)
            .args(["remote", "add", "origin", github_url])
            .output();

        let markers_to_sync = if ctx.matches.get_flag("all") {
            markers
        } else {
            let target = match ctx.matches.get_one::<String>("target") {
                Some(t) => t,
                None => {
                    eprintln!("No target specified and --all not used");
                    return;
                }
            };

            let filtered: Vec<Marker> =
                markers.into_iter().filter(|m| m.alias == *target).collect();

            if filtered.is_empty() {
                eprintln!("Target '{}' not found in markers", target);
                return;
            }
            filtered
        };

        for marker in markers_to_sync {
            println!("Syncing {}...", marker.alias);

            let add_result = Command::new("git")
                .current_dir(&ctx.cfg.doter_saves_dir_path)
                .args(["add", &marker.alias])
                .output();

            match add_result {
                Ok(output) if !output.status.success() => {
                    eprintln!("Failed to add {} to git", marker.alias);
                    eprintln!("{}", String::from_utf8_lossy(&output.stdout));
                    continue;
                }
                Err(e) => {
                    eprintln!("Error adding {} to git: {}", marker.alias, e);
                    continue;
                }
                _ => {}
            }

            let status_result = Command::new("git")
                .current_dir(&ctx.cfg.doter_dir_path)
                .args(["status", "--porcelain", &marker.alias])
                .output();

            match status_result {
                Ok(output) if output.stdout.is_empty() => {
                    println!("No changes to sync for {}", marker.alias);
                    if force {
                        println!("Forcing sync for {}", marker.alias);
                    } else {
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!("Error checking status for {}: {}", marker.alias, e);
                    continue;
                }
                _ => {}
            }

            let commit_result = Command::new("git")
                .current_dir(&ctx.cfg.doter_dir_path)
                .args(["commit", "-m", &format!("Update {}", marker.alias)])
                .output();

            match commit_result {
                Ok(output) if !output.status.success() => {
                    eprintln!("Failed to commit changes for {}", marker.alias);
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    continue;
                }
                Err(e) => {
                    eprintln!("Error committing {}: {}", marker.alias, e);
                    continue;
                }
                _ => {}
            }

            println!("Successfully committed changes for {}", marker.alias);
        }

        let push_result = Command::new("git")
            .current_dir(&ctx.cfg.doter_dir_path)
            .args(["push", "-u", "origin", "master"])
            .output();

        match push_result {
            Ok(output) if output.status.success() => {
                println!("Successfully pushed all changes to GitHub");
            }
            Ok(_) => {
                eprintln!("Failed to push changes to GitHub");
            }
            Err(e) => {
                eprintln!("Error pushing to GitHub: {}", e);
            }
        }
    }

    fn new() -> Box<Self> {
        Box::new(SyncCommand)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("sync")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("sync")
            .about("Sync dotfiles to GitHub")
            .arg(
                clap::Arg::new("target")
                    .help("Target alias to sync")
                    .required_unless_present("all"),
            )
            .arg(
                clap::Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Sync all markers")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("target"),
            )
            .arg(
                clap::Arg::new("force")
                    .short('f')
                    .long("force")
                    .help("Force push to origin")
                    .action(clap::ArgAction::SetTrue),
            )
    }
}
