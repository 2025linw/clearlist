use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    category::{
        repo::CategoryRepository,
        types::{
            CategoryID, CategoryModel,
            repo::{CreateModel, UpdateModel},
        },
    },
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    tests::mocks::{MockDB, MockUserRepository},
    user::{
        repo::UserRepository,
        types::{UserID, repo::CreateModel as UserCreateModel},
    },
};

#[derive(Clone)]
pub struct MockCategoryRepository {
    categories: MockDB<(CategoryID, UserID), CategoryModel>,
    users: MockUserRepository,

    error: Option<Error>,
}

impl MockCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: Arc::new(RwLock::new(HashMap::new())),
            users: MockUserRepository::new(),
            error: None,
        }
    }

    pub fn init(user_repo: MockUserRepository) -> Self {
        Self {
            categories: Arc::new(RwLock::new(HashMap::new())),
            users: user_repo,
            error: None,
        }
    }

    pub fn new_backend_error() -> Self {
        let mut mock = Self::new();
        mock.error = Some(Error::Backend("mock backend error".to_string()));

        mock
    }

    pub fn new_internal_error() -> Self {
        let mut mock = Self::new();
        mock.error = Some(Error::Internal("mock programming error".to_string()));

        mock
    }

    pub async fn add_user(&self) -> UserID {
        let user = self.users.create(UserCreateModel::default()).await.unwrap();

        user.id
    }
}

#[async_trait]
impl CategoryRepository for MockCategoryRepository {
    async fn get_id_by_name(&self, user_id: UserID, name: String) -> Result<CategoryID> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let repo = self.categories.read().await;

        repo.iter()
            .find(|((_, category_user_id), category)| {
                *category_user_id == user_id && category.name == name
            })
            .map(|((category_id, _), _)| *category_id)
            .ok_or(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Category,
            )))
    }

    async fn list(&self, _user_id: UserID) -> Result<Vec<CategoryModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }

        Ok(vec![])
    }

    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<CategoryModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }

        if self.users.get(user_id).await.unwrap().is_none() {
            return Err(Error::Constraint(ConstraintViolation::ForeignKey {
                resource: Resource::User,
                message: "tags_created_by_fkey".to_string(),
            }));
        }

        match self
            .get_id_by_name(user_id, create_model.name.clone())
            .await
        {
            Ok(_) => {
                return Err(Error::Constraint(ConstraintViolation::Unique {
                    resource: Resource::Category,
                    message: "categories_pkey".to_string(),
                }));
            }
            Err(err)
                if !matches!(
                    err,
                    Error::Constraint(ConstraintViolation::NotFound(Resource::Category))
                ) =>
            {
                return Err(err);
            }
            _ => (),
        }

        let category = CategoryModel {
            id: CategoryID::new_random(),
            name: create_model.name,
            position_key: create_model.position_key,
        };
        let mut repo = self.categories.write().await;
        repo.insert((category.id, user_id), category.clone());

        Ok(repo.get(&(category.id, user_id)).unwrap().to_owned())
    }

    async fn get(&self, id: CategoryID, user_id: UserID) -> Result<Option<CategoryModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let repo = self.categories.read().await;

        Ok(repo.get(&(id, user_id)).cloned())
    }

    async fn update(
        &self,
        id: CategoryID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<CategoryModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.categories.write().await;

        let category_opt = repo.get(&(id, user_id));
        if category_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Category,
            )));
        }
        let mut category = category_opt.unwrap().clone();

        if let Some(name) = update_model.name
            && name != category.name
        {
            category.name = name;
        }

        if let Some(position_key) = update_model.position_key
            && position_key != category.position_key
        {
            category.position_key = position_key;
        }

        repo.insert((id, user_id), category.clone());

        return Ok(category);
    }

    async fn delete(&self, id: CategoryID, user_id: UserID) -> Result<()> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.categories.write().await;

        let category_opt = repo.get(&(id, user_id));
        if category_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Category,
            )));
        }

        repo.remove(&(id, user_id)).unwrap();

        Ok(())
    }
}
