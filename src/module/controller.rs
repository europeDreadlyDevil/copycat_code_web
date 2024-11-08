use crate::course::service::CourseService;
use crate::course::CourseServiceError;
use crate::module::model::ModuleModelDto;
use crate::module::service::ModuleService;
use crate::module::{ModuleServiceError, ModuleServiceRequest};
use crate::DataBase;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use std::str::FromStr;
use surrealdb::sql::Thing;

pub struct ModuleController;

impl ModuleController {
    async fn post_module(
        session: Session,
        module_service_request: ModuleServiceRequest,
        db: DataBase,
    ) -> anyhow::Result<()> {
        if let ModuleServiceRequest::CreateModule { course_id, title } = module_service_request {
            if let Some(id) = session.get::<String>("id")? {
                return match CourseService::get_course_by_id(course_id, db.clone()).await? {
                    None => Err(CourseServiceError::CourseNotFound.into()),
                    Some(course) => {
                        if course.owner_id == Thing::from_str(&id).unwrap() {
                            ModuleService::create_module(
                                ModuleModelDto {
                                    course_id: course.id.unwrap(),
                                    title,
                                },
                                db,
                            )
                            .await?;
                            Ok(())
                        } else {
                            Err(ModuleServiceError::IsNotCourseOwner.into())
                        }
                    }
                };
            }
            return Err(ModuleServiceError::UnauthorizedRequest.into());
        }
        Err(ModuleServiceError::BadRequest.into())
    }
}

#[post("/create-module")]
pub async fn post_module_handler(
    session: Session,
    module_service_request: Json<ModuleServiceRequest>,
    db: DataBase,
) -> HttpResponse {
    match ModuleController::post_module(session, module_service_request.0, db).await {
        Ok(_) => HttpResponse::Created().body("Module created successful"),
        Err(e) => match e.downcast::<ModuleServiceError>() {
            Ok(mse) => match mse {
                ModuleServiceError::BadRequest => HttpResponse::BadRequest().body(mse.to_string()),
                ModuleServiceError::IsNotCourseOwner => {
                    HttpResponse::Conflict().body(mse.to_string())
                }
                ModuleServiceError::UnauthorizedRequest => {
                    HttpResponse::Unauthorized().body(mse.to_string())
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}
