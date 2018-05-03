use db::get_connection;
use super::rusqlite::Error;
use super::chrono::{Utc, DateTime};

#[derive(Debug, Serialize)]
pub struct RobotBatteryLevel {
    pub robot_id: i64,
    pub level: i8,
    pub updated_at: DateTime<Utc>,
}

impl RobotBatteryLevel {
    pub fn create(robot_id: i64) -> Result<RobotBatteryLevel, Error> {
        let current_time = Utc::now();
        let default_battery_level = 0;

        let conn = get_connection();
        let mut stmt = conn.prepare(
            "INSERT INTO robot_battery_levels (robot_id, level, updated_at) VALUES (?, ?, ?)",
        )?;
        let id = stmt.insert(&[&robot_id, &default_battery_level, &current_time.timestamp().to_string()])?;

        Ok(RobotBatteryLevel {
            robot_id: id,
            level: default_battery_level,
            updated_at: current_time,
        })
    }

    pub fn update_battery_level(robot_id: i64, level: i32) -> Result<(), Error> {
        let conn = get_connection();
        let mut stmt =
            conn.prepare("UPDATE robot_battery_levels SET level = ? WHERE robot_id = ?")?;
        stmt.execute(&[&level, &robot_id])?;
        Ok(())
    }
}
