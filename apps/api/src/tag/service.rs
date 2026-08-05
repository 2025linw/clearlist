mod helpers;

#[cfg(test)]
mod tests;

use async_trait::async_trait;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error as RepoError},
        service::{Error, NO_EMPTY_STRING, NO_WHITESPACE, Result, ValidationError},
    },
    types::pagination::SQLPagination,
};

use super::{
    repo::TagRepository,
    service::helpers::{validate_create_request, validate_query_opts, validate_update_request},
    types::{
        Tag, TagID,
        repo::{CreateModel, Filter, QueryOpts, UpdateModel},
        route::{CreateRequest, URLQueryOpts, UpdateRequest},
        service::UserContext,
    },
};

#[async_trait]
pub trait TagServiceTrait {
    async fn list(
        &self,
        user_context: UserContext,
        query: Option<URLQueryOpts>,
    ) -> Result<Vec<Tag>>;
    async fn create(&self, user_context: UserContext, create_request: CreateRequest)
    -> Result<Tag>;
    async fn get(&self, id: TagID, user_context: UserContext) -> Result<Tag>;
    async fn update(
        &self,
        id: TagID,
        user_context: UserContext,
        update_request: UpdateRequest,
    ) -> Result<Tag>;
    async fn delete(&self, id: TagID, user_context: UserContext) -> Result<()>;
}

#[derive(Clone)]
pub struct TagService<R: TagRepository> {
    repo: R,
}

impl<R: TagRepository> TagService<R> {
    pub fn init(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: TagRepository> TagServiceTrait for TagService<R> {
    async fn list(
        &self,
        user_context: UserContext,
        query: Option<URLQueryOpts>,
    ) -> Result<Vec<Tag>> {
        let query = if let Some(query) = query {
            let URLQueryOpts {
                page,
                limit,
                category,
            } = validate_query_opts(query)?;

            let mut filter = Filter::new();
            if let Some(category) = category {
                if let Some(id) = self.repo.get_category_id(user_context.id, category).await? {
                    filter.category(id);
                } else {
                    return Ok(vec![]);
                }
            }

            let page = page.unwrap_or(1);
            let limit = limit.unwrap_or(25).min(150);
            let offset = limit * (page - 1);
            let mut pagination = SQLPagination::new();
            pagination.limit(limit);
            pagination.offset(offset);

            QueryOpts { filter, pagination }
        } else {
            let mut pagination = SQLPagination::new();
            pagination.limit(25);
            pagination.offset(0);

            QueryOpts {
                filter: Filter::new(),
                pagination,
            }
        };

        self.repo
            .list(user_context.id, Some(query))
            .await
            .map(|tags| tags.into_iter().map(Tag::from).collect())
            .map_err(Error::from)
    }

    async fn create(
        &self,
        user_context: UserContext,
        create_request: CreateRequest,
    ) -> Result<Tag> {
        let CreateRequest {
            label,
            category,
            position_key,
        } = validate_create_request(create_request)?;

        // Get id for category name, if exists
        let category_id = if let Some(category) = category {
            if let Some(id) = self
                .repo
                .get_category_id(user_context.id, category.clone())
                .await?
            {
                Some(id)
            } else {
                Some(
                    self.repo
                        .add_category(user_context.id, category, "a".to_string())
                        .await?,
                )
            }
        } else {
            None
        };

        let create_model = CreateModel {
            label,
            category_id,
            position_key,
        };
        self.repo
            .create(user_context.id, create_model)
            .await
            .map(Tag::from)
            .map_err(Error::from)
    }

    async fn get(&self, id: TagID, user_context: UserContext) -> Result<Tag> {
        self.repo
            .get(id, user_context.id)
            .await
            .map_err(Error::from)?
            .map(Tag::from)
            .ok_or(Error::NotFound(Resource::Tag))
    }

    async fn update(
        &self,
        id: TagID,
        user_context: UserContext,
        update_request: UpdateRequest,
    ) -> Result<Tag> {
        if update_request.is_noop() {
            return self.get(id, user_context).await;
        }

        let UpdateRequest {
            label,
            category,
            position_key,
        } = validate_update_request(update_request)?;

        // Get id for category name, if exists
        let category_id = if let Some(category_opt) = category {
            Some(if let Some(category) = category_opt {
                if category.is_empty() {
                    return Err(Error::Validation(ValidationError::InvalidValue {
                        field: "category",
                        reason: NO_EMPTY_STRING,
                    }));
                } else if category.chars().any(|c| c.is_whitespace() && c != ' ') {
                    return Err(Error::Validation(ValidationError::InvalidValue {
                        field: "category",
                        reason: NO_WHITESPACE,
                    }));
                }

                if let Some(id) = self
                    .repo
                    .get_category_id(user_context.id, category.clone())
                    .await?
                {
                    Some(id)
                } else {
                    Some(
                        self.repo
                            .add_category(user_context.id, category, "a".to_string())
                            .await?,
                    )
                }
            } else {
                None
            })
        } else {
            None
        };

        let update_model = UpdateModel {
            label,
            category_id,
            position_key,
        };
        self.repo
            .update(id, user_context.id, update_model)
            .await
            .map_err(|err| {
                if let RepoError::Constraint(ConstraintViolation::NotFound(Resource::Tag)) = err {
                    return Error::NotFound(Resource::Tag);
                }

                err.into()
            })
            .map(Tag::from)
    }

    async fn delete(&self, id: TagID, user_context: UserContext) -> Result<()> {
        let res = self.repo.delete(id, user_context.id).await;
        if let Err(err) = res {
            if matches!(
                err,
                RepoError::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ) {
                return Ok(());
            }

            return Err(err.into());
        }

        Ok(())
    }
}
