use ws::Sender;
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
        let mut dispatcher = DISPATCHER.lock().unwrap();
        if let Some(subscribers) = dispatcher.subscribers.get(event) {
            for subscriber in subscribers {
                subscriber.send(message.clone());
            }
        }
    }
}