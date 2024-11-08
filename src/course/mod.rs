use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

pub mod controller;
pub mod service;
pub mod model;

#[derive(Serialize, Deserialize)]
pub enum CourseServiceRequest {
    CreateCourse {
        image_id: Option<String>,
        title: String,
        description: String,
    }
}

#[derive(Debug)]
pub enum CourseServiceError {
    BadRequest,
    Unauthorized,
}

impl Display for CourseServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CourseServiceError::BadRequest => write!(f, "Bad request"),
            CourseServiceError::Unauthorized => write!(f, "Try create course with unauthorized")
        }
    }
}

impl Error for CourseServiceError {}