use db::get_connection;
use super::rusqlite::Error;
use super::chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Serialize)]
pub struct RobotLocation {
    pub robot_id: i64,
    pub x: i64,
    pub y: i64,
    pub angle: i64,
    pub updated_at: DateTime<Utc>,
}

impl RobotLocation {
    pub fn get(robot_id: i64) -> Result<RobotLocation, Error> {
        let conn = get_connection();
        let mut stmt =
            conn.prepare("SELECT robot_id, x, y, angle, updated_at FROM robot_locations WHERE robot_id = ?")?;

        let mapped_rows = stmt.query_map(&[&robot_id], |row| RobotLocation {
            robot_id: row.get(0),
            x: row.get(1),
            y: row.get(2),
            angle: row.get(3),
            updated_at: {
                let secs: i64 = row.get(4);
                Utc.timestamp(secs, 0)
            },
        });

        mapped_rows
            .and_then(|rows| Ok(rows.map(|row| row.unwrap()).collect::<Vec<RobotLocation>>()))
            .and_then(|mut locations| Ok(locations.pop().unwrap()))
    }

    pub fn create(robot_id: i64, x: i64, y: i64, angle: i64) -> Result<RobotLocation, Error> {
        let current_time = Utc::now();

        let conn = get_connection();
        let mut stmt = conn.prepare("INSERT INTO robot_locations (robot_id, x, y, angle, updated_at) VALUES (?, ?, ?, ?, ?)")?;
        let id = stmt.insert(&[
            &robot_id,
            &x,
            &y,
            &angle,
            &current_time.timestamp().to_string(),
        ])?;

        Ok(RobotLocation {
            robot_id: id,
            x: x,
            y: y,
            angle: angle,
            updated_at: current_time,
        })
    }

    pub fn update(robot_id: i64, x: i64, y: i64, angle: i64) -> Result<(), Error> {
        let current_time = Utc::now();

        let conn = get_connection();
        let mut stmt = conn.prepare(
            "UPDATE robot_locations SET x=?, y=?, angle=?, updated_at=? WHERE robot_id=?",
        )?;
        stmt.execute(&[
            &x,
            &y,
            &angle,
            &current_time.timestamp().to_string(),
            &robot_id,
        ])?;
        Ok(())
    }
}
