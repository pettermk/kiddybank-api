use diesel::{prelude::*, insert_into};
use alcoholic_jwt::ValidJWT;
use crate::models::{User, NewUser, Kid, NewKid, KidDto, TransactionDto, NewTransaction, Transaction, KidWithBalance};
use kiddybank_api::establish_connection;
use chrono::{Utc};
use crate::schema::users::dsl::users;
use crate::schema::kids::dsl::kids;
use crate::schema::transactions::dsl::transactions;

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
        .expect("Family name should be in token");
    let external_id = jwt.claims.get("sub")
        .expect("Need sub field for external id")
        .as_str()
        .expect("Need sub field for external id");
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
    let uid = jwt.claims.get(String::from("sub")).expect("JWT should have sub");
    let uid = uid.as_str().expect("Sub should be string");

    let user: Option<User> = get_user(String::from(uid)).await;
    print!("{:?}", user);
    match user {
        Some(u) => u,
        None => create_user(jwt).await,
    }
}

async fn get_balance(kid: &Kid) -> f64 {
    use crate::schema::transactions::dsl::*;
    let connection = &mut establish_connection();
    let balance = transactions
        .filter(kid_id.eq(&kid.id))
        .select(diesel::dsl::sum(change))
        .first::<Option<f64>>(connection)
        .expect("Error loading transactions");
    match balance {
        Some(b) => b,
        None => 0.0,
    }
}

pub async fn get_kids_with_balance(user: &User) -> Vec<KidWithBalance> {
    use crate::schema::kids::dsl::*;
    let connection = &mut establish_connection();
    let all_kids = kids
        .filter(user_id.eq(&user.id))
        .load::<Kid>(connection)
        .expect("Error loading users");
    let mut retval: Vec<KidWithBalance> = Vec::new();
    for kid in all_kids {
        let balance = get_balance(&kid).await;
        retval.push(KidWithBalance {
            id: kid.id,
            name: kid.name,
            user_id: kid.user_id,
            balance: balance,
        });
    }
    retval
}

async fn get_kid(user: &User, kid_name: String) -> Option<Kid> {
    use crate::schema::kids::dsl::*;
    let connection = &mut establish_connection();
    let kid = kids
        .filter(user_id.eq(&user.id))
        .filter(name.eq(&kid_name))
        .limit(1)
        .load::<Kid>(connection)
        .expect("Error loading users");
    println!("{:?}", &kid.first());
    match kid.len() {
        0 => None,
        _ => Some(kid[0].clone()),
    }
}

async fn get_kid_by_id(user: &User, kid_db_id: &i32) -> Option<Kid> {
    use crate::schema::kids::dsl::*;
    let connection = &mut establish_connection();
    let kid = kids
        .filter(user_id.eq(&user.id))
        .filter(id.eq(&kid_db_id))
        .limit(1)
        .load::<Kid>(connection)
        .expect("Error loading users");
    println!("{:?}", &kid.first());
    match kid.len() {
        0 => None,
        _ => Some(kid[0].clone()),
    }
}

pub async fn get_transactions(user: &User, kid_db_id: &i32) -> Vec<Transaction> {
    use crate::schema::transactions::dsl::*;
    let connection = &mut establish_connection();
    let kid = get_kid_by_id(user, kid_db_id).await;
    if kid.is_none() {
        // TODO unauthorized
        return Vec::new();
    }
    transactions
        .filter(kid_id.eq(&kid_db_id))
        .load::<Transaction>(connection)
        .expect("Error loading users")
}

pub async fn create_kid(user: &User, new_kid: &NewKid) -> Kid {
    let connection = &mut establish_connection();
    let kid_dto = KidDto {
        name: new_kid.name.clone(),
        user_id: user.id,
    };

    insert_into(kids)
        .values(&kid_dto)
        .execute(connection)
        .expect("Kid should be inserted");
    get_kid(&user, new_kid.name.clone()).await.expect("Kid has been created")
}

pub async fn create_transaction(user: &User, new_transaction: &NewTransaction) -> () {
    let connection = &mut establish_connection();
    let kid = get_kid_by_id(user, &new_transaction.kid_id).await;
    if kid.is_none() {
        // TODO unauthorized
        return ();
    }
    let transaction_dto =TransactionDto {
        ts: Utc::now().naive_utc(),
        change: new_transaction.change,
        kid_id: new_transaction.kid_id,
        description: new_transaction.description.clone(),
    };

    insert_into(transactions)
        .values(&transaction_dto)
        .execute(connection)
        .expect("Transaction should be inserted");
}