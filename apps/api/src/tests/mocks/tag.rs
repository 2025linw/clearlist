use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    sync::Arc,
};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    tag::{
        repo::TagRepository,
        types::{
            CategoryID, TagID, TagModel,
            repo::{CreateModel, QueryOpts, UpdateModel},
        },
    },
    tests::{
        helpers::{generate_a_z, get_today_date_pg},
        mocks::MockDB,
    },
    user::types::UserID,
};

#[derive(Clone)]
pub struct MockTagRepository {
    users: Arc<RwLock<HashSet<UserID>>>,
    categories: MockDB<(CategoryID, UserID), (String, String)>,
    tags: MockDB<(TagID, UserID), TagModel>,

    error: Option<Error>,
    last_filter: Arc<RwLock<Option<QueryOpts>>>,
}

impl MockTagRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashSet::new())),
            categories: Arc::new(RwLock::new(HashMap::new())),
            tags: Arc::new(RwLock::new(HashMap::new())),
            error: None,
            last_filter: Arc::new(RwLock::new(None)),
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

    pub async fn get_last_filter(&self) -> QueryOpts {
        let mut last_filter = self.last_filter.write().await;
        let res = last_filter.clone();

        *last_filter = None;

        res.unwrap()
    }

    pub async fn add_user(&self) -> UserID {
        let user_id = UserID::new_v4();

        let mut user_repo = self.users.write().await;
        assert!(user_repo.insert(user_id));

        user_id
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
        let cat_repo = self.categories.read().await;
        let user_repo = self.users.read().await;

        if !user_repo.contains(&user_id) {
            return Err(Error::Constraint(ConstraintViolation::MissingUser));
        }

        let (category_id, category_name) = if let Some(category_id) = create_model.category_id {
            if let Some((category_name, _)) = cat_repo.get(&(category_id, user_id)).cloned() {
                (Some(category_id), Some(category_name))
            } else {
                return Err(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Category,
                )));
            }
        } else {
            (None, None)
        };
        let created_at = get_today_date_pg();
        let tag = TagModel {
            id: TagID::new_v4(),
            label: create_model.label,
            category_id,
            category_name,
            position_key: create_model.position_key,
            cat_position_key: Some(generate_a_z(0).to_string()),
            updated_at: created_at,
            created_at,
            created_by: user_id,
        };
        repo.insert((tag.id, user_id), tag.clone());

        Ok(repo.get(&(tag.id, user_id)).unwrap().clone())
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<TagModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let repo = self.tags.read().await;

        let tag_opt = repo.get(&(id, user_id));
        if let Some(tag) = tag_opt {
            Ok(Some(tag.clone()))
        } else {
            Ok(None)
        }
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
        let cat_repo = self.categories.read().await;
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
                let category_name =
                    if let Some((category_name, _)) = cat_repo.get(&(id, user_id)).cloned() {
                        Some(category_name)
                    } else {
                        return Err(Error::Constraint(ConstraintViolation::NotFound(
                            Resource::Category,
                        )));
                    };

                tag.category_id = category_id;
                tag.category_name = category_name;
            } else {
                tag.category_id = None;
                tag.category_name = None;
            }
            changed = true;
        }
        if let Some(position_key) = update_model.position_key
            && position_key != tag.position_key
        {
            tag.position_key = position_key;
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

    async fn get_category_id(&self, user_id: UserID, name: String) -> Result<Option<CategoryID>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let cat_repo = self.categories.read().await;

        if let Some(((id, _), _)) = cat_repo
            .iter()
            .find(|((_, created_by), (cat_name, _))| &user_id == created_by && name == *cat_name)
        {
            Ok(Some(*id))
        } else {
            Ok(None)
        }
    }

    async fn add_category(
        &self,
        user_id: UserID,
        category: String,
        position_key: String,
    ) -> Result<CategoryID> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut cat_repo = self.categories.write().await;
        let user_repo = self.users.read().await;

        if !user_repo.contains(&user_id) {
            return Err(Error::Constraint(ConstraintViolation::MissingUser));
        }

        let id = CategoryID::new_v4();
        cat_repo.insert((id, user_id), (category, position_key));

        Ok(id)
    }

    async fn reposition_category(
        &self,
        id: CategoryID,
        user_id: UserID,
        position_key: String,
    ) -> Result<CategoryID> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut cat_repo = self.categories.write().await;

        match cat_repo.entry((id, user_id)) {
            Entry::Occupied(mut occupied_entry) => {
                let (category_name, _) = occupied_entry.get();
                occupied_entry.insert((category_name.to_owned(), position_key));

                Ok(id)
            }
            Entry::Vacant(_) => Ok(id),
        }
    }

    async fn remove_category(&self, id: CategoryID, user_id: UserID) -> Result<()> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut cat_repo = self.categories.write().await;

        if cat_repo.remove(&(id, user_id)).is_none() {
            Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Category,
            )))
        } else {
            Ok(())
        }
    }
}
