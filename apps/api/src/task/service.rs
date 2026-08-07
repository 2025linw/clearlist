mod helpers;

#[cfg(test)]
mod tests;

use async_trait::async_trait;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error as RepoError},
        service::{Error, Result},
    },
    tag::types::{Tag, TagID},
    task::{
        service::helpers::validate_set_tags,
        types::repo::{CreateModel, UpdateModel},
    },
    types::{date::Start, order::SortOrder, pagination::SQLPagination},
};

use super::{
    repo::TaskRepository,
    service::helpers::{validate_create_request, validate_query_opts, validate_update_request},
    types::{
        SortBy, Task, TaskID,
        repo::{Filter, QueryOpts, Sort},
        route::{CreateRequest, URLQueryOpts, UpdateRequest},
        service::UserContext,
    },
};

#[async_trait]
pub trait TaskServiceTrait {
    async fn list(
        &self,
        user_context: UserContext,
        query: Option<URLQueryOpts>,
    ) -> Result<Vec<Task>>;
    async fn create(
        &self,
        user_context: UserContext,
        create_request: CreateRequest,
    ) -> Result<Task>;
    async fn get(&self, id: TaskID, user_context: UserContext) -> Result<Task>;
    async fn update(
        &self,
        id: TaskID,
        user_context: UserContext,
        update_request: UpdateRequest,
    ) -> Result<Task>;

    async fn delete(&self, id: TaskID, user_context: UserContext) -> Result<()>;
    async fn restore(&self, id: TaskID, user_context: UserContext) -> Result<()>;

    async fn complete(&self, id: TaskID, user_context: UserContext) -> Result<()>;
    async fn reopen(&self, id: TaskID, user_context: UserContext) -> Result<()>;

    async fn list_tags(&self, id: TaskID, user_context: UserContext) -> Result<Vec<Tag>>;
    async fn add_tag(&self, id: TaskID, user_context: UserContext, tag_id: TagID) -> Result<()>;
    async fn remove_tag(&self, id: TaskID, user_context: UserContext, tag_id: TagID) -> Result<()>;
    async fn set_tags(
        &self,
        id: TaskID,
        user_context: UserContext,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<Tag>>;
}

#[derive(Clone)]
pub struct TaskService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn init(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: TaskRepository> TaskServiceTrait for TaskService<R> {
    async fn list(
        &self,
        user_context: UserContext,
        query: Option<URLQueryOpts>,
    ) -> Result<Vec<Task>> {
        let query = if let Some(query) = query {
            let URLQueryOpts {
                page,
                limit,
                sort_by,
                sort_order,
                start,
                deadline,
                completed,
                deleted,
                tags,
            } = validate_query_opts(query)?;

            let mut filter = Filter::new();
            if let Some(start) = start {
                // TODO: normalize start date to user context
                filter.start(start.try_into()?);
            }
            if let Some(deadline) = deadline {
                filter.deadline(deadline.try_into()?);
            }
            if let Some(completed) = completed {
                filter.completed(completed);
            }
            if let Some(deleted) = deleted {
                filter.deleted(deleted);
            }
            if let Some(tags) = tags {
                filter.tags(tags);
            }

            let sort = Sort::new(sort_by, sort_order.unwrap_or(SortOrder::Descending));

            let page = page.unwrap_or(1);
            let limit = limit.unwrap_or(25).min(150);
            let offset = limit * (page - 1);
            let mut pagination = SQLPagination::new();
            pagination.limit(limit);
            pagination.offset(offset);

            QueryOpts {
                filter,
                sort,
                pagination,
            }
        } else {
            let mut filter = Filter::new();
            filter.completed(false);
            filter.deleted(false);

            let sort = Sort::new(Some(SortBy::Updated), SortOrder::Descending);

            let mut pagination = SQLPagination::new();
            pagination.limit(25);
            pagination.offset(0);

            QueryOpts {
                filter,
                sort,
                pagination,
            }
        };

        let tasks = self
            .repo
            .list(user_context.id, Some(query))
            .await
            .map_err(Error::from)?;
        let mut task_tags = self
            .repo
            .list_task_tags(tasks.iter().map(|task| task.id).collect(), user_context.id)
            .await?;

        Ok(tasks
            .into_iter()
            .map(|task| {
                let id = task.id;

                Task::from(
                    task,
                    user_context.tz,
                    task_tags.remove(&id).unwrap_or_default(),
                )
            })
            .collect())
    }

    async fn create(
        &self,
        user_context: UserContext,
        create_request: CreateRequest,
    ) -> Result<Task> {
        let CreateRequest {
            title,
            notes,
            start,
            deadline,
            tags,
            position_key,
        } = validate_create_request(create_request)?;

        // Convert and normalized start
        let mut has_time = false;
        let start = start.map(|start| match start {
            Start::Date(naive_date) => {
                has_time = false;

                naive_date
                    .and_hms_opt(0, 0, 0)
                    .expect("midnight should be valid")
                    .and_local_timezone(user_context.tz)
                    .single()
                    .expect("midnight should be unambiguous")
                    .to_utc()
            }
            Start::DateTime(date_time) => {
                has_time = true;

                date_time
            }
        });

        let create_model = CreateModel {
            title,
            notes,
            start,
            has_time,
            deadline,
            position_key,
        };
        let task_model = self
            .repo
            .create(user_context.id, create_model)
            .await
            .map_err(Error::from)?;
        let tags = self
            .repo
            .set_tags(task_model.id, user_context.id, tags)
            .await?;

        Ok(Task::from(task_model, user_context.tz, tags))
    }

    async fn get(&self, id: TaskID, user_context: UserContext) -> Result<Task> {
        let tags = self.repo.list_tags(id, user_context.id).await?;
        let task = self.repo.get(id, user_context.id).await?;
        if !task.exists() {
            return Err(Error::NotFound(Resource::Task));
        }

        let task = Task::from(
            task.expect("task already checked for existence"),
            user_context.tz,
            tags,
        );
        Ok(task)
    }

    async fn update(
        &self,
        id: TaskID,
        user_context: UserContext,
        update_request: UpdateRequest,
    ) -> Result<Task> {
        let UpdateRequest {
            title,
            notes,
            start,
            deadline,
            tags,
            position_key,
        } = validate_update_request(update_request)?;

        // Convert and normalize start date
        let mut has_time = None;
        let start = if let Some(start) = start {
            start.map(|start| match start {
                Start::Date(naive_date) => {
                    has_time = Some(false);

                    Some(
                        naive_date
                            .and_hms_opt(0, 0, 0)
                            .expect("midnight should be valid")
                            .and_local_timezone(user_context.tz)
                            .single()
                            .expect("midnight should be unambiguous")
                            .to_utc(),
                    )
                }
                Start::DateTime(date_time) => {
                    has_time = Some(true);

                    Some(date_time)
                }
            })
        } else {
            None
        };

        let update_model = UpdateModel {
            title,
            notes,
            start,
            has_time,
            deadline,
            completed: None,
            deleted: None,
            position_key,
        };
        let task_model = self
            .repo
            .update(id, user_context.id, update_model)
            .await
            .map_err(Error::from)?;

        let tags = if let Some(tags) = tags {
            self.repo.set_tags(id, user_context.id, tags).await?
        } else {
            Vec::new()
        };
        Ok(Task::from(task_model, user_context.tz, tags))
    }

    async fn delete(&self, id: TaskID, user_context: UserContext) -> Result<()> {
        if let Err(err) = self
            .repo
            .update(
                id,
                user_context.id,
                UpdateModel {
                    deleted: Some(true),
                    ..Default::default()
                },
            )
            .await
        {
            return if let RepoError::Constraint(ConstraintViolation::NotFound(Resource::Task))
            | RepoError::Constraint(ConstraintViolation::Deleted(Resource::Task)) = err
            {
                Ok(())
            } else {
                Err(err.into())
            };
        }

        Ok(())
    }

    async fn restore(&self, id: TaskID, user_context: UserContext) -> Result<()> {
        self.repo
            .update(
                id,
                user_context.id,
                UpdateModel {
                    deleted: Some(false),
                    ..Default::default()
                },
            )
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    async fn complete(&self, id: TaskID, user_context: UserContext) -> Result<()> {
        self.repo
            .update(
                id,
                user_context.id,
                UpdateModel {
                    completed: Some(true),
                    ..Default::default()
                },
            )
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    async fn reopen(&self, id: TaskID, user_context: UserContext) -> Result<()> {
        self.repo
            .update(
                id,
                user_context.id,
                UpdateModel {
                    completed: Some(false),
                    ..Default::default()
                },
            )
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    async fn list_tags(&self, id: TaskID, user_context: UserContext) -> Result<Vec<Tag>> {
        Ok(self
            .repo
            .list_tags(id, user_context.id)
            .await
            .map_err(Error::from)?
            .into_iter()
            .map(Tag::from)
            .collect())
    }

    async fn add_tag(&self, id: TaskID, user_context: UserContext, tag_id: TagID) -> Result<()> {
        self.repo
            .add_tag(id, user_context.id, tag_id)
            .await
            .map_err(Error::from)
    }

    async fn remove_tag(&self, id: TaskID, user_context: UserContext, tag_id: TagID) -> Result<()> {
        self.repo
            .remove_tag(id, user_context.id, tag_id)
            .await
            .map_err(Error::from)
    }

    async fn set_tags(
        &self,
        id: TaskID,
        user_context: UserContext,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<Tag>> {
        let tag_ids = validate_set_tags(tag_ids)?;

        Ok(self
            .repo
            .set_tags(id, user_context.id, tag_ids)
            .await
            .map_err(Error::from)?
            .into_iter()
            .map(Tag::from)
            .collect())
    }
}
