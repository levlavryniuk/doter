use super::{CommandHandler, Context};

pub struct LsCommand;

impl CommandHandler for LsCommand {
    // lets make the prints prettier
    fn handle(&self, ctx: Context) {
        let markers = ctx.mgr.get_markers();

        let mut max_width = 0;
        if markers.is_empty() {
            println!("No markers found");
            return;
        }
        for (idx, marker) in markers.iter().enumerate() {
            let line = format!("{}. {} ~ {}", idx, marker.alias, marker.source_location);
            max_width = max_width.max(line.len());
        }

        let box_width = max_width + 10;

        println!("{}", "-".repeat(box_width));

        println!("| Markers{}|", " ".repeat(box_width - 10));

        println!("{}", "-".repeat(box_width));

        for (idx, marker) in markers.iter().enumerate() {
            let line = format!("{}. {} ~ {}", idx, marker.alias, marker.source_location);
            let padding = " ".repeat(box_width - line.len() - 4);
            println!("| {} {}|", line, padding);
        }

        println!("{}", "-".repeat(box_width));
    }
    fn new() -> Box<Self> {
        Box::new(Self)
    }

    fn eq(&self, other: &str) -> bool {
        other.eq("ls")
    }

    fn to_clap(&self) -> clap::Command {
        clap::Command::new("ls").about("List all markers")
    }
}
