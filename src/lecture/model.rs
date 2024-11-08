use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct LectureModel {
    id: Option<Thing>,
    title: String,
    //modules: Vec<ModuleModel>
}
