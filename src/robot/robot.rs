use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use db::get_connection;
use super::rusqlite::Error;

pub enum Status {
    Available,
    Waiting,
    Busy,
    Unreachable,
    Unavailable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            Status::Available => "AVAILABLE",
            Status::Waiting => "WAITING",
            Status::Busy => "BUSY",
            Status::Unreachable => "UNREACHABLE",
            Status::Unavailable => "UNAVAILABLE",
        };
        write!(f, "{}", status)
    }
}

#[derive(Debug, Serialize)]
pub struct Robot {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub created_at: u64,
}

impl Robot {
    pub fn create(name: &str) -> Result<Robot, Error> {
        let current_time = {
            let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
        };
        let formatted_status = format!("{}", Status::Unavailable);

        let conn = get_connection();
        let mut stmt =
            conn.prepare("INSERT INTO robots (name, status, created_at) VALUES (?, ?, ?)")?;
        let id = stmt.insert(&[&name, &formatted_status, &current_time.to_string()])?;

        Ok(Robot {
            id: id,
            name: name.to_string(),
            status: formatted_status,
            created_at: current_time,
        })
    }

    pub fn set_status(id: i64, status: Status) -> Result<(), Error> {
        let conn = get_connection();
        let mut stmt = conn.prepare("UPDATE robots SET status=? WHERE id=?")?;
        let id = stmt.execute(&[&status.to_string(), &id])?;
        Ok(())
    }
}
