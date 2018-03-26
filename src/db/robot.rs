extern crate rusqlite;

use db::{get_connection};

#[derive(Debug, Serialize)]
pub struct Robot {
    pub id: i64,
    pub name: String
}

impl Robot {
    pub fn create(name: &str) -> Result<Robot, rusqlite::Error> {
        let conn = get_connection();
        let mut stmt = conn.prepare("INSERT INTO robots (name) VALUES (?)")?;
        let id = stmt.insert(&[&name])?;

        Ok(Robot {
            id: id,
            name: name.to_string(),
        })
    }

    pub fn update_battery_level(id: i64, level: i32) -> Result<(), rusqlite::Error> {
        let conn = get_connection();
        let mut stmt = conn.prepare("UPDATE robots SET battery_level = ? WHERE id = ?")?;
        stmt.execute(&[&level, &id])?;
        Ok(())
    }
}
