mod jwt;
mod crud;
mod models;
mod schema;

#[macro_use] extern crate rocket;

use models::{Kid, NewTransaction, Transaction, KidWithBalance};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::data::{Data, FromData, self};
use rocket::http::{Status, Header};
use rocket::request::{Outcome, Request, FromRequest};
use rocket::{Response};
use rocket::serde::json::{serde_json, Json};
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

#[rocket::async_trait]
impl<'r> FromData<'r> for Kid{
    type Error = KidError;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;
        let mut data = data.open("256 kB".parse().unwrap());
        let mut string = String::new();
        data.read_to_string(&mut string).await;
        match serde_json::from_str::<Kid>(&string) {
            Ok(kid) => Success(kid),
            Err(_) => Failure((Status::BadRequest, KidError::Invalid)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewTransaction {
    type Error = KidError;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;
        let mut data = data.open("256 kB".parse().unwrap());
        let mut string = String::new();
        data.read_to_string(&mut string).await;
        match serde_json::from_str::<NewTransaction>(&string) {
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

#[get("/kid")]
async fn get_kids(user: User) -> Json<Vec<KidWithBalance>> {
    Json(crud::get_kids_with_balance(&user).await)
}

#[post("/kid", data="<name>")]
async fn create_kid(user: User, name: NewKid) -> () {
    crud::create_kid(&user, &name).await;
}

#[post("/transaction", data="<transaction>")]
async fn create_transaction(user: User, transaction: NewTransaction) -> () {
    crud::create_transaction(&user, &transaction).await;
}

#[get("/transaction?<kid_id>")]
async fn get_transactions(user: User, kid_id: i32) -> Json<Vec<Transaction>> {
    Json(crud::get_transactions(&user, &kid_id).await)
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![
        index,
        all_options,
        user,
        get_kids,
        create_kid,
        create_transaction,
        get_transactions,
    ])
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}