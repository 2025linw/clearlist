use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    tests::mocks::MockDB,
    user::{
        repo::UserRepository,
        types::{
            UserID, UserModel,
            repo::{CreateModel, UpdateModel},
        },
    },
};

#[derive(Clone)]
pub struct MockUserRepository {
    repo: MockDB<UserID, UserModel>,
    error: Option<Error>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(RwLock::new(HashMap::new())),
            error: None,
        }
    }

    pub fn new_backend_error() -> Self {
        let mut mock = Self::new();
        mock.error = Some(Error::Backend("mock backend error".to_string()));

        mock
    }

    pub fn new_programming_error() -> Self {
        let mut mock = Self::new();
        mock.error = Some(Error::Programming("mock programming error".to_string()));

        mock
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, create_model: CreateModel) -> Result<UserModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.repo.write().await;

        let user = UserModel {
            id: create_model.id,
            display_name: create_model.display_name,
            preferred_timezone: create_model.preferred_timezone,
            completed_task_retention: create_model.completed_task_retention,
            updated_at: create_model.created_at,
            created_at: create_model.created_at,
        };
        repo.insert(create_model.id, user);

        Ok(repo.get(&create_model.id).unwrap().clone())
    }

    async fn get(&self, id: UserID) -> Result<Option<UserModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }

        Ok(self.repo.read().await.get(&id).cloned())
    }

    async fn update(&self, id: UserID, update_model: UpdateModel) -> Result<UserModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.repo.write().await;

        let user_opt = repo.get(&id);
        if user_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::User,
            )));
        }

        let mut user = user_opt.unwrap().clone();
        if let Some(display_name) = update_model.display_name {
            user.display_name = display_name;
        }
        if let Some(completed_task_retention) = update_model.completed_task_retention {
            user.completed_task_retention = completed_task_retention;
        }

        repo.insert(id, user.clone());

        Ok(user)
    }
}
