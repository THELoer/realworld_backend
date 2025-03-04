use serde::{Deserialize, Serialize};

pub mod token;
pub mod registration;
pub mod login;
mod error;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    username: String,
    email: String,
    password: String,
}

