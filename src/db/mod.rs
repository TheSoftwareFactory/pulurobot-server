extern crate lazy_static;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

pub mod robot;
pub mod robot_battery_level;

use std::path::Path;
use std::env;
use self::r2d2_sqlite::SqliteConnectionManager;

lazy_static! {
    pub static ref DB_POOL: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> = setup_db_pool();
}

fn setup_db_pool() -> r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> {
    let path = env::current_dir()
            .unwrap()
            .join(Path::new("db/database.db"));
    let manager = SqliteConnectionManager::file(path);
    r2d2::Pool::new(manager).unwrap()
}

pub fn get_connection() -> r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager> {
    let pool = DB_POOL.clone();
    pool.get().unwrap()
}
