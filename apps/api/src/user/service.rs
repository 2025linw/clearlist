mod helpers;

use sqlx::postgres::types::PgInterval;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error as RepoError},
        service::{Error, Result},
    },
    user::types::User,
};

use super::{
    repo::UserRepository,
    types::{
        UserID,
        repo::{CreateModel, UpdateModel},
        route::{ProvisionRequest, UpdateRequest},
    },
};

#[derive(Clone)]
pub struct UserService<R>
where
    R: UserRepository,
{
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn init(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(&self, provision_request: ProvisionRequest) -> Result<User> {
        let ProvisionRequest {
            id,
            display_name,
            created_at,
        } = helpers::validate_create_request(provision_request)?;

        let create_model = CreateModel {
            id,
            display_name,
            created_at,
        };
        self.repo
            .create(create_model)
            .await
            .map(User::from)
            .map_err(Error::from)
    }

    pub async fn get(&self, id: UserID) -> Result<User> {
        self.repo
            .get(id)
            .await
            .map_err(Error::from)?
            .map(User::from)
            .ok_or(Error::NotFound(Resource::User))
    }

    pub async fn update(&self, id: UserID, update_request: UpdateRequest) -> Result<User> {
        if update_request.is_noop() {
            return self.get(id).await;
        }

        let UpdateRequest {
            display_name,
            preferred_timezone,
            completed_task_retention,
        } = helpers::validate_update_request(update_request)?;

        let update_model = UpdateModel {
            display_name,
            preferred_timezone: preferred_timezone.map(|inner| inner.map(|tz| tz.to_string())),
            completed_task_retention: completed_task_retention
                .map(|inner| inner.map(PgInterval::from)),
        };

        self.repo
            .update(id, update_model)
            .await
            .map_err(|err| {
                if let RepoError::Constraint(ConstraintViolation::NotFound(Resource::User)) = err {
                    return Error::NotFound(Resource::User);
                }

                err.into()
            })
            .map(User::from)
    }
}
