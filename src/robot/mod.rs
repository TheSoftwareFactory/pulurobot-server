extern crate rusqlite;

mod robot;
mod robot_battery_level;
mod robot_location;
mod robot_history_location;

use self::rusqlite::Error;
pub use self::robot::Robot;
pub use self::robot_battery_level::RobotBatteryLevel;
pub use self::robot_location::RobotLocation;
pub use self::robot_history_location::RobotHistoryLocation;

pub fn create(name: &str) -> Result<Robot, Error> {
    let robot = Robot::create(name)?;
    RobotBatteryLevel::create(robot.id)?;
    RobotLocation::create(robot.id, 0, 0, 0)?;
    Ok(robot)
}

pub fn update_battery_level(robot_id: i64, level: i32) -> Result<(), Error> {
    RobotBatteryLevel::update_battery_level(robot_id, level)
}

pub fn update_location(robot_id: i64, x: i64, y: i64, angle: i64) -> Result<(), Error> {
    RobotLocation::update(robot_id, x, y, angle)?;
    RobotHistoryLocation::create(robot_id, x, y, angle)?;
    Ok(())
}

pub fn get_location_history(robot_id: i64) -> Result<Vec<RobotHistoryLocation>, Error> {
    RobotHistoryLocation::all(robot_id)
}

pub fn all_robots() -> Result<Vec<Robot>, Error> {
    Robot::all()
}
