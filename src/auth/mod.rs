use std::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod controller;

#[derive(Deserialize, Serialize)]
pub enum AuthRequest {
    Registration {
        login: String,
        password: String,
        email: String,
    },
    Login {
        login: String,
        password: String,
    },
}

#[derive(Debug)]
pub enum AuthError {
    LoginAlreadyExists,
    LoginIsInvalid,
    PasswordIsInvalid,
    BadRequest
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::LoginAlreadyExists => write!(f, "Login already exists"),
            AuthError::LoginIsInvalid => write!(f, "Login is invalid"),
            AuthError::PasswordIsInvalid => write!(f, "Password is invalid"),
            AuthError::BadRequest => write!(f, "Bad request")
        }
    }
}

impl Error for AuthError {}