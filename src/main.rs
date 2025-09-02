mod cli;
mod config;
pub mod file;
mod tome;

use clap::Parser;
use cli::Args;
use config::Config;
use tome::Passage;

use crate::tome::Archive;

fn main() {
    let args = Args::parse();

    let archive = Archive::load_config().expect("create archive");
    let mut tome = archive.load_or_create_daily_tome().expect("creating daily");
    let mut new_passage = Passage::default();

    if let Some(duration) = args.duration {
        let duration = tome::bindings::try_duration_from_string(&duration)
            .expect("invalid date format as `00h 00m`");
        new_passage.duration = Some(duration);
    }

    if let Some(tags) = args.tags {
        new_passage.tags = tags;
    }

    if let Some(project) = args.project {
        new_passage.project = Some(project);
    }

    let edited = new_passage
        .edit(&archive.config.editor)
        .expect("edit passage");

    if edited.body.is_empty() {
        panic!("expected body exited without saving");
    }

    tome.passages.push(edited);

    tome.to_file(&archive.config.archive).expect("write tome");

    // let config = Config::load_or_create_home().expect("failed to load config");
    // let entry = Passage::default().edit(config.editor.as_path());
    // println!("edited\n{:#?}", entry);
    // println!("Hello, world!");
}
