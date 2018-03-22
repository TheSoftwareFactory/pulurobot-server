extern crate serde_json;

use std::io::Read;
use rocket::{Data, Request};
use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::Outcome::{Failure, Success};
use db::robot::Robot;
use auth::{ApiKey, jwt};

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    name: String,
}

impl FromData for RegisterPayload {
    type Error = String;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Read the data into a String.
        let mut json = String::new();
        if let Err(e) = data.open().read_to_string(&mut json) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        // Deserialize the json
        match serde_json::from_str(&json) {
            Ok(payload) => Success(payload),
            Err(e) => Failure((Status::InternalServerError, format!("{:?}", e))),
        }
    }
}

#[post("/register", data = "<payload>")]
fn register(payload: RegisterPayload) -> String {
    let robot = Robot::create(&payload.name);
    jwt::generate(&robot.id.to_string())
}

// TODO: Need to write this endpoint
#[patch("/battery/level", data = "<payload>")]
fn update_battery_level(key: ApiKey, payload: String) -> String {
    println!("{:?}", payload);
    println!("{:?}", key.as_i32());
    String::from("OK")
}