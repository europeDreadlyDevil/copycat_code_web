use crate::course::model::{CourseModel, CourseModelCreateDto};
use crate::course::service::CourseService;
use crate::course::{CourseServiceError, CourseServiceRequest};
use crate::DataBase;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{delete, get, post, put, HttpResponse};
use std::str::FromStr;
use anyhow::Error;
use surrealdb::rpc::Data;
use surrealdb::sql::Thing;
use crate::module::controller::ModuleController;
use crate::module::ModuleServiceRequest;
use crate::module::service::ModuleService;

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
            return match CourseService::get_course_by_id(&id, db).await? {
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
                match CourseService::get_course_by_id(&id, db.clone()).await? {
                    None => return Err(CourseServiceError::CourseNotFound.into()),
                    Some(course) => {
                        if course.owner_id == Thing::from_str(&owner_id).unwrap() {
                            return CourseService::update_course(&id, dto, db.clone()).await;
                        }
                    }
                }
            }
            return Err(CourseServiceError::Unauthorized.into());
        }
        Err(CourseServiceError::BadRequest.into())
    }

    async fn get_course_list(db: DataBase) -> anyhow::Result<Vec<CourseModel>> {
        CourseService::get_course_list(db).await
    }

    async fn delete_course(session: Session, course_service_request: CourseServiceRequest, db: DataBase) -> anyhow::Result<()> {
        if let CourseServiceRequest::DeleteCourse {id} = course_service_request {
            if let Some(owner_id) = session.get::<String>("id")? {
                return match CourseService::get_course_by_id(&id, db.clone()).await? {
                    None => Err(CourseServiceError::CourseNotFound.into()),
                    Some(course) => if course.owner_id.to_string() == owner_id {
                        let modules = ModuleService::get_all_modules_in_course(&id, db).await?;
                        if !modules.is_empty() {
                            for module in modules {
                                ModuleController::delete_module(session.clone(), ModuleServiceRequest::DeleteModule {id: module.id.unwrap().to_string()}, db.clone()).await?;
                            }
                        }
                        CourseService::delete_course(&id, db.clone()).await?;
                        Ok(())
                    } else {
                        Err(CourseServiceError::Unauthorized.into())
                    }
                }
            }
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

#[get("/get-all-courses")]
pub async fn get_course_list_handler(db: DataBase) -> HttpResponse {
    match CourseController::get_course_list(db).await {
        Ok(courses) => HttpResponse::Accepted().json(courses),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/delete-course")]
pub async fn delete_course(
    session: Session,
    course_service_request: Json<CourseServiceRequest>,
    db: DataBase
) -> HttpResponse {
    match CourseController::delete_course(session, course_service_request.0, db).await {
        Ok(_) => HttpResponse::Accepted().body("Course deleted successful"),
        Err(e) => match e.downcast::<CourseServiceError>() {
            Ok(cse) => match cse {
                CourseServiceError::BadRequest => HttpResponse::BadRequest().body(cse.to_string()),
                CourseServiceError::CourseNotFound => HttpResponse::NotFound().body(cse.to_string()),
                CourseServiceError::Unauthorized => HttpResponse::Unauthorized().body(cse.to_string())
            }
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}