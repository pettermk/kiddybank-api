use diesel::{prelude::*, insert_into};
use alcoholic_jwt::ValidJWT;
use crate::models::{User, NewUser, Kid, NewKid};
use kiddybank_api::establish_connection;
use crate::schema::users::dsl::{users, kids};

async fn get_user(ext_id: String) -> Option<User> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let user = users
        .filter(external_id.eq(ext_id))
        .limit(1)
        .load::<User>(connection)
        .expect("Error loading users");
    println!("{:?}", &user.first());
    match user.len() {
        0 => None,
        _ => Some(user[0].clone()),
    }
}

async fn create_user(jwt: &ValidJWT) -> User {
    let connection = &mut establish_connection();
    let first_name = jwt.claims.get("given_name")
        .expect("Given name should exist in token")
        .as_str()
        .expect("Given name should be string");
    let last_name = jwt.claims.get("family_name")
        .expect("Family name should be in token")
        .as_str()
        .expect("Token should have family name");
    let external_id = jwt.claims.get("oid")
        .expect("Family name should be in token")
        .as_str()
        .expect("Token should have family name");
    let new_user = NewUser {
        first_name: String::from(first_name),
        last_name: String::from(last_name),
        external_id: String::from(external_id)
    };

    insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("User should be inserted");
    get_user(String::from(external_id)).await.expect("User has been created")
}

pub async fn process_user(jwt: &ValidJWT) -> User {
    let uid = jwt.claims.get(String::from("oid")).expect("JWT should have uid");
    let uid = uid.as_str().expect("Uid should be string");

    let user: Option<User> = get_user(String::from(uid)).await;
    print!("{:?}", user);
    match user {
        Some(u) => u,
        None => create_user(jwt).await,
    }
}

async fn get_kid(user_id: i32, name: String) -> Option<Kid> {
    let connection = &mut establish_connection();
    let kid = kids
        .filter(user_id.eq(&user_id), name.eq(&name))
        .limit(1)
        .load::<Kid>(connection)
        .expect("Error loading users");
    println!("{:?}", &kid.first());
    match kid.len() {
        0 => None,
        _ => Some(kid[0].clone()),
    }
}

pub async fn create_kid(user: &User, name: String) -> Kid {
    let connection = &mut establish_connection();
    let new_kid = NewKid {
        name: name,
        user_id: user.id,
    };

    insert_into(kids)
        .values(&new_kid)
        .execute(connection)
        .expect("User should be inserted");
    get_kid(user.id, name).await.expect("User has been created")
}