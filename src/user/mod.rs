use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod controller;
pub mod model;
pub mod service;

#[derive(Serialize, Deserialize)]
pub enum UserServiceResponse {
    Create {
        login: String,
        email: String,
        password: String,
    },
    GetByLogin {
        login: String,
    },
}

#[derive(Debug)]
pub enum UserServiceError {
    UnauthorizedRequest,
    BadCreateRequest,
    UserNotFound,
}

impl Display for UserServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserServiceError::UnauthorizedRequest => {
                write!(f, "Try to get user data without authorization")
            }
            UserServiceError::BadCreateRequest => write!(f, "Bad request to create user"),
            UserServiceError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl Error for UserServiceError {}
