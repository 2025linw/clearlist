use async_trait::async_trait;

use super::{
    repo::TaskRepository,
    types::{Model, TaskID, URLQueryOpts},
};
use crate::{
    error::service::Result,
    tag::types::{Model as TagModel, TagID},
    types::date::StartPrecision,
    user::types::UserID,
};

#[async_trait]
pub trait TaskServiceTrait {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>>;
    async fn create(&self, user_id: UserID, create_task: CreateModel) -> Result<Model>;
    async fn get(&self, id: TaskID, user_id: UserID) -> Result<Option<Model>>;
    async fn update(&self, id: TaskID, user_id: UserID, update_task: UpdateModel) -> Result<Model>;

    async fn delete(&self, id: TaskID, user_id: UserID) -> Result<()>;
    async fn restore(&self, id: TaskID, user_id: UserID) -> Result<()>;

    async fn complete(&self, id: TaskID, user_id: UserID) -> Result<()>;
    async fn uncomplete(&self, id: TaskID, user_id: UserID) -> Result<()>;

    async fn list_tags(&self, id: TaskID, user_id: UserID) -> Result<Vec<TagModel>>;
    async fn add_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()>;
    async fn remove_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()>;
    async fn set_tags(
        &self,
        id: TaskID,
        user_id: UserID,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<TagModel>>;
}

struct TaskService<R: TaskRepository> {
    repo: R,
}

#[async_trait]
impl<R> TaskServiceTrait for TaskService<R>
where
    R: TaskRepository,
{
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>> {
        todo!()
    }
    async fn create(&self, user_id: UserID, create_task: CreateModel) -> Result<Model> {
        todo!()
    }
    async fn get(&self, id: TaskID, user_id: UserID) -> Result<Option<Model>> {
        todo!()
    }
    async fn update(&self, id: TaskID, user_id: UserID, update_task: UpdateModel) -> Result<Model> {
        todo!()
    }

    async fn delete(&self, id: TaskID, user_id: UserID) -> Result<()> {
        todo!()
    }
    async fn restore(&self, id: TaskID, user_id: UserID) -> Result<()> {
        todo!()
    }

    async fn complete(&self, id: TaskID, user_id: UserID) -> Result<()> {
        todo!()
    }
    async fn uncomplete(&self, id: TaskID, user_id: UserID) -> Result<()> {
        todo!()
    }

    async fn list_tags(&self, id: TaskID, user_id: UserID) -> Result<Vec<TagModel>> {
        todo!()
    }
    async fn add_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        todo!()
    }
    async fn remove_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        todo!()
    }
    async fn set_tags(
        &self,
        id: TaskID,
        user_id: UserID,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<TagModel>> {
        todo!()
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Default))]
pub struct CreateModel {
    pub title: String,
    pub notes: Option<String>,
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    pub start_precision: StartPrecision,
    pub deadline: Option<chrono::NaiveDate>,
    pub tags: Vec<TagID>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub title: Option<String>,
    pub notes: Option<Option<String>>,
    pub start: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub start_precision: Option<StartPrecision>,
    pub deadline: Option<Option<chrono::NaiveDate>>,
    pub tags: Option<Vec<TagID>>,

    pub completed: Option<bool>,
    pub deleted: Option<bool>,

    pub position_key: Option<String>,
}
