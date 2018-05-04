use ws::{CloseCode, Error, Handler, Handshake, Message, Result, Sender};
use serde_json::{self, Value};
use auth::{jwt, ApiKey};
use robot;

pub struct RobotWebSocket {
    id: i64,
    out: Sender,
}

impl From<Sender> for RobotWebSocket {
    fn from(out: Sender) -> Self {
        RobotWebSocket { id: 0, out: out }
    }
}

#[derive(Debug, Deserialize)]
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

enum Event {
    LocationUpdate(UpdateLocationPayload),
}

impl Event {
    fn from_str(json: &str) -> Option<Event> {
        match serde_json::from_str::<Value>(&json) {
            Ok(v) => match v["action"] {
                Value::String(ref s) if s == "LOCATION_UPDATE" => {
                    UpdateLocationPayload::from_str(v["payload"].to_string().as_ref())
                }
                _ => None,
            },
            Err(_) => None,
        }
    }
}

impl Handler for RobotWebSocket {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        let s = handshake.request.resource();
        match s.get(1..s.len()) {
            Some(jwt) => match jwt::decode(jwt) {
                Ok(token) => {
                    self.id = ApiKey(token).as_i64();
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
                    Ok(_) => self.out.send("OK"),
                    Err(_) => self.out.send("ERROR_MALFORMED_INPUT"),
                }
            }
            None => self.out.send("ERROR_MALFORMED_INPUT"),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The robot is done with the connection."),
            CloseCode::Away => println!("The robot is leaving the connection."),
            _ => println!("The robot encountered an error: {}", reason),
        }
    }
}
