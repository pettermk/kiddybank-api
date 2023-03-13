extern crate rocket;

use std::fmt;
use diesel::{prelude::*};
// use serde::Serialize;
use crate::schema::users;


#[derive(Queryable, Clone, Debug)]
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
