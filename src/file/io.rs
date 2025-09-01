use super::{Entries, Entry, Value};
use anyhow::{Error, anyhow};
use std::iter::IntoIterator;

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::List(list) => Self::list_to_string(list),
            Self::Text(value) => value.clone(),
        }
    }

    fn list_to_string(list: &[String]) -> String {
        let formatted_strings = list.iter().map(|s| s.clone()).collect::<Vec<_>>().join(" ");
        format!("({})", formatted_strings)
    }
}

impl Value {
    pub fn from_str(s: &str) -> Self {
        if s.starts_with('(') && s.ends_with(')') {
            Self::List(Self::list_from_str(s))
        } else {
            Self::Text(s.to_owned())
        }
    }

    fn list_from_str(s: &str) -> Vec<String> {
        let mut s = s.chars();
        s.next();
        s.next_back();
        let s = s.collect::<String>();
        s.split(' ').into_iter().map(str::to_string).collect()
    }
}

impl Entry {
    pub fn to_string(&self) -> String {
        let mut entry = String::new();
        self.metadata.iter().for_each(|(key, value)| {
            entry.push_str(key);
            entry.push_str(": ");
            entry.push_str(&value.to_string());
            entry.push_str("\n");
        });
        entry.push_str(">>>\n\n");
        entry.push_str(&self.body);
        entry
    }
}

impl Entry {
    pub fn try_from_str(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Err(anyhow!("Cannot make entry from empty string"));
        } else if let Some((metadata, body)) = s.split_once(">>>") {
            let metadata = Self::try_metadata_from_str(metadata)?;
            Ok(Self {
                metadata,
                body: body.trim().into(),
            })
        } else {
            let metadata = Self::try_metadata_from_str(s)?;
            Ok(Self {
                metadata,
                body: String::new(),
            })
        }
    }

    fn try_metadata_from_str(s: &str) -> Result<Vec<(String, Value)>, Error> {
        s.lines()
            .into_iter()
            .map(|line| match line.split_once(':') {
                Some((key, value)) => Ok((key.to_string(), Value::from_str(value.trim()))),
                None => Err(anyhow!("expected kv pair in metadata")),
            })
            .collect::<Result<Vec<(_, _)>, _>>()
    }
}

impl Entries {
    pub fn to_string(&self) -> String {
        self.entries
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
            .join("\n---\n")
    }

    pub fn try_from_str(s: &str) -> Result<Self, Error> {
        s.split("\n---\n")
            .map(Entry::try_from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(|good| Entries { entries: good })
    }
}
