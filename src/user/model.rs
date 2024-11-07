use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct UserModel {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub login: String,
    pub email: String,
    pub password: String,
}

pub struct UserModelDto {
    pub login: String,
    pub email: String,
    pub password: String,
}
