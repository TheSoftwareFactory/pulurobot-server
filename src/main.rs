#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate ws;

mod ws_station;
mod ws_robot;

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
    rocket::ignite().mount("/", routes![index]).launch();
}
