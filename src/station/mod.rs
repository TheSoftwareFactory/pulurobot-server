extern crate rusqlite;

mod pinned_location;

use self::rusqlite::Error;
pub use self::pinned_location::PinnedLocation;

pub fn pin_location(name: &str, x: i64, y: i64, angle: i64) -> Result<PinnedLocation, Error> {
    PinnedLocation::create(name, x, y, angle)
}