use ws::{CloseCode, Error, Handler, Handshake, Message, Result, Sender};

pub struct RobotWebSocket {
    out: Sender,
}

impl From<Sender> for RobotWebSocket {
    fn from(out: Sender) -> Self {
        RobotWebSocket { out }
    }
}

impl Handler for RobotWebSocket {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Robot connected");
        self.out.send("Connected successfull")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Robot sent message: {:?}", msg);
        self.out.send("Message received successfully")
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
