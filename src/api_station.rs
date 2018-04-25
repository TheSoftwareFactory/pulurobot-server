use rocket_contrib::Json;
use rocket::response;
use rocket::http::Status;
use auth::{jwt, ApiKey};
use station::{self, PinnedLocation};

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    name: String,
}

#[derive(Debug, Deserialize)]
struct PinLocationPayload {
    name: String,
    x: i64,
    y: i64,
    angle: i64,
}

#[post("/register", data = "<payload>")]
fn register(payload: Json<RegisterPayload>) -> Result<String, response::Failure> {
    match station::create(&payload.name) {
        Ok(station) => Ok(jwt::generate(&station.id.to_string())),
        Err(e) => {
            println!("{:?}", e);
            Err(response::Failure::from(Status::raw(400)))
        }
    }
}

#[post("/pin-location", data = "<payload>")]
fn pin_location(
    _key: ApiKey,
    payload: Json<PinLocationPayload>,
) -> Result<Json<PinnedLocation>, response::Failure> {
    match station::pin_location(&payload.name, payload.x, payload.y, payload.angle) {
        Ok(pinned_location) => Ok(Json(pinned_location)),
        Err(_) => Err(response::Failure::from(Status::raw(400))),
    }
}
