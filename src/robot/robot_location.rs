use std::time::{SystemTime, UNIX_EPOCH};
use db::get_connection;
use super::rusqlite::Error;

#[derive(Debug, Serialize)]
pub struct RobotLocation {
    pub robot_id: i64,
    pub x: i64,
    pub y: i64,
    pub angle: i64,
    pub updated_at: u64,
}

impl RobotLocation {
    pub fn create(robot_id: i64, x: i64, y: i64, angle: i64) -> Result<RobotLocation, Error> {
        let current_time = {
            let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
        };

        let conn = get_connection();
        let mut stmt = conn.prepare("INSERT INTO robot_locations (robot_id, x, y, angle, updated_at) VALUES (?, ?, ?, ?, ?)")?;
        let id = stmt.insert(&[&robot_id, &x, &y, &angle, &current_time.to_string()])?;

        Ok(RobotLocation {
            robot_id: id,
            x: x,
            y: y,
            angle: angle,
            updated_at: current_time,
        })
    }

    pub fn update(robot_id: i64, x: i64, y: i64, angle: i64) -> Result<(), Error> {
        let current_time = {
            let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
        };

        let conn = get_connection();
        let mut stmt = conn.prepare(
            "UPDATE robot_locations SET x=?, y=?, angle=?, updated_at=? WHERE robot_id=?",
        )?;
        let id = stmt.execute(&[&x, &y, &angle, &current_time.to_string(), &robot_id])?;
        Ok(())
    }
}
