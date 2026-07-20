#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use crate::{error::service::Result, user::types::UserID};

use super::{
    repo::TagRepository,
    types::{
        TagID, TagModel,
        route::{CreateRequest, URLQueryOpts, UpdateRequest},
    },
};

#[async_trait]
pub trait TagServiceTrait {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<TagModel>>;
    async fn create(&self, user_id: UserID, create_tag: CreateRequest) -> Result<TagModel>;
    async fn get(&self, id: TagID, user_id: UserID) -> Result<TagModel>;
    async fn update(
        &self,
        id: TagID,
        user_id: UserID,
        update_tag: UpdateRequest,
    ) -> Result<TagModel>;
    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()>;
}

#[derive(Clone)]
pub struct TagService<R: TagRepository> {
    repo: R,
}

impl<R: TagRepository> TagService<R> {
    pub fn init(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: TagRepository> TagServiceTrait for TagService<R> {
    async fn list(&self, user_id: UserID, query: Option<URLQueryOpts>) -> Result<Vec<TagModel>> {
        todo!()
    }

    async fn create(&self, user_id: UserID, create_tag: CreateRequest) -> Result<TagModel> {
        todo!()
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<TagModel> {
        todo!()
    }

    async fn update(
        &self,
        id: TagID,
        user_id: UserID,
        update_tag: UpdateRequest,
    ) -> Result<TagModel> {
        todo!()
    }

    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        todo!()
    }
}
