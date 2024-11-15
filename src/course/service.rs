use crate::course::model::{CourseModel, CourseModelCreateDto, CourseModelUpdateDto};
use crate::DataBase;
use std::str::FromStr;
use surrealdb::opt::Resource;
use surrealdb::sql::Thing;

pub struct CourseService;

impl CourseService {
    pub async fn get_course_by_id(id: &str, db: DataBase) -> anyhow::Result<Option<CourseModel>> {
        Ok(db
            .query("SELECT * FROM course WHERE id = $id")
            .bind(("id", Thing::from_str(id).unwrap()))
            .await?
            .take(0)?)
    }

    pub async fn get_course_by_owner_id(
        owner_id: String,
        db: DataBase,
    ) -> anyhow::Result<Option<CourseModel>> {
        Ok(db
            .query("SELECT * FROM course WHERE owner_id = $owner_id")
            .bind(("owner_id", Thing::from_str(&owner_id).unwrap()))
            .await?
            .take(0)?)
    }

    pub async fn create_course(dto: CourseModelCreateDto, db: DataBase) -> anyhow::Result<()> {
        let _: Option<CourseModel> = db
            .create("course")
            .content(CourseModel {
                id: None,
                owner_id: Thing::from_str(&dto.owner_id).unwrap(),
                image_id: if dto.image_id.is_some() {
                    Some(Thing::from_str(&dto.image_id.unwrap()).unwrap())
                } else {
                    None
                },
                title: dto.title,
                description: dto.description,
                rating: 0.0,
                modules: vec![],
            })
            .await?;
        Ok(())
    }

    pub async fn update_course(
        id: &str,
        dto: CourseModelUpdateDto,
        db: DataBase,
    ) -> anyhow::Result<()> {
        let mut id = id.split(":");
        db.update(Resource::from((id.nth(0).unwrap(), id.nth(0).unwrap())))
            .merge(dto)
            .await?;
        Ok(())
    }

    pub async fn get_course_list(db: DataBase) -> anyhow::Result<Vec<CourseModel>> {
        Ok(db.select("course").await?)
    }

    pub async fn delete_course(id: &str, db: DataBase) -> anyhow::Result<()> {
        let mut id = id.split(":");
        db.delete(Resource::from((id.nth(0).unwrap(), id.nth(0).unwrap()))).await?;
        Ok(())
    }
}
