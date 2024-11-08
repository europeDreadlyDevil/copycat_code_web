use crate::module::model::ModuleModel;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct CourseModel {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub owner_id: Thing,
    pub image_id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub rating: f32,
    pub modules: Vec<ModuleModel>,
}

#[derive(Serialize, Deserialize)]
pub struct CourseModelCreateDto {
    pub owner_id: String,
    pub image_id: Option<String>,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CourseModuleUpdateDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f32>,
}
