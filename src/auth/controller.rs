use crate::auth::{AuthError, AuthRequest};
use crate::user::model::UserModelDto;
use crate::user::service::UserService;
use crate::DataBase;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{delete, get, post, HttpResponse};
use anyhow::Error;
use base64ct::{Base64, Encoding};
use std::io::Write;
use whirlpool::{Digest, Whirlpool};

pub struct AuthController;

impl AuthController {
    async fn register(
        session: Session,
        auth_request: AuthRequest,
        db: DataBase,
    ) -> anyhow::Result<()> {
        if let AuthRequest::Registration {
            login,
            password,
            email,
        } = auth_request
        {
            if let None = UserService::get_user(&login, db.clone()).await? {
                let mut hasher = Whirlpool::default();
                hasher.write(password.as_bytes())?;
                let hashed_pass = Base64::encode_string(&hasher.finalize());
                UserService::create_user(
                    UserModelDto {
                        login: login.clone(),
                        password: hashed_pass,
                        email: email.clone(),
                    },
                    db,
                )
                .await?;
                let _ = session.insert("login", login);
                let _ = session.insert("email", email);
            } else {
                return Err(AuthError::LoginAlreadyExists.into());
            }
        }
        Ok(AuthError::BadRequest.into())
    }
    async fn login(
        session: Session,
        auth_request: AuthRequest,
        db: DataBase,
    ) -> anyhow::Result<()> {
        if let AuthRequest::Login { login, password } = auth_request {
            match UserService::get_user(&login, db.clone()).await? {
                None => return Err(Error::msg(AuthError::LoginIsInvalid)),
                Some(user) => {
                    let mut hasher = Whirlpool::default();
                    hasher.write(password.as_bytes())?;
                    let hashed_pass = Base64::encode_string(&hasher.finalize());
                    if user.password == hashed_pass {
                        let _ = session.insert("login", login);
                        let _ = session.insert("email", user.email);
                    } else {
                        return Err(AuthError::PasswordIsInvalid.into());
                    }
                }
            }
        }
        Err(AuthError::BadRequest.into())
    }
    async fn logout(session: Session) -> anyhow::Result<()> {
        session.purge();
        Ok(())
    }
}

#[post("auth/registration")]
pub async fn register_handler(
    session: Session,
    auth_request: Json<AuthRequest>,
    db: DataBase,
) -> HttpResponse {
    match AuthController::register(session, auth_request.0, db).await {
        Ok(_) => HttpResponse::Accepted().body(""),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[get("auth/login")]
pub async fn login_handler(
    session: Session,
    auth_request: Json<AuthRequest>,
    db: DataBase,
) -> HttpResponse {
    match AuthController::login(session, auth_request.0, db).await {
        Ok(_) => HttpResponse::Accepted().body("Login success"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[delete("auth/logout")]
pub async fn logout_handler(session: Session) -> HttpResponse {
    match AuthController::logout(session).await {
        Ok(_) => HttpResponse::Accepted().body("Logout success"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
