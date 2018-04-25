use std::time::{SystemTime, UNIX_EPOCH};
use db::get_connection;
use super::rusqlite::Error;

#[derive(Debug, Serialize)]
pub struct PinnedLocation {
    pub id: i64,
    pub name: String,
    pub x: i64,
    pub y: i64,
    pub angle: i64,
}

impl PinnedLocation {
    pub fn create(name: &str, x: i64, y: i64, angle: i64) -> Result<PinnedLocation, Error> {
        let conn = get_connection();
        let mut stmt =
            conn.prepare("INSERT INTO pinned_locations (name, x, y, angle) VALUES (?, ?, ?, ?)")?;
        let id = stmt.insert(&[&name, &x, &y, &angle])?;

        Ok(PinnedLocation {
            id: id,
            name: name.to_string(),
            x: x,
            y: y,
            angle: angle,
        })
    }
}
