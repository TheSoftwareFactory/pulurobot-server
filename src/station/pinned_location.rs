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
    pub fn get(id: i64) -> Result<PinnedLocation, Error> {
        let conn = get_connection();
        let mut stmt = conn.prepare("SELECT id, name, x, y, angle FROM pinned_locations WHERE id = ?")
            .unwrap();

        let mapped_rows = stmt.query_map(&[&id], |row| PinnedLocation {
            id: row.get(0),
            name: row.get(1),
            x: row.get(2),
            y: row.get(3),
            angle: row.get(4),
        });

        mapped_rows
            .and_then(|rows| Ok(rows.map(|row| row.unwrap()).collect::<Vec<PinnedLocation>>()))
            .and_then(|mut locations| locations.pop().ok_or(Error::IntegralValueOutOfRange(1, 1)))
    }

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
