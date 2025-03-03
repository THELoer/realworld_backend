use actix_web::{web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    username: String,
    email: String,
    password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub token: String,
    pub user: User,
}


pub async fn register(user: web::Json<RegisterRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    println!("started");
    match reg_acc(&pool, &user).await {
        Ok(_) => return HttpResponse::Created().json(user),
        Err(_) => return HttpResponse::InternalServerError().finish()
    }
}


pub async fn reg_acc(pool: &PgPool, form: &RegisterRequest) -> Result<(), sqlx::Error> {
    let uuid = Uuid::new_v4();

    println!("registerrrr");

    let password_hash = hash(&form.password, DEFAULT_COST).unwrap();

    tracing::info!(
        "New account: {}, time: {}, email: {}",
        uuid,
        Utc::now(),
        form.email,
    );

    sqlx::query!(
        r#"
        INSERT INTO accounts (id, email, username, password)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid.to_string(),
        form.email,
        form.username,
        password_hash
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}