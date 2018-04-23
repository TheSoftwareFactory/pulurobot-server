#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate rocket;
extern crate ws;

mod ws_station;
mod ws_robot;
mod api_station;
mod api_robot;
mod auth;
mod db;
mod robot;

use std::thread;
use ws_station::StationWebSocket;
use ws_robot::RobotWebSocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    thread::spawn(|| ws::listen("127.0.0.1:3001", |out| StationWebSocket::from(out)).unwrap());
    thread::spawn(|| ws::listen("127.0.0.1:3002", |out| RobotWebSocket::from(out)).unwrap());

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/v1/station", routes![api_station::auth])
        .mount(
            "/api/v1/robot",
            routes![api_robot::register, api_robot::update_battery_level],
        )
        .launch();
}
