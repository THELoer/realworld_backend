use std::env;
use actix_web::{HttpRequest, HttpServer};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sqlx::PgPool;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}


pub enum Error {
    TokenIsExpired(String),
    AccountDoesNotExist(String),
}


pub fn create_token(id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now().timestamp() + 86400;
    let claims = Claims {
        sub: id,
        exp: expiration,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET not found").as_ref()))
}

fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET not found").as_ref()),
        &Validation::default()
    )
}


