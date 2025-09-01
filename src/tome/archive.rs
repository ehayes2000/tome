use super::{Archive, Tome};
use crate::Config;
use anyhow::Error;
use chrono::Local;

impl Archive {
    pub fn load_config() -> Result<Self, Error> {
        let config = Config::load_or_create_home()?;
        Ok(Self { config })
    }

    pub fn load_or_create_daily_tome(&self) -> Result<Tome, Error> {
        let today = Local::now().date_naive();
        let path = self.config.archive.join(Tome::file_name_from_date(today));
        if path.exists() {
            Tome::try_from_file(path.as_path())
        } else {
            Ok(Tome::default())
        }
    }
}
