use rocket::request::Form;

#[derive(FromForm)]
struct AuthPayload {
    public_key: String,
}

#[post("/auth", data = "<payload>")]
fn auth(payload: Form<AuthPayload>) -> &'static str {
    let private_key = &payload.get().public_key;
    println!("{:?}", private_key);
    "Welcome!"
}
