use super::{CommandHandler, Context};

pub struct SyncCommand;

impl CommandHandler for SyncCommand {
    fn handle(&self, ctx: Context) {
        if ctx.cfg.github_repo_url.is_none() {
            eprintln!("Github repo url is required. Use origin command to set it");
            return;
        }
    }

    fn new() -> Box<Self> {
        Box::new(SyncCommand)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("sync")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("pull")
            .about("Sync dotfiles to GitHub")
            .arg(
                clap::Arg::new("target")
                    .help("Target alias to pull")
                    .required_unless_present("all"),
            )
            .arg(
                clap::Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Pull all markers")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("target"),
            )
            .arg(
                clap::Arg::new("force")
                    .short('f')
                    .long("force")
                    .help("Force pull from origin with override")
                    .action(clap::ArgAction::SetTrue),
            )
    }
}
