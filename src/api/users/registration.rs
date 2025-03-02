use actix_web::{web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct User {
    username: String,
    email: String,
    password: String,
}

pub async fn register(form: web::Json<User>, pool: web::Data<PgPool>) -> HttpResponse {
    match reg_acc(&pool, &form).await {
        Ok(_) => {
            
        }
    }
}


pub async fn reg_acc(pool: &PgPool, form: &User) -> Result<(), sqlx::Error> {
    let uuid = Uuid::new_v4();

    let password_hash = hash(&form.password, DEFAULT_COST).unwrap();

    tracing::info!(
        "New account: {}, time: {}, email: {}",
        uuid,
        Utc::now(),
        form.email,
    );

    sqlx::query!(
        r#"
        INSERT INTO accounts (id, email, username, password
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