use super::Entry;
use anyhow::{Context, Error, anyhow};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::process::Command;

pub fn edit_as_tmp(data: &str, editor: &Path) -> Result<String, Error> {
    let mut tmp = tempfile::NamedTempFile::new().context("failed to create tmp file")?;

    tmp.write_all(data.as_bytes())
        .context("failed to setup tmp file")?;
    tmp.flush().context("write to fild")?;

    let mut child = Command::new(editor)
        .arg("+")
        .arg(tmp.path())
        .spawn()
        .context("failed to spawn editor")?;

    child
        .wait()
        .context("error waiting for child")
        .and_then(|exit_code| {
            if exit_code.success() {
                Ok(())
            } else {
                Err(anyhow!("editor exited with non-zero exit code"))
            }
        })?;

    tmp.seek(SeekFrom::Start(0)).context("reset read head")?;

    let data = std::fs::read_to_string(tmp.path()).context("failed to read entry")?;

    Ok(data)
}

impl Entry {
    pub fn edit(self, editor: &Path) -> Result<Self, Error> {
        let data = self.to_string();
        let raw_edit = edit_as_tmp(&data, editor)?;
        Self::try_from_str(&raw_edit)
    }
}
