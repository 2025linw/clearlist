#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;

use crate::error::service::Result;

use super::{
    repo::UserRepository,
    types::{
        Model, UserID,
        route::{CreateRequest, UpdateRequest},
    },
};

#[async_trait]
pub trait UserServiceTrait {
    async fn create(&self, create_user: CreateRequest) -> Result<Model>;
    async fn get(&self, id: UserID) -> Result<Model>;
    async fn update(&self, id: UserID, update_user: UpdateRequest) -> Result<Model>;
}

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn init(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: UserRepository> UserServiceTrait for UserService<R> {
    async fn create(&self, create_user: CreateRequest) -> Result<Model> {
        todo!()
    }

    async fn get(&self, id: UserID) -> Result<Model> {
        todo!()
    }

    async fn update(&self, id: UserID, update_user: UpdateRequest) -> Result<Model> {
        todo!()
    }
}
