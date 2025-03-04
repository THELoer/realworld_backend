use crate::api::users::token::{Error, verify_token};
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, query_as};

#[derive(Deserialize, Serialize)]
pub struct User {
    username: String,
    id: String,
    email: String,
}

pub async fn current_user(req: HttpRequest, pool: Data<PgPool>) -> HttpResponse {
    match check_by_token(req, &pool).await {
        Ok((ok, token)) => {
            return HttpResponse::Ok().json(json!({"user": {
                "email": ok.email,
                "username": ok.username,
                "token": token,
                "image": "",
                "bio": "",
            }}));
        }
        Err(e) => return HttpResponse::Unauthorized().json(json!({"error": e.to_string()})),
    }
}

async fn check_by_token(req: HttpRequest, pool: &PgPool) -> Result<(User, String), Error> {
    let auth_header = req.headers().get("Authorization");
    let token = match auth_header {
        Some(header) => {
            let header_str = header.to_str();
            if header_str.is_err() {
                return Err(Error::TokenIsExpired("Ошибка токена".to_string()));
            }
            let header_str = header_str.unwrap();
            if header_str.starts_with("Token ") {
                header_str[6..].to_string()
            } else {
                return Err(Error::TokenIsExpired("Ошибка токена".to_string()));
            }
        }
        None => return Err(Error::TokenIsExpired("Токена не существует".to_string())),
    };

    match verify_token(&token) {
        Ok(token_data) => {
            let user_id = token_data.claims.sub;

            let user = sqlx::query_as!(
                User,
                "SELECT id, email, username FROM accounts WHERE id = $1",
                user_id
            )
            .fetch_one(pool)
            .await;

            if user.is_err() {
                return Err(Error::AccountDoesNotExist(
                    "Пользователь не существует".to_string(),
                ));
            }

            return Ok((user.unwrap(), token));
        }
        Err(e) => return Err(Error::TokenIsExpired(e.to_string())),
    }
}
