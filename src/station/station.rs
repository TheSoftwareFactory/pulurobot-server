use db::get_connection;
use super::rusqlite::Error;
use super::chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct Station {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl Station {
    pub fn create(name: &str) -> Result<Station, Error> {
        let current_time = Utc::now();

        let conn = get_connection();
        let mut stmt = conn.prepare("INSERT INTO stations (name, created_at) VALUES (?, ?)")?;
        let id = stmt.insert(&[&name, &current_time.timestamp().to_string()])?;

        Ok(Station {
            id: id,
            name: name.to_string(),
            created_at: current_time,
        })
    }
}
