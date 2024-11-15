use crate::course::model::{CourseModel, CourseModelUpdateDto};
use crate::course::service::CourseService;
use crate::course::CourseServiceError;
use crate::module::model::{ModuleModel, ModuleModelDto};
use crate::module::service::ModuleService;
use crate::module::{ModuleServiceError, ModuleServiceRequest};
use crate::DataBase;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{get, post, put, HttpResponse};
use anyhow::Error;
use std::str::FromStr;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub struct ModuleController;

impl ModuleController {
    pub(crate) async fn delete_module(session: Session, module_service_request: ModuleServiceRequest, db: DataBase) -> anyhow::Result<()> {
        
    }
}

impl ModuleController {
    async fn post_module(
        session: Session,
        module_service_request: ModuleServiceRequest,
        db: DataBase,
    ) -> anyhow::Result<()> {
        if let ModuleServiceRequest::CreateModule {
            course_id,
            title,
            description,
        } = module_service_request
        {
            if let Some(id) = session.get::<String>("id")? {
                return match CourseService::get_course_by_id(&course_id, db.clone()).await? {
                    None => Err(CourseServiceError::CourseNotFound.into()),
                    Some(mut course) => {
                        if course.owner_id == Thing::from_str(&id).unwrap() {
                            match ModuleService::create_module(
                                ModuleModelDto {
                                    course_id: course.id.unwrap(),
                                    title,
                                    description,
                                },
                                db.clone(),
                            )
                            .await?
                            {
                                None => Err(Error::msg("Fail to create module")),
                                Some(module) => {
                                    course.modules.push(module);
                                    CourseService::update_course(
                                        &course_id,
                                        CourseModelUpdateDto {
                                            modules: Some(course.modules),
                                            ..Default::default()
                                        },
                                        db.clone(),
                                    )
                                    .await?;
                                    Ok(())
                                }
                            }
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

    async fn get_module(
        module_service_request: ModuleServiceRequest,
        db: DataBase,
    ) -> anyhow::Result<ModuleModel> {
        if let ModuleServiceRequest::GetModule { id } = module_service_request {
            return match ModuleService::get_module_by_id(&id, db.clone()).await? {
                None => Err(ModuleServiceError::ModuleNotFound.into()),
                Some(module) => Ok(module),
            };
        }
        Err(ModuleServiceError::BadRequest.into())
    }

    async fn put_module(
        session: Session,
        module_service_request: ModuleServiceRequest,
        db: DataBase
    ) -> anyhow::Result<()> {
        if let ModuleServiceRequest::UpdateModule {id, dto} = module_service_request {
            if let Some(user_id) = session.get::<String>("id")? {
                let module = match ModuleService::get_module_by_id(&id, db.clone()).await {
                    Ok(module) => match module {
                        None => return Err(ModuleServiceError::ModuleNotFound.into()),
                        Some(module) => module
                    }
                    Err(e) => return Err(e)
                };
                let course = match CourseService::get_course_by_id(&module.course_id.to_string(), db.clone()).await {
                    Ok(course) => match course {
                        None => return Err(CourseServiceError::CourseNotFound.into()),
                        Some(course) => course,
                    }
                    Err(e) => return Err(e)
                };
                if course.owner_id.to_string() == user_id {
                    return ModuleService::update_module(&user_id, dto, db).await
                }
            }
            return Err(ModuleServiceError::UnauthorizedRequest.into())
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
                _ => HttpResponse::InternalServerError().body(mse.to_string()),
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[get("/get-module")]
pub async fn get_module_by_id_handler(
    module_service_request: Json<ModuleServiceRequest>,
    db: DataBase,
) -> HttpResponse {
    match ModuleController::get_module(module_service_request.0, db.clone()).await {
        Ok(module) => HttpResponse::Accepted().json(module),
        Err(e) => match e.downcast::<ModuleServiceError>() {
            Ok(mse) => match mse {
                ModuleServiceError::BadRequest => HttpResponse::BadRequest().body(mse.to_string()),
                ModuleServiceError::ModuleNotFound => {
                    HttpResponse::NotFound().body(mse.to_string())
                }
                _ => HttpResponse::InternalServerError().body(mse.to_string()),
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[put("/update-module")]
pub async fn put_module_handler(
    session: Session,
    module_service_request: Json<ModuleServiceRequest>,
    db: DataBase
) -> HttpResponse {
    match ModuleController::put_module(session, module_service_request.0, db).await {
        Ok(_) => HttpResponse::Accepted().body("Module update successful"),
        Err(e) => match e.downcast::<ModuleServiceError>() {
            Ok(mse) => match mse {
                ModuleServiceError::BadRequest => HttpResponse::BadRequest().body(mse.to_string()),
                ModuleServiceError::UnauthorizedRequest => HttpResponse::Unauthorized().body(mse.to_string()),
                _ => HttpResponse::InternalServerError().body(mse.to_string()),
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}