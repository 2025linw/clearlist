#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use super::{
    repo::UserRepository,
    types::{Model, UserID},
};
use crate::error::service::Result;

#[async_trait]
pub trait UserServiceTrait {
    async fn create(&self, create_user: CreateModel) -> Result<Model>;
    async fn get(&self, id: UserID) -> Result<Option<Model>>;
    async fn update(&self, id: UserID, update_user: UpdateModel) -> Result<Model>;
}

struct UserService<R: UserRepository> {
    repo: R,
}

#[async_trait]
impl<R: UserRepository> UserServiceTrait for UserService<R> {
    async fn create(&self, user: CreateModel) -> Result<Model> {
        todo!()
    }

    async fn get(&self, id: UserID) -> Result<Option<Model>> {
        todo!()
    }

    async fn update(&self, id: UserID, user: UpdateModel) -> Result<Model> {
        todo!()
    }
}

#[derive(Debug)]
pub struct CreateModel {
    pub id: UserID,

    pub display_name: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub display_name: Option<String>,
}
