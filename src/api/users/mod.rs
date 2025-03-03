pub mod token;
pub mod registration;


#[derive(serde::Serialize)]
pub struct user {
    email: String,
    token: String,
    username: String,
}