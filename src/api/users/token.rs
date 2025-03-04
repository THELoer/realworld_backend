use actix_web::{HttpRequest, HttpResponse, HttpServer, Responder, web};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde_json::json;
use sqlx::PgPool;
use std::env;
use std::fmt::Formatter;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub enum Error {
    TokenIsExpired(String),
    AccountDoesNotExist(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccountDoesNotExist(e) => write!(f, "{}", e),
            Error::TokenIsExpired(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct User {
    username: String,
    id: String,
    email: String,
    password: String,
}

pub fn create_token(id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now().timestamp() + 86400;
    let claims = Claims {
        sub: id,
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET not found")
                .as_ref(),
        ),
    )
}

pub fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET not found")
                .as_ref(),
        ),
        &Validation::default(),
    )
}

pub async fn verify_token_handler(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    let auth_header = req.headers().get("Authorization");
    let token = match auth_header {
        Some(header) => {
            let header_str = header.to_str().map_err(|e| {
                eprintln!("Invalid header: {}", e);
                HttpResponse::BadRequest().finish()
            });
            let header_str = header_str.unwrap();
            if header_str.starts_with("Token ") {
                header_str[6..].to_string()
            } else {
                return HttpResponse::BadRequest().json(json!({
                    "error": "Invalid Authorization header format. User 'Token <token>'"
                }));
            }
        }
        None => {
            return HttpResponse::Unauthorized().json(json!({
                "error": "No Auth header provided"
            }));
        }
    };

    match verify_token(&token) {
        Ok(token_data) => {
            let user_id = token_data.claims.sub;

            let user = sqlx::query_as!(User, "SELECT * FROM accounts WHERE id = $1", user_id)
                .fetch_one(pool.get_ref())
                .await
                .map_err(|e| {
                    eprintln!("Database errror: {}", e);
                    HttpResponse::InternalServerError().finish()
                });

            HttpResponse::Ok().json(json!({
                "token": token
            }))
        }
        Err(e) => {
            eprintln!("Token verification error: {}", e);
            HttpResponse::Unauthorized().json(json!({
                "error": "Invalid or expired token"
            }))
        }
    }
}
