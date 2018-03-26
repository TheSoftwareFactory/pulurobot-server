extern crate jsonwebtoken;
use self::jsonwebtoken::{Header, Validation};

// TODO: Need to take the secret key from config file
const SECRET_KEY: &'static str = "secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

impl Claims {
    fn new(sub: &str) -> Self {
        Claims {
            sub: sub.to_string(),
        }
    }
}

pub fn generate(sub: &str) -> String {
    let claims = Claims::new(sub);
    jsonwebtoken::encode(&Header::default(), &claims, SECRET_KEY.as_ref()).unwrap()
}

pub fn decode(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let token_data =
        jsonwebtoken::decode::<Claims>(token, SECRET_KEY.as_ref(), &Validation::default())?;
    Ok(token_data.claims.sub)
}
