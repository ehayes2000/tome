use super::date::ts_hh_mm;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]

pub struct DataFile {
    #[serde(skip)]
    pub day: NaiveDate,
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Entry {
    #[serde(with = "ts_hh_mm")]
    pub created_at: NaiveTime,
    pub tags: Vec<String>,
    pub project: Option<String>,
    pub content: String,
}
