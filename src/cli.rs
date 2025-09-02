use clap::Parser;

/// A personal log
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// duration - duration the entry describes
    #[arg(short, long)]
    pub duration: Option<String>,
    /// tag an entry with metadata
    #[arg(short, long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,
    /// bucket an entry in a project
    #[arg(short, long)]
    pub project: Option<String>,
}
