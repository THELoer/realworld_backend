pub mod token;
mod registration;


#[derive(serde::Serialize)]
pub struct user {
    email: String,
    token: String,
    username: String,
}