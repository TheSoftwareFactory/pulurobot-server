use ws::{CloseCode, Error, Handler, Handshake, Message, Result, Sender};
use serde_json::{self, Value};
use auth::{jwt, ApiKey};
use dispatcher::Dispatcher;

#[derive(Debug, Deserialize)]
struct SubscribeToEventPayload {
    event: String,
}

impl SubscribeToEventPayload {
    fn from_str(json: &str) -> Option<Event> {
        match serde_json::from_str::<SubscribeToEventPayload>(json) {
            Ok(payload) => Some(Event::SubscribeToEvent(payload)),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
enum Event {
    SubscribeToEvent(SubscribeToEventPayload),
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
                "SUBSCRIBE_TO_EVENT" => SubscribeToEventPayload::from_str(&payload),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct StationWebSocket {
    id: i64,
    out: Sender,
}

impl From<Sender> for StationWebSocket {
    fn from(out: Sender) -> Self {
        StationWebSocket { 
            id: 0,
            out: out,
        }
    }
}

impl Handler for StationWebSocket {
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
            Some(Event::SubscribeToEvent(payload)) => {
                Dispatcher::subscribe_to_event(payload.event, self.out.clone());
                self.out.send("OK")
            }
            None => self.out.send("ERROR_MALFORMED_INPUT"),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The station is done with the connection."),
            CloseCode::Away => println!("The station is leaving the connection."),
            _ => println!("The station encountered an error: {}", reason),
        }
    }
}
