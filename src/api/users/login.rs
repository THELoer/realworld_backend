use crate::api::users::error::Error;
use crate::api::users::token::create_token;
use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use bcrypt::verify;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Login {
    user: User,
}

#[derive(Deserialize)]
pub struct User {
    email: String,
    password: String,
}

//TODO: return user{email, username, bio, image, token)
pub async fn login(form: Json<Login>, pool: Data<PgPool>) -> HttpResponse {
    match loginn(&form, &pool).await {
        Ok((id, username)) => {
            let json = json!({ "user": {
                "email": form.user.email,
                "username": username,
                "bio": "",
                "image": "",
                "token": create_token(id).unwrap_or("".to_string()),
            }});

            return HttpResponse::Ok().json(json);
        }
        Err(e) => match e {
            Error::UserDidNotExists(e) => {
                HttpResponse::InternalServerError().json(json!({"error": e}))
            }
            Error::PasswordOrLoginIsIncorrect(e) => {
                HttpResponse::InternalServerError().json(json!({"error": e}))
            }
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

pub async fn loginn(form: &Login, pool: &PgPool) -> Result<(String, String), Error> {
    let query = sqlx::query!(
        "SELECT email, password, username, id FROM accounts WHERE email = $1",
        &form.user.email
    )
    .fetch_one(pool)
    .await;

    if query.is_err() {
        return Err(Error::UserDidNotExists(
            "Пользователь не существует".to_string(),
        ));
    }

    let query = query.unwrap();

    if !verify(&form.user.password, &query.password).unwrap_or(false) {
        return Err(Error::PasswordOrLoginIsIncorrect(
            "Логин или пароль неверный".to_string(),
        ));
    }

    return Ok((query.id, query.username));
}
