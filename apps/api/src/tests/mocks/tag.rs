use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    category::{
        repo::CategoryRepository,
        types::{CategoryModel, repo::CreateModel as CategoryCreateModel},
    },
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    tag::{
        repo::TagRepository,
        types::{
            TagID, TagModel,
            repo::{CreateModel, QueryOpts, UpdateModel},
        },
    },
    tests::{
        helpers::get_today_date_pg,
        mocks::{MockDB, MockUserRepository, category::MockCategoryRepository},
    },
    user::{
        repo::UserRepository,
        types::{UserID, repo::CreateModel as UserCreateModel},
    },
};

#[derive(Clone)]
pub struct MockTagRepository {
    tags: MockDB<(TagID, UserID), TagModel>,
    categories: MockCategoryRepository,
    users: MockUserRepository,

    error: Option<Error>,
    last_filter: Arc<RwLock<Option<QueryOpts>>>,
}

impl MockTagRepository {
    pub fn new() -> Self {
        let user_repo = MockUserRepository::new();

        Self {
            tags: Arc::new(RwLock::new(HashMap::new())),
            categories: MockCategoryRepository::init(user_repo.clone()),
            users: user_repo,
            error: None,
            last_filter: Arc::new(RwLock::new(None)),
        }
    }

    pub fn init(user_repo: MockUserRepository, category_repo: MockCategoryRepository) -> Self {
        Self {
            tags: Arc::new(RwLock::new(HashMap::new())),
            categories: category_repo,
            users: user_repo,
            error: None,
            last_filter: Arc::new(RwLock::new(None)),
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

    pub fn categories(&self) -> MockCategoryRepository {
        self.categories.clone()
    }

    pub async fn get_last_filter(&self) -> QueryOpts {
        let mut last_filter = self.last_filter.write().await;
        let res = last_filter.clone();

        *last_filter = None;

        res.unwrap()
    }

    pub async fn add_user(&self) -> UserID {
        let user = self.users.create(UserCreateModel::default()).await.unwrap();

        user.id
    }

    pub async fn add_category(&self, user_id: UserID) -> CategoryModel {
        self.categories
            .create(
                user_id,
                CategoryCreateModel {
                    name: "Test Category".to_string(),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
    }
}

#[async_trait]
impl TagRepository for MockTagRepository {
    async fn list(&self, _user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<TagModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        if let Some(query) = query {
            let mut last_filter = self.last_filter.write().await;
            *last_filter = Some(query);
        }

        Ok(vec![])
    }

    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<TagModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.tags.write().await;

        if self.users.get(user_id).await.unwrap().is_none() {
            return Err(Error::Constraint(ConstraintViolation::ForeignKey {
                resource: Resource::User,
                message: "tags_created_by_fkey".to_string(),
            }));
        }

        let category = if let Some(category_id) = create_model.category_id {
            if let Some(category) = self.categories.get(category_id, user_id).await.unwrap() {
                Some(category)
            } else {
                return Err(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Category,
                )));
            }
        } else {
            None
        };
        let (category_id, category_name, category_position_key) = match category {
            Some(c) => (Some(c.id), Some(c.name), Some(c.position_key)),
            None => (None, None, None),
        };
        let created_at = get_today_date_pg();
        let tag = TagModel {
            id: TagID::new_random(),
            label: create_model.label,
            category_id,
            category_name,
            position_key: create_model.position_key,
            category_position_key,
            updated_at: created_at,
            created_at,
            created_by: user_id,
        };
        repo.insert((tag.id, user_id), tag.clone());

        Ok(repo.get(&(tag.id, user_id)).unwrap().to_owned())
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<TagModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let repo = self.tags.read().await;

        Ok(repo.get(&(id, user_id)).cloned())
    }

    async fn update(
        &self,
        id: TagID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TagModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.tags.write().await;

        let tag_opt = repo.get(&(id, user_id));
        if tag_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Tag,
            )));
        }
        let mut tag = tag_opt.unwrap().clone();

        let mut changed = false;
        let now = get_today_date_pg();
        if let Some(label) = update_model.label
            && label != tag.label
        {
            tag.label = label;
            changed = true;
        }
        if let Some(category_id) = update_model.category_id
            && category_id != tag.category_id
        {
            if let Some(id) = category_id {
                let category =
                    if let Some(category) = self.categories.get(id, user_id).await.unwrap() {
                        category
                    } else {
                        return Err(Error::Constraint(ConstraintViolation::NotFound(
                            Resource::Category,
                        )));
                    };

                tag.category_id = Some(category.id);
                tag.category_name = Some(category.name);
                tag.category_position_key = Some(category.position_key);
            } else {
                tag.category_id = None;
                tag.category_name = None;
                tag.category_position_key = None;
            }
            changed = true;
        }

        if let Some(position_key) = update_model.position_key
            && position_key != tag.position_key
        {
            tag.position_key = position_key;
            changed = true;
        }

        if !changed {
            return Ok(tag);
        }

        tag.updated_at = now;
        repo.insert((id, user_id), tag.clone());

        Ok(tag)
    }

    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.tags.write().await;

        let tag_opt = repo.get(&(id, user_id));
        if tag_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Tag,
            )));
        }

        repo.remove(&(id, user_id)).unwrap();

        Ok(())
    }
}
