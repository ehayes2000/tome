use super::types::DataFile;
use anyhow::{Error, anyhow};
use chrono::NaiveDate;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

impl DataFile {
    const FORMAT: &str = "%Y_%B_%d";

    pub fn write(&self, parent_path: &Path) -> Result<(), Error> {
        let data = toml::to_string_pretty(self)?;
        let name = format!("{}.toml", self.day.format(Self::FORMAT).to_string());
        let full_path = parent_path.join(name);

        let mut fd = OpenOptions::new()
            .create(true)
            .write(true)
            .open(full_path)?;
        fd.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn read(path: &Path) -> Result<Self, Error> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("expected file name"))?
            .to_str()
            .ok_or_else(|| anyhow!("expected valid file name"))?;

        let Some((name, _ext)) = name.split_once('.') else {
            return Err(anyhow!("expected single file extension"));
        };

        let date = NaiveDate::parse_from_str(name, Self::FORMAT)?;

        let mut fd = OpenOptions::new().create(false).read(true).open(path)?;
        let mut data = String::new();
        fd.read_to_string(&mut data)?;
        let mut f = toml::from_str::<Self>(data.as_str())?;
        f.day = date;
        Ok(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entry::Entry;
    use chrono::prelude::*;
    #[test]
    fn test_entry_serialize() {
        let dt = chrono::Utc.with_ymd_and_hms(2025, 8, 31, 0, 0, 0).unwrap();
        let entry = Entry {
            created_at: dt.naive_utc().time(),
            project: None,
            tags: vec![],
            content: "lots of stuff goes in here!".into(),
        };
        let s_entry = toml::to_string(&entry).expect("serialize");
        let d_entry = toml::from_str::<Entry>(&s_entry).expect("deserialize");
        assert_eq!(entry, d_entry);
    }

    #[test]
    fn test_file_serialize() {
        let entry = Entry {
            created_at: chrono::Utc
                .with_ymd_and_hms(2025, 8, 31, 6, 0, 0)
                .unwrap()
                .naive_utc()
                .time(),
            project: None,
            tags: vec![],
            content: "awesome stuff happening today".into(),
        };

        let entry2 = Entry {
            created_at: chrono::Utc
                .with_ymd_and_hms(2025, 8, 31, 6, 30, 0)
                .unwrap()
                .naive_utc()
                .time(),
            project: None,
            tags: vec![],
            content: "not awesome stuff happening today (bad 30 minutes)".into(),
        };

        let file = DataFile {
            day: NaiveDate::from_ymd_opt(2025, 8, 31).expect("naive date"),
            entries: vec![entry, entry2],
        };
        let parent = std::path::Path::new(".");
        file.write(parent).expect("write");
        let expected_path = std::path::Path::new("./2025_August_31.toml");
        assert!(expected_path.exists());
        assert!(expected_path.is_file());
        let r_file = DataFile::read(&expected_path).expect("read file");
        std::fs::remove_file(expected_path).expect("delete tmp file");
        assert_eq!(file, r_file, "matching r w");
    }
}
