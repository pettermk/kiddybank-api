extern crate rocket;

use std::fmt;
use chrono::{NaiveDateTime};
use diesel::{prelude::*};
use rocket::serde::{Deserialize, Serialize};
use crate::schema::{users, kids, transactions};


#[derive(Identifiable, Queryable, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub external_id: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User: {} {} {}", self.id, self.first_name, self.last_name)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct UserDto {
    pub first_name: String,
    pub last_name: String,
    pub external_id: String,
}

#[derive(Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub external_id: String,
}

#[derive(Identifiable, Queryable, Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Kid {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

impl fmt::Display for Kid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kid: {}", self.name)
    }
}

#[derive(Insertable, Associations, Queryable, Clone, Debug, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name=kids)]
#[serde(crate = "rocket::serde")]
pub struct KidDto {
    pub name: String,
    pub user_id: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewKid {
    pub name: String,
}

#[derive(Identifiable, Queryable, Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    pub id: i32,
    pub ts: NaiveDateTime,
    pub change: f64,
    pub kid_id: i32,
    pub description: String,
}

#[derive(Insertable, Associations, Queryable, Clone, Debug)]
#[diesel(belongs_to(Kid))]
#[diesel(table_name=transactions)]
pub struct TransactionDto {
    pub ts: NaiveDateTime,
    pub change: f64,
    pub kid_id: i32,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewTransaction {
    pub change: f64,
    pub kid_id: i32,
    pub description: String,
}
