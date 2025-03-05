use crate::api::users::token::{Error, verify_token};
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, query_as, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    username: String,
    id: String,
    email: String,
    bio: String,
    image: String,
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
                "SELECT id, email, username, bio, image FROM accounts WHERE id = $1",
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



pub async fn update_user(req: HttpRequest, email: web::Json<serde_json::Value>, pool: Data<PgPool>) -> HttpResponse {
    let email = email.get("user")
    .and_then(|u| u.get("email"))
    .and_then(|e| e.as_str());

    if email.is_none() {return HttpResponse::BadRequest().finish()}
    let email = email.unwrap().to_string();


    match update(req, email, &pool).await {
        Ok((user, token)) => return HttpResponse::Ok().json(json!({
            "user": {
                "email": user.email,
                "username": user.username,
                "bio": user.bio,
                "image": user.image,
                "token": token,
            }
        })),
        Err(e) => return HttpResponse::Unauthorized().json(json!({"error": e.to_string()}))
    }
}


async fn update(req: HttpRequest, email: String, pool: &PgPool) -> Result<(User, String), Error> {
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
                "SELECT id, email, username, bio, image FROM accounts WHERE email = $1",
                email,
            )
                .fetch_one(pool)
                .await;

            if user.is_err() {
                println!("user ERROR: {:?}", user);
                return Err(Error::AccountDoesNotExist(
                    "Пользователь не существует".to_string(),
                ));
            }

            return Ok((user.unwrap(), token));
        }
        Err(e) => return Err(Error::TokenIsExpired(e.to_string())),
    }
}