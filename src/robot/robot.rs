use std::fmt;
use db::get_connection;
use super::rusqlite::Error;
use super::chrono::{Utc, DateTime, TimeZone};

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
    pub created_at: DateTime<Utc>,
}

impl Robot {
    pub fn all() -> Result<Vec<Robot>, Error> {
        let conn = get_connection();
        let mut stmt = conn.prepare("SELECT id, name, status, created_at FROM robots").unwrap();

        let mapped_rows = stmt.query_map(&[], |row| Robot {
            id: row.get(0),
            name: row.get(1),
            status: row.get(2),
            created_at: {
                let secs: i64 = row.get(3);
                Utc.timestamp(secs, 0)
            },
        });

        mapped_rows.and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Robot>>())
        })
    }

    pub fn create(name: &str) -> Result<Robot, Error> {
        let current_time = Utc::now();
        let formatted_status = format!("{}", Status::Unavailable);

        let conn = get_connection();
        let mut stmt =
            conn.prepare("INSERT INTO robots (name, status, created_at) VALUES (?, ?, ?)")?;
        let id = stmt.insert(&[&name, &formatted_status, &current_time.timestamp().to_string()])?;

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
        stmt.execute(&[&status.to_string(), &id])?;
        Ok(())
    }
}
