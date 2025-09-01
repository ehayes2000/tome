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
    // let args = Args::parse();
    let archive = Archive::load_config().expect("create archive");

    let mut tome = archive.load_or_create_daily_tome().expect("creating daily");
    let new_passage = Passage::default()
        .edit(&archive.config.editor)
        .expect("edit passage");
    tome.passages.push(new_passage);

    tome.to_file(&archive.config.archive).expect("write tome");

    // let config = Config::load_or_create_home().expect("failed to load config");
    // let entry = Passage::default().edit(config.editor.as_path());
    // println!("edited\n{:#?}", entry);
    // println!("Hello, world!");
}
