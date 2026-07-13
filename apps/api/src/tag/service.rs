#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use super::{
    repo::TagRepository,
    types::{Model, TagID, URLQueryOpts},
};
use crate::{error::service::Result, user::types::UserID};

#[async_trait]
pub trait TagServiceTrait {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>>;
    async fn create(&self, user_id: UserID, create_tag: CreateModel) -> Result<Model>;
    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<Model>>;
    async fn update(&self, id: TagID, user_id: UserID, update_tag: UpdateModel) -> Result<Model>;
    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()>;
}

struct TagService<R: TagRepository> {
    repo: R,
}

#[async_trait]
impl<R: TagRepository> TagServiceTrait for TagService<R> {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>> {
        todo!()
    }
    async fn create(&self, user_id: UserID, create_tag: CreateModel) -> Result<Model> {
        todo!()
    }
    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<Model>> {
        todo!()
    }
    async fn update(&self, id: TagID, user_id: UserID, update_tag: UpdateModel) -> Result<Model> {
        todo!()
    }
    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Default))]
pub struct CreateModel {
    pub label: String,
    pub category: Option<String>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub label: Option<String>,
    pub category: Option<Option<String>>,

    pub position_key: Option<String>,
}
