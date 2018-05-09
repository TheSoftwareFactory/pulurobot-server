extern crate chrono;
extern crate rusqlite;

mod station;
mod pinned_location;

use self::rusqlite::Error;

pub use self::station::Station;
pub use self::pinned_location::PinnedLocation;

pub fn create(name: &str) -> Result<Station, Error> {
    Station::create(name)
}

pub fn all_pinned_locations() -> Result<Vec<PinnedLocation>, Error> {
    PinnedLocation::all()
}

pub fn pin_location(name: &str, x: i64, y: i64, angle: i64) -> Result<PinnedLocation, Error> {
    PinnedLocation::create(name, x, y, angle)
}
