pub mod manager;
mod internet;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, NaiveDateTime, Local, TimeZone};
use log::{error,debug};
#[derive(Debug, Deserialize, Serialize)]
pub struct License {
    pub value: String,
    pub expire: Option<String>, // ISO string
    pub username: String,
    pub hwid: Option<String>,
}

impl License {
    pub fn is_expired(&self) -> bool {
        // Parse expire string to DateTime<Utc> and check if it's in the past
        debug!("parsing {:?}", &self.expire);
        match NaiveDateTime::parse_from_str(&self.expire.to_owned().unwrap_or_default(), "%Y-%m-%dT%H:%M:%S.%f") { // Python ISO
            
            Ok(expire) => {
                let expire: DateTime<Local> = Local.from_local_datetime(&expire).unwrap();
                Utc::now() > expire
            }
            Err(e) => {
                error!("failed to parse license expire {e}");
                false
            }
        }
    }
}
