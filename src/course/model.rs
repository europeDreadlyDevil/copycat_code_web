use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::lecture::model::LectureModel;

#[derive(Serialize, Deserialize)]
pub struct CourseModel {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub owner_id: Thing,
    pub image_id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub rating: f32,
    pub lectures: Vec<LectureModel>
}

#[derive(Serialize, Deserialize)]
pub struct CourseModelDto {
    pub owner_id: String,
    pub image_id: Option<String>,
    pub title: String,
    pub description: String,
}