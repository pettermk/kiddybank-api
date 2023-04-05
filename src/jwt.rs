use alcoholic_jwt::{JWKS, ValidJWT, Validation, validate, token_kid};
use reqwest;
use rocket::serde::json::serde_json;
use cached::proc_macro::cached;

use std::env;

#[cached]
async fn get_jwks() -> JWKS {
    let resp =
        reqwest::get("https://login.microsoftonline.com/3aa4a235-b6e2-48d5-9195-7fcf05b459b0/discovery/v2.0/keys")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    serde_json::from_str(&resp).expect("Should deserialize")
}


pub(crate) async fn process_jwt(token: &str) -> ValidJWT {
    // The function implied here would usually perform an HTTP-GET
    // on the JWKS-URL for an authentication provider and deserialize
    // the result into the `alcoholic_jwt::JWKS`-struct.
    let jwks: JWKS = get_jwks().await;

    let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");
    let validations;
    if environment == "local" {
        validations = vec![];
    } else {
        validations = vec![
            Validation::Issuer("https://sts.windows.net/3aa4a235-b6e2-48d5-9195-7fcf05b459b0/".into()),
            Validation::SubjectPresent,
            Validation::NotExpired,
        ];
    }

    // If a JWKS contains multiple keys, the correct KID first
    // needs to be fetched from the token headers.
    let kid = token_kid(&token)
        .expect("Failed to decode token headers")
        .expect("No 'kid' claim present in token");

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    validate(token, jwk, validations).expect("Token validation has failed!")
}