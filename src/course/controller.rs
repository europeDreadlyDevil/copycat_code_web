use crate::course::model::{CourseModel, CourseModelCreateDto};
use crate::course::service::CourseService;
use crate::course::{CourseServiceError, CourseServiceRequest};
use crate::DataBase;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{get, post, put, HttpResponse};
use std::str::FromStr;
use surrealdb::sql::Thing;

pub struct CourseController;

impl CourseController {
    async fn post_course(
        session: Session,
        course_service_request: CourseServiceRequest,
        db: DataBase,
    ) -> anyhow::Result<()> {
        if let CourseServiceRequest::CreateCourse {
            image_id,
            title,
            description,
        } = course_service_request
        {
            if let Some(id) = session.get::<String>("id")? {
                CourseService::create_course(
                    CourseModelCreateDto {
                        owner_id: id,
                        image_id,
                        title,
                        description,
                    },
                    db,
                )
                .await?;
                return Ok(());
            }
            return Err(CourseServiceError::Unauthorized.into());
        }
        Err(CourseServiceError::BadRequest.into())
    }
    async fn get_course_by_id(
        course_service_request: CourseServiceRequest,
        db: DataBase,
    ) -> anyhow::Result<CourseModel> {
        if let CourseServiceRequest::GetCourseById { id } = course_service_request {
            return match CourseService::get_course_by_id(id, db).await? {
                None => Err(CourseServiceError::CourseNotFound.into()),
                Some(course) => Ok(course),
            };
        }
        Err(CourseServiceError::BadRequest.into())
    }

    async fn put_course(
        session: Session,
        course_service_request: CourseServiceRequest,
        db: DataBase,
    ) -> anyhow::Result<()> {
        if let CourseServiceRequest::UpdateCourse { id, dto } = course_service_request {
            if let Some(owner_id) = session.get::<String>("id")? {
                match CourseService::get_course_by_id(id.clone(), db.clone()).await? {
                    None => return Err(CourseServiceError::CourseNotFound.into()),
                    Some(course) => {
                        if course.owner_id == Thing::from_str(&owner_id).unwrap() {
                            return CourseService::update_course(id, dto, db.clone()).await;
                        }
                    }
                }
            }
            return Err(CourseServiceError::Unauthorized.into());
        }
        Err(CourseServiceError::BadRequest.into())
    }
}

#[post("/create-course")]
pub async fn post_course_handler(
    session: Session,
    course_service_request: Json<CourseServiceRequest>,
    db: DataBase,
) -> HttpResponse {
    match CourseController::post_course(session, course_service_request.0, db).await {
        Ok(_) => HttpResponse::Accepted().body("Course created successful"),
        Err(e) => match e.downcast::<CourseServiceError>() {
            Ok(cse) => match cse {
                CourseServiceError::BadRequest => {
                    HttpResponse::BadRequest().body(CourseServiceError::BadRequest.to_string())
                }
                CourseServiceError::Unauthorized => {
                    HttpResponse::Unauthorized().body(CourseServiceError::Unauthorized.to_string())
                }
                _ => HttpResponse::InternalServerError().body(cse.to_string()),
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[get("/get-course")]
pub async fn get_course_by_id_handler(
    course_service_request: Json<CourseServiceRequest>,
    db: DataBase,
) -> HttpResponse {
    match CourseController::get_course_by_id(course_service_request.0, db).await {
        Ok(course) => HttpResponse::Accepted().json(course),
        Err(e) => match e.downcast::<CourseServiceError>() {
            Ok(cse) => match cse {
                CourseServiceError::BadRequest => {
                    HttpResponse::BadRequest().body(CourseServiceError::BadRequest.to_string())
                }
                CourseServiceError::CourseNotFound => {
                    HttpResponse::NotFound().body(CourseServiceError::CourseNotFound.to_string())
                }
                _ => HttpResponse::InternalServerError().body(cse.to_string()),
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[put("/update-course")]
pub async fn put_course_handler(
    session: Session,
    course_service_request: Json<CourseServiceRequest>,
    db: DataBase,
) -> HttpResponse {
    match CourseController::put_course(session, course_service_request.0, db).await {
        Ok(_) => HttpResponse::Accepted().body("Course update successful"),
        Err(e) => {
            match e.downcast::<CourseServiceError>() {
                Ok(cse) => match cse {
                    CourseServiceError::BadRequest => {
                        HttpResponse::BadRequest().body(CourseServiceError::BadRequest.to_string())
                    }
                    CourseServiceError::Unauthorized => HttpResponse::Unauthorized()
                        .body(CourseServiceError::Unauthorized.to_string()),
                    CourseServiceError::CourseNotFound => HttpResponse::NotFound()
                        .body(CourseServiceError::CourseNotFound.to_string()),
                    _ => HttpResponse::InternalServerError().body(cse.to_string()),
                },
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}
