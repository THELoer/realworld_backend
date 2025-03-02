use std::env;
use actix_web::{HttpRequest, HttpServer};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}


// FIXME: возможно не будет работать!!!!
pub async fn get_token(pool: &PgPool, req: HttpRequest) -> Result<String, std::io::Error> {
    let auth_header = req.headers().get("Authorization");

    if let Some(header) = auth_header {
        if let Ok(auth_str) = header.to_str() {
            if auth_str.starts_with("Token ") {
                let token = &auth_str[6..];
                let decoding_key = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());

                match decode::<Claims>(token, &decoding_key, &Validation::default()) {
                    Ok(token_data) => {
                        let user_id = token_data.claims.sub;
                        let user_r = sqlx::query!("SELECT * FROM accounts WHERE id = $1", user_id)
                            .fetch_one(pool)
                            .await;
                        if user_r.is_err() {
                            return Err("Нет токена");
                        }

                        return Ok(token.to_string());

                    }
                    Err(e) => return Err("Недействительный токен")
                }
            }
        }
    }
    return Err("Недействительный токен");
}


pub async fn make_token()

