#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use crate::{
    error::service::{Error, Result},
    tag::types::repo::{Filter, QueryOpts},
    types::pagination::SQLPagination,
    user::types::UserID,
};

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
    async fn create(&self, user_id: UserID, create_request: CreateRequest) -> Result<TagModel>;
    async fn get(&self, id: TagID, user_id: UserID) -> Result<TagModel>;
    async fn update(
        &self,
        id: TagID,
        user_id: UserID,
        update_request: UpdateRequest,
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
        if let Some(query) = query {
            let URLQueryOpts {
                page,
                limit,
                category,
            } = query;

            let mut filter = Filter::new();
            if let Some(category) = category {
                let category_id = self.repo.get_category_id(user_id, category).await?;

                filter.category(category_id);
            }

            let page = page.unwrap_or(1);
            let limit = limit.unwrap_or(25);
            let offset = limit * (page - 1);

            let mut pagination = SQLPagination::new();
            pagination.limit(limit);
            pagination.offset(offset);

            let query_opts = QueryOpts { filter, pagination };

            self.repo
                .list(user_id, Some(query_opts))
                .await
                .map_err(Error::from)
        } else {
            self.repo.list(user_id, None).await.map_err(Error::from)
        }
    }

    async fn create(&self, user_id: UserID, create_request: CreateRequest) -> Result<TagModel> {
        todo!()
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<TagModel> {
        todo!()
    }

    async fn update(
        &self,
        id: TagID,
        user_id: UserID,
        update_request: UpdateRequest,
    ) -> Result<TagModel> {
        todo!()
    }

    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        todo!()
    }
}
