use crate::user::model::{UserModel, UserModelDto};
use crate::DataBase;
use actix_web::web::Data;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub struct UserService;

impl UserService {
    pub async fn create_user(dto: UserModelDto, db: Data<Surreal<Client>>) -> anyhow::Result<()> {
        let _: Option<UserModel> = db
            .create("user")
            .content(UserModel {
                id: None,
                login: dto.login,
                email: dto.email,
                password: dto.password,
            })
            .await?;
        Ok(())
    }
    pub async fn get_user(login: &str, db: DataBase) -> anyhow::Result<Option<UserModel>> {
        let user: Option<UserModel> = db
            .query("SELECT * FROM user WHERE login = $login")
            .bind(("login", login.to_string()))
            .await?
            .take(0)?;
        Ok(user)
    }
}
