mod jwt;
mod crud;
mod models;
mod schema;

#[macro_use] extern crate rocket;

use rocket::Response;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use crate::crud::process_user;
use crate::models::{User, NewKid};


#[derive(Debug)]
pub enum AuthHeaderError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthHeaderError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        async fn get_user(key: &str) -> User {
            let token =
                key.split(" ").last().expect("Token should be there");
            let valid_jwt = jwt::process_jwt(token).await;
            process_user(&valid_jwt).await
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, AuthHeaderError::Missing)),
            Some(auth_header) => Outcome::Success(get_user(auth_header).await),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewKid {
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user")]
fn user(user: User) -> String {
    user.first_name.to_string()
}

#[post("/kid)]")]
fn create_kid(user: User, name: NewKid) -> String {
    crud::create_kid(&user, name);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        user,
        create_kid
    ])
}
