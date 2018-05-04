use db::get_connection;
use super::rusqlite::Error;
use super::chrono::{Utc, DateTime, TimeZone};

#[derive(Debug, Serialize)]
pub struct RobotHistoryLocation {
    pub robot_id: i64,
    pub x: i64,
    pub y: i64,
    pub angle: i64,
    pub created_at: DateTime<Utc>,
}

impl RobotHistoryLocation {
    pub fn all(robot_id: i64) -> Result<Vec<RobotHistoryLocation>, Error> {
        let conn = get_connection();
        let mut stmt = conn.prepare("SELECT robot_id, x, y, angle, created_at FROM robot_history_locations WHERE robot_id = ? ORDER BY created_at DESC").unwrap();

        let mapped_rows = stmt.query_map(&[&robot_id], |row| RobotHistoryLocation {
            robot_id: row.get(0),
            x: row.get(1),
            y: row.get(2),
            angle: row.get(3),
            created_at: {
               let secs: i64 = row.get(3);
                Utc.timestamp(secs, 0)
            },
        });

        mapped_rows.and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<RobotHistoryLocation>>())
        })
    }

    pub fn create(
        robot_id: i64,
        x: i64,
        y: i64,
        angle: i64,
    ) -> Result<RobotHistoryLocation, Error> {
        let current_time = Utc::now();

        let conn = get_connection();
        let mut stmt = conn.prepare("INSERT INTO robot_history_locations (robot_id, x, y, angle, created_at) VALUES (?, ?, ?, ?, ?)")?;
        let id = stmt.insert(&[&robot_id, &x, &y, &angle, &current_time.timestamp().to_string()])?;

        Ok(RobotHistoryLocation {
            robot_id: id,
            x: x,
            y: y,
            angle: angle,
            created_at: current_time,
        })
    }
}
