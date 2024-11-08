use actix_session::Session;
use actix_web::{post, HttpResponse};
use actix_web::web::{Data, Json};
use anyhow::Error;
use crate::auth::AuthError;
use crate::course::{CourseServiceError, CourseServiceRequest};
use crate::course::model::CourseModelDto;
use crate::course::service::CourseService;
use crate::DataBase;

pub struct CourseController;

impl CourseController {
    pub async fn post_course(session: Session, course_service_request: CourseServiceRequest, db: DataBase) -> anyhow::Result<()> {
        if let CourseServiceRequest::CreateCourse { image_id, title, description } = course_service_request {
            if let Some(id) = session.get::<String>("id")? {
                CourseService::create_course(
                    CourseModelDto {
                        owner_id: id,
                        image_id,
                        title,
                        description,
                    },
                    db
                ).await?;
                return Ok(())
            }
            return Err(CourseServiceError::Unauthorized.into())
        }
        Err(CourseServiceError::BadRequest.into())
    }
}

#[post("course/create-course")]
pub async fn post_course_handler(session: Session, course_service_request: Json<CourseServiceRequest>, db: DataBase) -> HttpResponse {
    match CourseController::post_course(session, course_service_request.0, db).await {
        Ok(_) => HttpResponse::Accepted().body("Course created successful"),
        Err(e) => match e.downcast::<CourseServiceError>() {
            Ok(cse) => match cse {
                CourseServiceError::BadRequest => HttpResponse::BadRequest().body(CourseServiceError::BadRequest.to_string()),
                CourseServiceError::Unauthorized => HttpResponse::Unauthorized().body(CourseServiceError::Unauthorized.to_string())
            }
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}