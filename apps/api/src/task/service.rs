#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use crate::{
    error::service::Result,
    tag::types::{TagID, TagModel},
    user::types::UserID,
};

use super::{
    repo::TaskRepository,
    types::{
        TaskID, TaskModel,
        route::{CreateRequest, URLQueryOpts, UpdateRequest},
        service::UserContext,
    },
};

#[async_trait]
pub trait TaskServiceTrait {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<TaskModel>>;
    async fn create(
        &self,
        user_context: UserContext,
        create_task: CreateRequest,
    ) -> Result<TaskModel>;
    async fn get(&self, id: TaskID, user_id: UserID) -> Result<TaskModel>;
    async fn update(
        &self,
        id: TaskID,
        user_context: UserContext,
        update_task: UpdateRequest,
    ) -> Result<TaskModel>;

    async fn delete(&self, id: TaskID, user_id: UserID) -> Result<()>;
    async fn restore(&self, id: TaskID, user_id: UserID) -> Result<()>;

    async fn complete(&self, id: TaskID, user_id: UserID) -> Result<()>;
    async fn reopen(&self, id: TaskID, user_id: UserID) -> Result<()>;

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

impl<R: TaskRepository> TaskService<R> {
    pub fn init(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: TaskRepository> TaskServiceTrait for TaskService<R> {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<TaskModel>> {
        todo!()
    }

    async fn create(
        &self,
        user_context: UserContext,
        create_task: CreateRequest,
    ) -> Result<TaskModel> {
        todo!()
    }

    async fn get(&self, id: TaskID, user_id: UserID) -> Result<TaskModel> {
        todo!()
    }

    async fn update(
        &self,
        id: TaskID,
        user_context: UserContext,
        update_task: UpdateRequest,
    ) -> Result<TaskModel> {
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

    async fn reopen(&self, id: TaskID, user_id: UserID) -> Result<()> {
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
