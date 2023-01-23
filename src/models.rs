extern crate rocket;
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
