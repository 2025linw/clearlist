use uuid::Uuid;

use crate::{db::user::UserRepository, models::user::Model};
use super::Result;

// WARN: REMOVE THIS
#[allow(dead_code)]
pub trait UserServiceTrait {
    async fn create(self, user: Model) -> Result<Model>;
    async fn get(self, id: Uuid) -> Result<Option<Model>>;
    async fn update(self, id: Uuid, user: Model) -> Result<Option<Model>>;
}

#[derive(Debug, Clone)]
pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: UserRepository> UserServiceTrait for UserService<R> {
    async fn create(self, user: Model) -> Result<Model> {
        // Business logic

        Ok(self.repo.create(user).await?)
    }

    async fn get(self, id: Uuid) -> Result<Option<Model>> {
        // Business logic

        Ok(self.repo.get(id).await?)
    }

    async fn update(self, id: Uuid, user: Model) -> Result<Option<Model>> {
        // Business logic

        Ok(self.repo.update(id, user).await?)
    }
}
