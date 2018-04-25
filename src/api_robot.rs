use std::io::Read;
use rocket::{Data, Request};
use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::response;
use rocket::Outcome::{Failure, Success};
use rocket_contrib::Json;
use robot;
use auth::{jwt, ApiKey};

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateLocationPayload {
    x: i64,
    y: i64,
    angle: i64,
}

#[post("/register", data = "<payload>")]
fn register(payload: Json<RegisterPayload>) -> Result<String, response::Failure> {
    match robot::create(&payload.name) {
        Ok(robot) => Ok(jwt::generate(&robot.id.to_string())),
        Err(e) => {
            println!("{:?}", e);
            Err(response::Failure::from(Status::raw(400)))
        }
    }
}

#[patch("/battery/level", data = "<payload>")]
fn update_battery_level(key: ApiKey, payload: String) -> Result<(), response::Failure> {
    let id = key.as_i64();
    let level = payload.parse::<i32>().unwrap();
    match robot::update_battery_level(id, level) {
        Ok(_) => Ok(()),
        Err(_) => Err(response::Failure::from(Status::raw(400))),
    }
}

#[patch("/location", data = "<payload>")]
fn update_location(
    key: ApiKey,
    payload: Json<UpdateLocationPayload>,
) -> Result<(), response::Failure> {
    let id = key.as_i64();
    match robot::update_location(id, payload.x, payload.y, payload.angle) {
        Ok(_) => Ok(()),
        Err(_) => Err(response::Failure::from(Status::raw(400))),
    }
}
