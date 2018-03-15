use ws::{connect, CloseCode, Error, Handler, Handshake, Message, Result, Sender};

pub struct StationWebSocket {
    out: Sender,
}

impl From<Sender> for StationWebSocket {
    fn from(out: Sender) -> Self {
        StationWebSocket { out }
    }
}

impl Handler for StationWebSocket {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Station connected");
        self.out.send("Connected successfull")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Station sent message: {:?}", msg);
        self.out.send("Message received successfully")
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
