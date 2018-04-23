extern crate rusqlite;

mod robot;
mod robot_battery_level;

use self::rusqlite::Error;
use self::robot::Robot;
use self::robot_battery_level::RobotBatteryLevel;

pub fn create(name: &str) -> Result<Robot, Error> {
    let robot = Robot::create(name)?;
    RobotBatteryLevel::create(robot.id)?;
    Ok(robot)
}

pub fn update_battery_level(robot_id: i64, level: i32) -> Result<(), Error> {
    RobotBatteryLevel::update_battery_level(robot_id, level)
}