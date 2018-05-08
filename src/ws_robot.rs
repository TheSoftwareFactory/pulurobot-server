use ws::{CloseCode, Error, Handler, Handshake, Message, Result, Sender};
use serde_json::{self, Value};
use auth::{jwt, ApiKey};
use robot;
use dispatcher::Dispatcher;

#[derive(Debug, Serialize, Deserialize)]
struct UpdateLocationPayload {
    x: i64,
    y: i64,
    angle: i64,
}

impl UpdateLocationPayload {
    fn from_str(json: &str) -> Option<Event> {
        match serde_json::from_str::<UpdateLocationPayload>(json) {
            Ok(payload) => Some(Event::LocationUpdate(payload)),
            Err(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateBatteryLevelPayload {
    level: i32,
}

impl UpdateBatteryLevelPayload {
    fn from_str(json: &str) -> Option<Event> {
        match serde_json::from_str::<UpdateBatteryLevelPayload>(json) {
            Ok(payload) => Some(Event::BatteryLevelUpdate(payload)),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
enum Event {
    LocationUpdate(UpdateLocationPayload),
    BatteryLevelUpdate(UpdateBatteryLevelPayload),
}

impl Event {
    fn from_str(json: &str) -> Option<Event> {
        if let Ok(v) = serde_json::from_str::<Value>(&json) {
            let action = v["action"]
                .to_string()
                .as_str()
                .replace("\"", "")
                .to_string();
            let payload = v["payload"].to_string();

            match action.as_ref() {
                "LOCATION_UPDATE" => UpdateLocationPayload::from_str(&payload),
                "BATTERY_LEVEL_UPDATE" => UpdateBatteryLevelPayload::from_str(&payload),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub struct RobotWebSocket {
    id: i64,
    out: Sender,
}

impl From<Sender> for RobotWebSocket {
    fn from(out: Sender) -> Self {
        RobotWebSocket { id: 0, out: out }
    }
}

impl Handler for RobotWebSocket {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        let s = handshake.request.resource();
        match s.get(1..s.len()) {
            Some(jwt) => match jwt::decode(jwt) {
                Ok(token) => {
                    self.id = ApiKey(token).as_i64();

                    let event = "CONNECTED_ROBOT";
                    let mut message = String::from("{\"id\": ");
                    message.push_str(&format!("{}", self.id));
                    message.push('}');
                    Dispatcher::publish_event(event, message);

                    self.out.send("OK")
                }
                Err(_) => self.out.send("ERROR_UNAUTHORIZED"),
            },
            None => self.out.send("ERROR_INVALID_JWT"),
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let json = msg.into_text()?;

        match Event::from_str(&json) {
            Some(Event::LocationUpdate(payload)) => {
                match robot::update_location(self.id, payload.x, payload.y, payload.angle) {
                    Ok(_) => {
                        let event = format!("LOCATION_UPDATE#{}", self.id);
                        let message = serde_json::to_string(&payload).unwrap();
                        Dispatcher::publish_event(&event, message);

                        robot::update_status(self.id, true);
                        self.out.send("OK")
                    }
                    Err(_) => self.out.send("ERROR_MALFORMED_INPUT"),
                }
            }
            Some(Event::BatteryLevelUpdate(payload)) => {
                match robot::update_battery_level(self.id, payload.level) {
                    Ok(_) => {
                        let event = format!("BATTERY_LEVEL_UPDATE#{}", self.id);
                        let message = serde_json::to_string(&payload).unwrap();
                        Dispatcher::publish_event(&event, message);

                        self.out.send("OK")
                    },
                    Err(_) => self.out.send("ERROR_MALFORMED_INPUT"),
                }
            }
            None => self.out.send("ERROR_MALFORMED_INPUT"),
        }
    }

    fn on_error(&mut self, err: Error) {
        robot::update_status(self.id, false);
        println!("The server encountered an error: {:?}", err);
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        robot::update_status(self.id, false);

        match code {
            CloseCode::Normal => println!("The robot is done with the connection."),
            CloseCode::Away => println!("The robot is leaving the connection."),
            _ => println!("The robot encountered an error: {}", reason),
        }
    }
}
