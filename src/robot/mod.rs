extern crate rusqlite;

mod robot;
mod robot_battery_level;
mod robot_location;
mod robot_history_location;

use self::rusqlite::Error;
use self::robot::Robot;
use self::robot_battery_level::RobotBatteryLevel;
use self::robot_location::RobotLocation;
use self::robot_history_location::RobotHistoryLocation;

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
