use std::str::FromStr;
use surrealdb::sql::Thing;
use crate::course::model::{CourseModel, CourseModelDto};
use crate::DataBase;

pub struct CourseService;

impl CourseService {
    pub async fn get_course_by_id(id: String, db: DataBase) -> anyhow::Result<Option<CourseModel>> {
        Ok(
            db
                .query("SELECT * FROM course WHERE id = $id")
                .bind(id).await?
                .take(0)?
        )
    }

    pub async fn create_course(dto: CourseModelDto, db: DataBase) -> anyhow::Result<()> {
        let _ : Option<CourseModel> = db
            .create("course")
            .content(
                CourseModel {
                    id: None,
                    owner_id: Thing::from_str(&dto.owner_id).unwrap(),
                    image_id: if dto.image_id.is_some() { Some(Thing::from_str(&dto.image_id.unwrap()).unwrap()) } else { None },
                    title: dto.title,
                    description: dto.description,
                    rating: 0.0,
                    lectures: vec![],
                }
            ).await?;
        Ok(())
    }
}