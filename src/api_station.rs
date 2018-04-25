use rocket_contrib::Json;
use rocket::response;
use rocket::http::Status;
use auth::{jwt, ApiKey};
use station::{self, PinnedLocation};

#[derive(Debug, Deserialize)]
struct PinLocationPayload {
    name: String,
    x: i64,
    y: i64,
    angle: i64,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    public_key: String,
}

#[post("/auth", data = "<payload>")]
fn auth(payload: Json<AuthPayload>) -> &'static str {
    let private_key = &payload.public_key;
    println!("{:?}", private_key);
    "Welcome!"
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
