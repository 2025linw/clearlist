#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use crate::{
    error::service::Result,
    tag::types::{Model as TagModel, TagID},
    user::types::UserID,
};

use super::{
    repo::TaskRepository,
    types::{
        Model, TaskID,
        route::{CreateRequest, URLQueryOpts, UpdateRequest},
    },
};

#[async_trait]
pub trait TaskServiceTrait {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>>;
    async fn create(&self, user_id: UserID, create_task: CreateRequest) -> Result<Model>;
    async fn get(&self, id: TaskID, user_id: UserID) -> Result<Option<Model>>;
    async fn update(
        &self,
        id: TaskID,
        user_id: UserID,
        update_task: UpdateRequest,
    ) -> Result<Model>;

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

#[derive(Clone)]
pub struct TaskService<R: TaskRepository> {
    repo: R,
}

#[async_trait]
impl<R: TaskRepository> TaskServiceTrait for TaskService<R> {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>> {
        todo!()
    }

    async fn create(&self, user_id: UserID, create_task: CreateRequest) -> Result<Model> {
        todo!()
    }

    async fn get(&self, id: TaskID, user_id: UserID) -> Result<Option<Model>> {
        todo!()
    }

    async fn update(
        &self,
        id: TaskID,
        user_id: UserID,
        update_task: UpdateRequest,
    ) -> Result<Model> {
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
