use crate::course::model::CourseModuleUpdateDto;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub mod controller;
pub mod model;
pub mod service;

#[derive(Serialize, Deserialize)]
pub enum CourseServiceRequest {
    CreateCourse {
        image_id: Option<String>,
        title: String,
        description: String,
    },
    GetCourseById {
        id: String,
    },
    UpdateCourse {
        id: String,
        dto: CourseModuleUpdateDto,
    },
}

#[derive(Debug)]
pub enum CourseServiceError {
    BadRequest,
    Unauthorized,
    CourseNotFound,
}

impl Display for CourseServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CourseServiceError::BadRequest => write!(f, "Bad request"),
            CourseServiceError::Unauthorized => write!(f, "Trying create course in unauthorized"),
            CourseServiceError::CourseNotFound => write!(f, "Course not found"),
        }
    }
}

impl Error for CourseServiceError {}
