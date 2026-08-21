mod helpers;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error as RepoError},
        service::{Error, Result},
    },
    types::extract::UserContext,
};

use super::{
    repo::CategoryRepository,
    types::{
        Category, CategoryID,
        repo::{CreateModel, UpdateModel},
        route::{CreateRequest, UpdateRequest},
    },
};

#[derive(Clone)]
pub struct CategoryService<R: CategoryRepository> {
    pub(super) repo: R,
}

impl<R: CategoryRepository> CategoryService<R> {
    pub fn init(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list(&self, user_context: UserContext) -> Result<Vec<Category>> {
        self.repo
            .list(user_context.id)
            .await
            .map(|categories| categories.into_iter().map(Category::from).collect())
            .map_err(Error::from)
    }

    pub async fn create(
        &self,
        user_context: UserContext,
        create_request: CreateRequest,
    ) -> Result<Category> {
        let CreateRequest { name, position_key } =
            helpers::validate_create_request(create_request)?;

        let create_model = CreateModel { name, position_key };
        self.repo
            .create(user_context.id, create_model)
            .await
            .map(Category::from)
            .map_err(Error::from)
    }

    pub async fn get(&self, id: CategoryID, user_context: UserContext) -> Result<Category> {
        self.repo
            .get(id, user_context.id)
            .await
            .map_err(Error::from)?
            .map(Category::from)
            .ok_or(Error::NotFound(Resource::Category))
    }

    pub async fn update(
        &self,
        id: CategoryID,
        user_context: UserContext,
        update_request: UpdateRequest,
    ) -> Result<Category> {
        if update_request.is_noop() {
            return self.get(id, user_context).await;
        }

        let UpdateRequest { name, position_key } =
            helpers::validate_update_request(update_request)?;

        let update_model = UpdateModel { name, position_key };
        self.repo
            .update(id, user_context.id, update_model)
            .await
            .map_err(|err| {
                if let RepoError::Constraint(ConstraintViolation::NotFound(Resource::Category)) =
                    err
                {
                    return Error::NotFound(Resource::Category);
                }

                err.into()
            })
            .map(Category::from)
    }

    pub async fn delete(&self, id: CategoryID, user_context: UserContext) -> Result<()> {
        let res = self.repo.delete(id, user_context.id).await;
        if let Err(err) = res {
            if matches!(
                err,
                RepoError::Constraint(ConstraintViolation::NotFound(Resource::Category))
            ) {
                return Ok(());
            }

            return Err(err.into());
        }

        Ok(())
    }
}
