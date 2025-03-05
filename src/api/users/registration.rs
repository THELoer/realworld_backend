use crate::api::users::User;
use crate::api::users::token::create_token;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use bcrypt::{DEFAULT_COST, hash};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// #[derive(Deserialize, Serialize, Debug)]
// pub struct User {
//     username: String,
//     email: String,
//     password: String,
// }
//
//
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    user: User,
}

pub async fn register(user: Json<RegisterRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    println!("started");
    match reg_acc(&pool, &user).await {
        Ok(token) => {
            let json = serde_json::json!({"user": {
                "email": user.user.email,
                "username": user.user.username,
                "bio": "",
                "image": "",
                "token": create_token(token).unwrap_or("".to_string()),
            }});

            return HttpResponse::Created().json(json);
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn reg_acc(pool: &PgPool, form: &RegisterRequest) -> Result<String, sqlx::Error> {
    let uuid = Uuid::new_v4();

    println!("registerrrr");

    let password_hash = hash(&form.user.password, DEFAULT_COST).unwrap();

    tracing::info!(
        "New account: {}, time: {}, email: {}",
        uuid,
        Utc::now(),
        form.user.email,
    );

    sqlx::query!(
        r#"
        INSERT INTO accounts (id, email, username, password, bio, image)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        uuid.to_string(),
        form.user.email,
        form.user.username,
        password_hash,
        String::from(""),
        String::from(""),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(uuid.to_string())
}
