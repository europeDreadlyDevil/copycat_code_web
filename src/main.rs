mod auth;
mod db_connector;
pub mod user;
pub mod course;
pub mod lecture;

use crate::auth::controller::{login_handler, logout_handler, register_handler};
use crate::db_connector::init_db_connection;
use crate::user::controller::{get_user_handler, post_user_handler};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, App, HttpServer};
use anyhow::Result;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::course::controller::post_course_handler;

pub type DataBase = Data<Surreal<Client>>;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let db = init_db_connection().await?;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(true)
                    .build(),
            )
            .service(hello)
            .service(post_user_handler)
            .service(get_user_handler)
            .service(register_handler)
            .service(login_handler)
            .service(logout_handler)
            .service(post_course_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world"
}
