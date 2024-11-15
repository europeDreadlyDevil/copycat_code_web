use crate::lecture::model::LectureModel;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct ModuleModel {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub owner_id: Thing,
    pub course_id: Thing,
    pub title: String,
    pub lectures: Vec<LectureModel>,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleModelDto {
    pub course_id: Thing,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleModelUpdateDto {
    pub title: Option<String>,
    pub description: Option<String>,
}
