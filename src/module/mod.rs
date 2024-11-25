use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::module::model::ModuleModelUpdateDto;

pub mod controller;
pub mod model;
pub mod service;

#[derive(Serialize, Deserialize)]
pub enum ModuleServiceRequest {
    CreateModule {
        course_id: String,
        title: String,
        description: String,
    },
    GetModule {
        id: String,
    },
    UpdateModule {
        id: String,
        dto: ModuleModelUpdateDto
    },
    DeleteModule { id: String },
}

#[derive(Debug)]
pub enum ModuleServiceError {
    BadRequest,
    IsNotCourseOwner,
    UnauthorizedRequest,
    ModuleNotFound,
}

impl Display for ModuleServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleServiceError::BadRequest => write!(f, "Bad request"),
            ModuleServiceError::IsNotCourseOwner => write!(
                f,
                "Trying add module to course, when you is not course owner"
            ),
            ModuleServiceError::UnauthorizedRequest => {
                write!(f, "Trying create module in unauthorized")
            }
            ModuleServiceError::ModuleNotFound => write!(f, "Module not found"),
        }
    }
}

impl Error for ModuleServiceError {}
