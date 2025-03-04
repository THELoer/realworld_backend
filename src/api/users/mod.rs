use serde::{Deserialize, Serialize};

mod error;
pub mod login;
pub mod registration;
pub mod token;
pub mod user;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    username: String,
    email: String,
    password: String,
}
