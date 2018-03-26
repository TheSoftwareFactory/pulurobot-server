pub mod jwt;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};

pub struct ApiKey(String);

impl ApiKey {
    pub fn as_i32(&self) -> i32 {
        self.0.parse().unwrap()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        match jwt::decode(keys[0]) {
            Ok(token) => Outcome::Success(ApiKey(token)),
            Err(_) => Outcome::Forward(()),
        }
    }
}
