use super::{Passage, Tome};
use crate::file::{Entries, Entry, Value};
use anyhow::{Context, Error, anyhow};
use chrono::{Duration, NaiveDate, NaiveTime};
use std::io::Write;
use std::path::Path;

const FORMAT_DATE: &str = "%Y_%B_%d";
const FORMAT_TIME: &str = "%H:%M";
const FORMAT_DURATION: &str = "%Hh %Mm";

fn date_to_string(date: NaiveDate) -> String {
    date.format(FORMAT_DATE).to_string()
}

fn try_date_from_str(s: &str) -> Result<NaiveDate, Error> {
    NaiveDate::parse_from_str(s, FORMAT_DATE).context("unable to parse time")
}

fn time_to_string(time: NaiveTime) -> String {
    time.format(FORMAT_TIME).to_string()
}

fn try_time_from_str(s: &str) -> Result<NaiveTime, Error> {
    NaiveTime::parse_from_str(s, FORMAT_TIME).context("unable to parse time")
}

pub fn duration_to_string(duration: Duration) -> String {
    // TODO: this is trash
    if duration.num_days() >= 1 {
        eprintln!("unsupported duration manually correct");
    }
    let time = NaiveTime::MIN + duration;
    time.format(FORMAT_DURATION).to_string()
}

pub fn try_duration_from_string(s: &str) -> Result<Duration, Error> {
    let t = NaiveTime::parse_from_str(&s, FORMAT_DURATION).context("unable to parse duration")?;
    Ok(t.signed_duration_since(NaiveTime::MIN))
}

const KEY_CREATED_AT: &str = "created_at";
const KEY_TAGS: &str = "tags";
const KEY_PROJECT: &str = "project";
const KEY_DURATION: &str = "duration";

impl Into<Entry> for Passage {
    fn into(self) -> Entry {
        let mut metadata = Vec::new();
        metadata.push((
            KEY_CREATED_AT.into(),
            time_to_string(self.created_at).into(),
        ));

        if !self.tags.is_empty() {
            metadata.push((KEY_TAGS.into(), self.tags.into()))
        }

        if let Some(project) = self.project {
            metadata.push((KEY_PROJECT.into(), project.into()))
        }

        if let Some(duration) = self.duration {
            metadata.push((KEY_DURATION.into(), duration_to_string(duration).into()));
        }

        Entry {
            body: self.body,
            metadata,
        }
    }
}

impl TryFrom<Entry> for Passage {
    type Error = Error;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        // required: created_at
        let mut passage = Passage::default();
        value
            .metadata(KEY_CREATED_AT)
            .ok_or(anyhow!("exected {}", KEY_CREATED_AT))
            .and_then(|raw_time| match raw_time {
                Value::Text(s) => try_time_from_str(&s),
                _ => Err(anyhow!("malformed time")),
            })
            .map(|time| passage.created_at = time)?;

        if let Some(v) = value.metadata(KEY_DURATION) {
            let duration = match v {
                Value::Text(s) => try_duration_from_string(&s),
                _ => Err(anyhow!("malformed time")),
            }?;
            passage.duration = Some(duration);
        }

        value.metadata(KEY_TAGS).map(|tags| match tags {
            Value::List(tags) => passage.tags = tags,
            Value::Text(tag) => passage.tags = vec![tag],
        });

        if let Some(project) = value.metadata(KEY_PROJECT) {
            match project {
                Value::Text(p) => passage.project = Some(p),
                _ => anyhow::bail!("malformed project"),
            }
        }

        passage.body = value.body;
        Ok(passage)
    }
}

impl Into<Entries> for Tome {
    fn into(self) -> Entries {
        let entries = self.passages.into_iter().map(Into::into).collect();
        Entries { entries }
    }
}

impl Tome {
    pub fn try_from_file(path: &Path) -> Result<Self, Error> {
        let date = path
            .file_stem()
            .ok_or(anyhow!("no file name"))?
            .to_str()
            .ok_or(anyhow!("Why unicode in file name!?!?!"))
            .and_then(|name| try_date_from_str(name))?;
        let data = std::fs::read_to_string(path)?;

        let passages = Entries::try_from_str(&data)?
            .entries
            .into_iter()
            .map(Passage::try_from)
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Self {
            day: date,
            passages,
        })
    }

    pub fn file_name_from_date(date: NaiveDate) -> String {
        format!("{}.txt", date_to_string(date))
    }

    pub fn to_file(self, parent: &Path) -> Result<(), Error> {
        let full_path = parent.join(Self::file_name_from_date(self.day));
        let data: Entries = self.into();
        let data_string = data.to_string();

        let mut fd = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(full_path)
            .context("failed to open file")?;

        fd.write_all(data_string.as_bytes())
            .context("failed to write file")?;

        fd.flush().context("no toilet")?;

        Ok(())
    }
}
