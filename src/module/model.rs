use crate::lecture::model::LectureModel;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct ModuleModel {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub course_id: Thing,
    pub title: String,
    pub lectures: Vec<LectureModel>,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleModelDto {
    pub course_id: Thing,
    pub title: String,
}
