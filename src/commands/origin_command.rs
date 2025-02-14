use std::process::Command;

use super::{CommandHandler, Context};

pub struct OriginCommand;

impl CommandHandler for OriginCommand {
    fn handle(&self, ctx: Context) {
        let new_origin = ctx.matches.get_one::<String>("origin");
        if let Some(old_origin) = &ctx.cfg.github_repo_url {
            if !ctx.matches.get_flag("force") {
                println!("You have already set origin: {}", old_origin);
                return;
            }
        }

        let init_result = Command::new("git")
            .current_dir(&ctx.cfg.doter_local_dir_path)
            .args(["init"])
            .output();

        if let Err(e) = init_result {
            eprintln!("Failed to initialize git repository: {}", e);
            return;
        }

        let _ = Command::new("git")
            .current_dir(&ctx.cfg.doter_local_dir_path)
            .args(["remote", "add", "origin", new_origin.unwrap()])
            .output();

        if let Some(new_origin) = new_origin {
            ctx.cfg.set_origin(new_origin.to_string());
            println!("Origin set to: {}", new_origin);
        }
    }

    fn new() -> Box<Self> {
        Box::new(Self)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("origin")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("origin")
            .about("Set origin")
            .arg(clap::Arg::new("origin").required(true))
            .arg(
                clap::Arg::new("force")
                    .short('f')
                    .long("force")
                    .action(clap::ArgAction::SetTrue),
            )
    }
}
