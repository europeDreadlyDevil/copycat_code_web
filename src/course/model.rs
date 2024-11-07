use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct Course {
    #[serde(skip_serializing)]
    id: Option<Thing>,
    title: String,

}