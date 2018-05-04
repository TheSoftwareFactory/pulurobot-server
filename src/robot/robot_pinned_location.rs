use db::get_connection;
use super::rusqlite::Error;

#[derive(Debug, Serialize)]
pub struct RobotPinnedLocation {
    pub pinned_location_id: i64,
    pub robot_id: i64,
    pub tag: String,
}

impl RobotPinnedLocation {
    pub fn all_charge_locations() -> Result<Vec<RobotPinnedLocation>, Error> {
        let conn = get_connection();
        let mut stmt = conn.prepare("SELECT pinned_location_id, robot_id, tag FROM robot_pinned_locations WHERE tag = 'CHARGE_STATION'")
            .unwrap();

        let mapped_rows = stmt.query_map(&[], |row| RobotPinnedLocation {
            pinned_location_id: row.get(0),
            robot_id: row.get(1),
            tag: row.get(2),
        });

        mapped_rows.and_then(|rows| {
            Ok(rows.map(|row| row.unwrap())
                .collect::<Vec<RobotPinnedLocation>>())
        })
    }

    pub fn create(pinned_location_id: i64, robot_id: i64, tag: &str) -> Result<RobotPinnedLocation, Error> {
        let conn = get_connection();
        let mut stmt =
            conn.prepare("INSERT INTO robot_pinned_locations (pinned_location_id, robot_id, tag) VALUES (?, ?, ?)")?;
        stmt.insert(&[&pinned_location_id, &robot_id, &tag])?;

        Ok(RobotPinnedLocation {
            pinned_location_id: pinned_location_id,
            robot_id: robot_id,
            tag: tag.to_string(),
        })
    }
}
