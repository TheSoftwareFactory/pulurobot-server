use rocket_contrib::Json;
use rocket::response;
use rocket::http::Status;
use auth::{jwt, ApiKey};
use station::{self, PinnedLocation};
use robot::{self, Robot, RobotHistoryLocation};

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    name: String,
}

#[derive(Debug, Deserialize)]
struct PinLocationPayload {
    name: String,
    x: i64,
    y: i64,
    angle: i64,
}

#[derive(FromForm)]
struct RobotLocationHistoryData {
    robot_id: i64,
}

#[post("/register", data = "<payload>")]
fn register(payload: Json<RegisterPayload>) -> Result<String, response::Failure> {
    match station::create(&payload.name) {
        Ok(station) => Ok(jwt::generate(&station.id.to_string())),
        Err(e) => {
            println!("{:?}", e);
            Err(response::Failure::from(Status::raw(400)))
        }
    }
}

#[post("/pin-location", data = "<payload>")]
fn pin_location(
    _key: ApiKey,
    payload: Json<PinLocationPayload>,
) -> Result<Json<PinnedLocation>, response::Failure> {
    match station::pin_location(&payload.name, payload.x, payload.y, payload.angle) {
        Ok(pinned_location) => Ok(Json(pinned_location)),
        Err(_) => Err(response::Failure::from(Status::raw(400))),
    }
}

#[get("/robot/location/history?<data>")]
fn robot_location_history(
    _key: ApiKey,
    data: RobotLocationHistoryData,
) -> Result<Json<Vec<RobotHistoryLocation>>, response::Failure> {
    match robot::get_location_history(data.robot_id) {
        Ok(locations) => Ok(Json(locations)),
        Err(_) => Err(response::Failure::from(Status::raw(400))),
    }
}

#[get("/robot/all")]
fn all_robots(
    _key: ApiKey
) -> Result<Json<Vec<Robot>>, response::Failure> {
    match robot::all_robots() {
        Ok(robots) => Ok(Json(robots)),
        Err(_) => Err(response::Failure::from(Status::raw(400))),
    }
}