use crate::Config;
use chrono::{Local, NaiveDate, NaiveTime, TimeDelta};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Archive {
    pub config: Config,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Tome {
    pub day: NaiveDate,
    pub passages: Vec<Passage>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Passage {
    pub created_at: NaiveTime,
    pub tags: Vec<String>,
    pub project: Option<String>,
    pub body: String,
    pub duration: Option<TimeDelta>,
}

impl Default for Passage {
    fn default() -> Self {
        Self {
            created_at: Local::now().naive_local().time(),
            body: String::new(),
            duration: None,
            project: None,
            tags: vec![],
        }
    }
}

impl Default for Tome {
    fn default() -> Self {
        Self {
            day: Local::now().naive_local().date(),
            passages: vec![],
        }
    }
}
