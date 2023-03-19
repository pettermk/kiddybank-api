mod jwt;
mod crud;
mod models;
mod schema;

#[macro_use] extern crate rocket;

use rocket::data::{Data, FromData, self};
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::serde::json::serde_json;
use rocket::tokio::io::AsyncReadExt;
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

#[derive(Debug)]
pub enum KidError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewKid {
    type Error = KidError;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;
        let mut data = data.open("256 kB".parse().unwrap());
        let mut string = String::new();
        data.read_to_string(&mut string).await;
        match serde_json::from_str::<NewKid>(&string) {
            Ok(kid) => Success(kid),
            Err(_) => Failure((Status::BadRequest, KidError::Invalid)),
        }
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user")]
fn user(user: User) -> String {
    user.first_name.to_string()
}

#[post("/kid", data="<name>")]
fn create_kid(user: User, name: NewKid) -> () {
    crud::create_kid(&user, &name);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        user,
        create_kid,
    ])
}
