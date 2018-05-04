extern crate chrono;
extern crate rusqlite;

mod robot;
mod robot_battery_level;
mod robot_location;
mod robot_pinned_location;
mod robot_history_location;

use self::rusqlite::Error;
pub use self::robot::Robot;
pub use self::robot_pinned_location::RobotPinnedLocation;
pub use self::robot_battery_level::RobotBatteryLevel;
pub use self::robot_location::RobotLocation;
pub use self::robot_history_location::RobotHistoryLocation;
use ::station::PinnedLocation;

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

pub fn update_status(robot_id: i64, connection_available: bool) {
    let status = match connection_available {
        true => infere_status_for_robot_available(robot_id),
        false => infere_status_for_robot_unavailable(robot_id),
    };
    Robot::update_status(robot_id, status).unwrap();
}

fn infere_status_for_robot_available(robot_id: i64) -> robot::Status {
    let location = RobotLocation::get(robot_id).unwrap();
    if is_robot_location_inside_charge_station(&location) {
        return robot::Status::Available;
    }

    let is_robot_moving = {
        RobotHistoryLocation::all(robot_id)
            .unwrap()
            .into_iter()
            .skip(1)
            .take(5)
            .any(|prev_loc| {
                let prev_loc_xy = (prev_loc.x, prev_loc.y);
                let curr_loc_xy = (location.x, location.y);
                prev_loc_xy != curr_loc_xy
            })
    };

    if is_robot_moving {
        return robot::Status::Busy;
    }

    return robot::Status::Waiting;
}

fn infere_status_for_robot_unavailable(robot_id: i64) -> robot::Status {
    let location = RobotLocation::get(robot_id).unwrap();

    match is_robot_location_inside_charge_station(&location) {
        true => robot::Status::Unavailable,
        false => robot::Status::Unreachable,
    }
}

fn is_robot_location_inside_charge_station(location: &RobotLocation) -> bool {
    let pinned_charge_stations = RobotPinnedLocation::all_charge_locations().unwrap_or(Vec::new());

    let l1 = (location.x, location.y);
    let l2 = pinned_charge_stations
        .into_iter()
        .map(|pinned_station| {
            PinnedLocation::get(pinned_station.pinned_location_id).unwrap()
        })
        .map(|loc| (loc.x, loc.y))
        .collect::<Vec<(i64, i64)>>();
    l2.contains(&l1)
}
