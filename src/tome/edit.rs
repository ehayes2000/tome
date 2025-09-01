use super::Passage;
use crate::file::Entry;
use anyhow::Error;
use std::path::Path;

impl Passage {
    pub fn edit(self, editor: &Path) -> Result<Self, Error> {
        let entry: Entry = self.into();
        let edited = entry.edit(editor)?;
        Self::try_from(edited)
    }
}
