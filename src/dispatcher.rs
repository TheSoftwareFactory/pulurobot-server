use ws::Sender;
use serde_json::{self, Value};
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref DISPATCHER: Mutex<Dispatcher> = Mutex::new(Dispatcher::new());
}

pub struct Dispatcher {
    pub subscribers: HashMap<String, Vec<Sender>>,
}

impl Dispatcher {
    fn new() -> Self {
        Dispatcher {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe_to_event(event: String, subscriber: Sender) {
        let mut dispatcher = DISPATCHER.lock().unwrap();
        let entry = dispatcher.subscribers.entry(event).or_insert(Vec::new());
        entry.push(subscriber);
    }

    pub fn publish_event(event: &str, message: String) {
        let dispatcher = DISPATCHER.lock().unwrap();
        if let Some(subscribers) = dispatcher.subscribers.get(event) {
            for subscriber in subscribers {
                let json = Event::to_json(event.to_string(), message.clone());
                subscriber.send(json);
            }
        }
    }
}

#[derive(Serialize)]
struct Event {
    event: String,
    payload: Value,
}

impl Event {
    fn to_json(event: String, payload: String) -> String {
        let v: Value = serde_json::from_str(&payload).unwrap();

        let object = Event{
            event: event, 
            payload: v
        };
        serde_json::to_string(&object).unwrap()
    }
}