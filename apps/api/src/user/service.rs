#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use async_trait::async_trait;
use sqlx::postgres::types::PgInterval;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error as RepoError},
        service::{Error, NO_WHITESPACE_REASON, Result, ValidationError},
    },
    user::types::repo::{CreateModel, UpdateModel},
};

use super::{
    repo::UserRepository,
    types::{
        UserID, UserModel,
        route::{CreateRequest, UpdateRequest},
    },
};

#[async_trait]
pub trait UserServiceTrait {
    async fn create(&self, create_request: CreateRequest) -> Result<UserModel>;
    async fn get(&self, id: UserID) -> Result<UserModel>;
    async fn update(&self, id: UserID, update_request: UpdateRequest) -> Result<UserModel>;
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
    async fn create(&self, create_request: CreateRequest) -> Result<UserModel> {
        let CreateRequest {
            id,
            display_name,
            preferred_timezone,
            completed_task_retention,
            created_at,
        } = create_request;

        // Normalization
        let display_name = display_name.trim().to_string();

        // Validation
        if display_name.chars().any(|c| c.is_whitespace() && c != ' ') {
            return Err(Error::Validation(ValidationError::InvalidValue {
                field: "display_name",
                reason: NO_WHITESPACE_REASON,
            }));
        }

        let create_model = CreateModel {
            id,
            display_name,
            preferred_timezone: preferred_timezone.map(|tz| tz.to_string()),
            completed_task_retention: completed_task_retention.map(|int| int.into()),
            created_at,
        };

        self.repo.create(create_model).await.map_err(Error::from)
    }

    async fn get(&self, id: UserID) -> Result<UserModel> {
        // Validation

        self.repo
            .get(id)
            .await
            .map_err(Error::from)?
            .ok_or(Error::NotFound(Resource::User))
    }

    async fn update(&self, id: UserID, update_request: UpdateRequest) -> Result<UserModel> {
        if update_request.is_noop() {
            return Err(Error::Validation(ValidationError::NoChanges));
        }

        let UpdateRequest {
            display_name,
            preferred_timezone,
            completed_task_retention,
        } = update_request;

        // Normalization
        let display_name = display_name.map(|name| name.trim().to_string());

        // Validation
        if let Some(ref name) = display_name
            && name.chars().any(|c| c.is_whitespace() && c != ' ')
        {
            return Err(Error::Validation(ValidationError::InvalidValue {
                field: "display_name",
                reason: NO_WHITESPACE_REASON,
            }));
        }

        let update_model = UpdateModel {
            display_name,
            preferred_timezone: preferred_timezone.map(|inner| inner.map(|tz| tz.to_string())),
            completed_task_retention: completed_task_retention
                .map(|inner| inner.map(PgInterval::from)),
        };
        self.repo.update(id, update_model).await.map_err(|err| {
            if let RepoError::Constraint(ConstraintViolation::NotFound(Resource::User)) = err {
                return Error::NotFound(Resource::User);
            }

            err.into()
        })
    }
}
