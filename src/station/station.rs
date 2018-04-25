use std::time::{SystemTime, UNIX_EPOCH};
use db::get_connection;
use super::rusqlite::Error;

#[derive(Debug, Serialize)]
pub struct Station {
    pub id: i64,
    pub name: String,
}

impl Station {
    pub fn create(name: &str) -> Result<Station, Error> {
        let current_time = {
            let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
        };

        let conn = get_connection();
        let mut stmt = conn.prepare("INSERT INTO stations (name, created_at) VALUES (?, ?)")?;
        let id = stmt.insert(&[&name, &current_time.to_string()])?;

        Ok(Station {
            id: id,
            name: name.to_string(),
        })
    }
}
