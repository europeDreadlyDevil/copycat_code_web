use crate::module::model::{ModuleModel, ModuleModelDto, ModuleModelUpdateDto};
use crate::DataBase;
use std::str::FromStr;
use surrealdb::opt::Resource;
use surrealdb::sql::Thing;

pub struct ModuleService;

impl ModuleService {
    pub async fn get_all_modules_in_course(owner_id: &String, db: DataBase) -> anyhow::Result<Vec<ModuleModel>> {
        let modules: Vec<ModuleModel> = db
            .query("SELECT * FROM module WHERE course_id = $course_id")
            .bind(("course_id", owner_id))
            .await?
            .take(0)?;
        Ok(modules)
    }
}

impl ModuleService {
    pub async fn create_module(
        dto: ModuleModelDto,
        db: DataBase,
    ) -> anyhow::Result<Option<ModuleModel>> {
        let res: Option<ModuleModel> = db
            .create("module")
            .content(ModuleModel {
                id: None,
                course_id: dto.course_id,
                title: dto.title,
                lectures: vec![],
            })
            .await?;
        Ok(res)
    }

    pub async fn get_module_by_id(id: &str, db: DataBase) -> anyhow::Result<Option<ModuleModel>> {
        Ok(db
            .query("SELECT * FROM course WHERE id = $id")
            .bind(("id", Thing::from_str(id).unwrap()))
            .await?
            .take(0)?)
    }

    pub async fn update_module(
        id: &str,
        dto: ModuleModelUpdateDto,
        db: DataBase,
    ) -> anyhow::Result<()> {
        let mut id = id.split(":");
        db.update(Resource::from((id.nth(0).unwrap(), id.nth(0).unwrap())))
            .merge(dto)
            .await?;
        Ok(())
    }
}
