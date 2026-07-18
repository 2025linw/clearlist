#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use crate::{error::service::Result, user::types::UserID};

use super::{
    repo::TagRepository,
    types::{
        Model, TagID,
        route::{CreateRequest, URLQueryOpts, UpdateRequest},
    },
};

#[async_trait]
pub trait TagServiceTrait {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>>;
    async fn create(&self, user_id: UserID, create_tag: CreateRequest) -> Result<Model>;
    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<Model>>;
    async fn update(&self, id: TagID, user_id: UserID, update_tag: UpdateRequest) -> Result<Model>;
    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()>;
}

#[derive(Clone)]
pub struct TagService<R: TagRepository> {
    repo: R,
}

#[async_trait]
impl<R: TagRepository> TagServiceTrait for TagService<R> {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<Model>> {
        todo!()
    }

    async fn create(&self, user_id: UserID, create_tag: CreateRequest) -> Result<Model> {
        todo!()
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<Model>> {
        todo!()
    }

    async fn update(&self, id: TagID, user_id: UserID, update_tag: UpdateRequest) -> Result<Model> {
        todo!()
    }

    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        todo!()
    }
}
