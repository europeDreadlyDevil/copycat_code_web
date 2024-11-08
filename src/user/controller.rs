use crate::user::model::{UserModel, UserModelDto};
use crate::user::service::UserService;
use crate::user::{UserServiceError, UserServiceResponse};
use crate::DataBase;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{get, post, HttpResponse};

struct UserController {}

impl UserController {
    async fn post_user(
        user_service_response: UserServiceResponse,
        db: DataBase,
    ) -> anyhow::Result<()> {
        if let UserServiceResponse::Create {
            login,
            email,
            password,
        } = user_service_response
        {
            UserService::create_user(
                UserModelDto {
                    login,
                    email,
                    password,
                },
                db,
            )
            .await?;
            return Ok(());
        }
        Err(UserServiceError::BadCreateRequest.into())
    }

    async fn get_user(session: Session, db: DataBase) -> anyhow::Result<UserModel> {
        if let Some(login) = session.get::<String>("login")? {
            if let Ok(user) = UserService::get_user(&login, db).await {
                return match user {
                    None => Err(UserServiceError::UserNotFound.into()),
                    Some(user) => Ok(user),
                };
            }
        }
        Err(UserServiceError::UnauthorizedRequest.into())
    }
}

#[post("/create-user")]
pub async fn post_user_handler(
    user_service_response: Json<UserServiceResponse>,
    db: DataBase,
) -> HttpResponse {
    match UserController::post_user(user_service_response.0, db).await {
        Ok(_) => HttpResponse::Accepted().body("User created successful"),
        Err(e) => match e.downcast::<UserServiceError>().unwrap() {
            UserServiceError::BadCreateRequest => {
                HttpResponse::BadRequest().body(UserServiceError::BadCreateRequest.to_string())
            }
            _ => HttpResponse::InternalServerError().await.unwrap(),
        },
    }
}

#[get("/get-user")]
pub async fn get_user_handler(session: Session, db: DataBase) -> HttpResponse {
    match UserController::get_user(session, db).await {
        Ok(user) => HttpResponse::Accepted().json(user),
        Err(e) => match e.downcast::<UserServiceError>().unwrap() {
            UserServiceError::UnauthorizedRequest => {
                HttpResponse::Unauthorized().body(UserServiceError::UnauthorizedRequest.to_string())
            }
            UserServiceError::UserNotFound => {
                HttpResponse::NotFound().body(UserServiceError::UserNotFound.to_string())
            }
            _ => HttpResponse::InternalServerError().await.unwrap(),
        },
    }
}
