use crate::practice::model::PracticeModel;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct LectureModel {
    id: Option<Thing>,
    title: String,
    description: String,
    content: String,
    modules: Vec<PracticeModel>,
}
