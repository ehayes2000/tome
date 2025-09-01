use anyhow::{Context, Error, anyhow};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config {
    pub root: PathBuf,
    pub archive: PathBuf,
    pub editor: PathBuf,
}

impl Config {
    pub fn load_or_create_home() -> Result<Self, Error> {
        let config_dir = std::env::home_dir()
            .ok_or(anyhow!("Expected home directory. Set $HOME"))?
            .join(".tome");
        Self::load_or_create(config_dir)
    }

    pub fn load_or_create(config_dir: PathBuf) -> Result<Self, Error> {
        if !config_dir.exists() {
            std::fs::create_dir(&config_dir).context("Error creating config")?;
        }

        let archive = config_dir.clone().join("archive");

        if !archive.exists() {
            std::fs::create_dir(&archive).context("Error creating logs dir")?;
        }

        let editor = std::env::var("EDITOR")
            .context("Expected text editor. Set $EDITOR")
            .and_then(|editor| PathBuf::from_str(&editor).context("Invalid editor path"))
            .and_then(|editor| {
                if editor.exists() {
                    Ok(editor)
                } else {
                    Err(anyhow!("Editor could not be found at {:?}", editor))
                }
            })?;

        Ok(Self {
            root: config_dir,
            archive,
            editor,
        })
    }
}
