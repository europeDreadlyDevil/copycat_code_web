use crate::module::model::{ModuleModel, ModuleModelDto};
use crate::DataBase;

pub struct ModuleService;

impl ModuleService {
    pub async fn create_module(dto: ModuleModelDto, db: DataBase) -> anyhow::Result<()> {
        let _: Option<ModuleModel> = db
            .create("module")
            .content(ModuleModel {
                id: None,
                course_id: dto.course_id,
                title: dto.title,
                lectures: vec![],
            })
            .await?;
        Ok(())
    }
}
