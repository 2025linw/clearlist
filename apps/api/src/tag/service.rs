mod helpers;

use crate::{
    category::repo::CategoryRepository,
    error::{
        Resource,
        repo::{ConstraintViolation, Error as RepoError},
        service::{Error, Result},
    },
    types::{extract::UserContext, pagination::SQLPagination},
};

use super::{
    repo::TagRepository,
    types::{
        Tag, TagID,
        repo::{CreateModel, Filter, QueryOpts, UpdateModel},
        route::{CreateRequest, URLQueryOpts, UpdateRequest},
    },
};

#[derive(Clone)]
pub struct TagService<R: TagRepository, C: CategoryRepository> {
    pub(super) repo: R,
    pub(super) category_repo: C,
}

impl<R: TagRepository, C: CategoryRepository> TagService<R, C> {
    pub fn init(repo: R, category_repo: C) -> Self {
        Self {
            repo,
            category_repo,
        }
    }

    pub async fn list(
        &self,
        user_context: UserContext,
        query: Option<URLQueryOpts>,
    ) -> Result<Vec<Tag>> {
        let query = if let Some(query) = query {
            let URLQueryOpts {
                page,
                limit,
                category,
            } = helpers::validate_query_opts(query)?;

            let mut filter = Filter::new();
            if let Some(category) = category {
                match self
                    .category_repo
                    .get_id_by_name(user_context.id, category)
                    .await
                {
                    Ok(id) => filter.category(id),
                    Err(err) => {
                        if matches!(
                            err,
                            RepoError::Constraint(ConstraintViolation::NotFound(
                                Resource::Category
                            ))
                        ) {
                            return Ok(Vec::new());
                        }

                        return Err(err.into());
                    }
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

    pub async fn create(
        &self,
        user_context: UserContext,
        create_request: CreateRequest,
    ) -> Result<Tag> {
        let CreateRequest {
            label,
            category,
            position_key,
        } = helpers::validate_create_request(create_request)?;

        // Get id for category name, if exists
        let category_id = if let Some(category_name) = category {
            match self
                .category_repo
                .get_id_by_name(user_context.id, category_name.clone())
                .await
            {
                Ok(id) => Some(id),
                Err(err) => {
                    if matches!(
                        err,
                        RepoError::Constraint(ConstraintViolation::NotFound(Resource::Category))
                    ) {
                        return Err(Error::NotFound(Resource::Category));
                    }

                    return Err(err.into());
                }
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

    pub async fn get(&self, id: TagID, user_context: UserContext) -> Result<Tag> {
        self.repo
            .get(id, user_context.id)
            .await
            .map_err(Error::from)?
            .map(Tag::from)
            .ok_or(Error::NotFound(Resource::Tag))
    }

    pub async fn update(
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
        } = helpers::validate_update_request(update_request)?;

        // Get id for category name, if exists
        let category_id = if let Some(category_opt) = category {
            Some(if let Some(category) = category_opt {
                match self
                    .category_repo
                    .get_id_by_name(user_context.id, category.clone())
                    .await
                {
                    Ok(id) => Some(id),
                    Err(err) => {
                        if matches!(
                            err,
                            RepoError::Constraint(ConstraintViolation::NotFound(
                                Resource::Category
                            ))
                        ) {
                            return Err(Error::NotFound(Resource::Category));
                        }

                        return Err(err.into());
                    }
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

    pub async fn delete(&self, id: TagID, user_context: UserContext) -> Result<()> {
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
