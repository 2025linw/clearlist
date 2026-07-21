#[cfg(test)]
mod tests;

use std::collections::HashSet;

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, QueryBuilder, query, query_scalar};
use uuid::Uuid;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    tag::types::{TagID, TagModel},
    types::date::StartPrecision,
    user::types::UserID,
    utils::repo::{query_as, set_updated_timestamp},
};

use super::types::{
    TaskID, TaskModel,
    repo::{CreateModel, QueryOpts, TaskState, UpdateModel},
};

#[async_trait]
pub trait TaskRepository: Send + Sync + Clone {
    async fn task_exists(&self, id: TaskID, user_id: UserID) -> Result<bool>;

    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<TaskModel>>;
    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<TaskModel>;
    async fn get(&self, id: TaskID, user_id: UserID) -> Result<TaskState<TaskModel>>;
    async fn update(
        &self,
        id: TaskID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TaskState<TaskModel>>;
    async fn delete(&self, id: TaskID, user_id: UserID) -> Result<()>;

    async fn list_tags(&self, id: TaskID, user_id: UserID) -> Result<Vec<TagModel>>;
    async fn add_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()>;
    async fn remove_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()>;
    async fn set_tags(
        &self,
        id: TaskID,
        user_id: UserID,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<TagModel>>;
}

#[derive(Clone)]
pub struct PgTaskRepository {
    db: PgPool,
}

impl PgTaskRepository {
    pub fn init(db: PgPool) -> Self {
        Self { db }
    }

    async fn fetch_all_task_rows(
        conn: &mut PgConnection,
        user_id: UserID,
        query: Option<QueryOpts>,
    ) -> Result<Vec<TaskModel>> {
        let mut builder = QueryBuilder::new(
            "SELECT t.* FROM app.tasks t
            LEFT JOIN app.task_tags tt ON t.id = tt.task_id
            WHERE created_by = ",
        );
        builder.push_bind(user_id);
        if let Some(opts) = query {
            opts.add_to_builder(&mut builder);
        } else {
            builder.push(" GROUP BY t.id");
        }

        let query = builder.build_query_as::<TaskModel>();

        Ok(query.fetch_all(conn.as_mut()).await?)
    }

    async fn create_model(
        conn: &mut PgConnection,
        user_id: UserID,
        create_model: CreateModel,
    ) -> Result<TaskModel> {
        let id = query_scalar(
            "INSERT INTO app.tasks (id, title, notes, start_dt, has_time, deadline, position_key, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id",
        )
        .bind(Uuid::new_v4())
        .bind(create_model.title)
        .bind(create_model.notes)
        .bind(create_model.start)
        .bind(matches!(
            create_model.start_precision,
            StartPrecision::DateTime
        ))
        .bind(create_model.deadline)
        .bind(create_model.position_key)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await.map_err(|err| {
            if let Some(pg_err) = err.as_database_error() && pg_err.is_foreign_key_violation() {
                return Error::Constraint(ConstraintViolation::MissingUser);
            }

            err.into()
        })?;

        Ok(Self::fetch_task_row(conn, id, user_id)
            .await?
            .expect("task was just created"))
    }

    async fn fetch_task_row(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
    ) -> Result<TaskState<TaskModel>> {
        let task_opt = query_as::<TaskModel>(
            "SELECT * FROM app.tasks
            WHERE id = $1 AND created_by = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(conn.as_mut())
        .await?;

        match task_opt {
            Some(task) => {
                if task.deleted_at.is_some() {
                    Ok(TaskState::Deleted(task))
                } else {
                    Ok(TaskState::Existing(task))
                }
            }
            None => Ok(TaskState::None),
        }
    }

    async fn update_task_row(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TaskModel> {
        let mut builder = QueryBuilder::new("UPDATE app.tasks SET ");
        update_model.add_to_builder(&mut builder);
        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" AND created_by = ");
        builder.push_bind(user_id);

        let res = builder.build().execute(conn.as_mut()).await?;
        if res.rows_affected() == 0 {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        if let Some(task) = Self::fetch_task_row(conn, id, user_id)
            .await
            .expect("task was just updated")
            .into_inner()
        {
            Ok(task)
        } else {
            unreachable!()
        }
    }

    async fn delete_task_row(conn: &mut PgConnection, id: TaskID, user_id: UserID) -> Result<()> {
        let res = query(
            "DELETE FROM app.tasks
            WHERE id = $1 AND created_by = $2",
        )
        .bind(id)
        .bind(user_id)
        .execute(conn.as_mut())
        .await?;

        if res.rows_affected() == 0 {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        Ok(())
    }

    async fn fetch_task_tags(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
    ) -> Result<Vec<TagModel>> {
        Ok(query_as::<TagModel>(
            "SELECT tg.*, tc.category_name
            FROM app.tags tg
            LEFT JOIN app.categories tc ON tg.category_id = tc.id
            JOIN app.task_tags tt ON tg.id = tt.tag_id
            JOIN app.tasks t ON tt.task_id = t.id
            WHERE tt.task_id = $1 AND t.created_by = $2
            ORDER BY tg.position_key ASC NULLS FIRST, tg.label ASC, tg.updated_at DESC, tg.id ASC",
        )
        .bind(id)
        .bind(user_id)
        .fetch_all(conn.as_mut())
        .await?)
    }

    async fn add_tag_to_task(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        tag_id: TagID,
    ) -> Result<()> {
        let res = query(
            "INSERT INTO app.task_tags (task_id, tag_id)
            VALUES ($1, $2)",
        )
        .bind(id)
        .bind(tag_id)
        .execute(conn.as_mut())
        .await;
        if let Err(err) = res {
            let mut error = None;
            if let Some(pg_err) = err.as_database_error() {
                if pg_err.is_unique_violation() {
                    return Ok(());
                }

                let message = pg_err.message();
                if message == "resource_not_found" || message == "ownership_mismatch" {
                    error = Some(Error::Constraint(ConstraintViolation::NotFound(
                        Resource::Tag,
                    )));
                }
            }

            return Err(error.unwrap_or(err.into()));
        }

        let res = res.unwrap();
        if res.rows_affected() != 0 {
            set_updated_timestamp(conn, id, user_id).await?;
        }

        Ok(())
    }

    async fn remove_tag_from_task(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        tag_id: TagID,
    ) -> Result<()> {
        let res = query(
            "DELETE FROM app.task_tags
            WHERE task_id = $1 AND tag_id = $2
            RETURNING *",
        )
        .bind(id)
        .bind(tag_id)
        .execute(conn.as_mut())
        .await?;

        if res.rows_affected() != 0 {
            set_updated_timestamp(conn, id, user_id).await?;
        }

        Ok(())
    }

    async fn replace_tags_on_task(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<TagModel>> {
        let existing_tags: Vec<TagID> =
            query_scalar("SELECT tag_id FROM app.task_tags WHERE task_id = $1")
                .bind(id)
                .fetch_all(conn.as_mut())
                .await?;

        let current: HashSet<TagID> = existing_tags.into_iter().collect();
        let desired: HashSet<TagID> = tag_ids.iter().copied().collect();
        if current == desired {
            return Self::fetch_task_tags(conn, id, user_id).await;
        }

        let to_remove: Vec<&TagID> = current.difference(&desired).collect();
        let num_removed =
            query("DELETE FROM app.task_tags WHERE task_id = $1 AND tag_id = ANY($2)")
                .bind(id)
                .bind(&to_remove)
                .execute(conn.as_mut())
                .await?
                .rows_affected();
        assert_eq!(num_removed as usize, to_remove.len());

        let to_add: Vec<&TagID> = desired.difference(&current).collect();
        let num_added = query(
            "INSERT INTO app.task_tags (task_id, tag_id)
            SELECT $1, unnest_tag
            FROM UNNEST($2::uuid[]) AS unnest_tag",
        )
        .bind(id)
        .bind(&to_add)
        .execute(conn.as_mut())
        .await
        .map_err(|err| {
            if let Some(pg_err) = err.as_database_error() {
                let message = pg_err.message();
                if message == "resource_not_found" || message == "ownership_mismatch" {
                    return Error::Constraint(ConstraintViolation::NotFound(Resource::Tag));
                }
            }

            err.into()
        })?
        .rows_affected();
        assert_eq!(num_added as usize, to_add.len());

        set_updated_timestamp(conn, id, user_id).await?;

        Self::fetch_task_tags(conn, id, user_id).await
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn task_exists(&self, id: TaskID, user_id: UserID) -> Result<bool> {
        let mut conn = self.db.acquire().await?;

        let exists = query_scalar(
            "SELECT EXISTS (SELECT 1 FROM app.tasks WHERE id = $1 AND created_by = $2)",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await?;

        conn.close().await?;
        Ok(exists)
    }

    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<TaskModel>> {
        let mut conn = self.db.acquire().await?;

        let tasks = Self::fetch_all_task_rows(&mut conn, user_id, query).await?;

        conn.close().await?;
        Ok(tasks)
    }

    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<TaskModel> {
        let mut tx = self.db.begin().await?;

        let task = Self::create_model(&mut tx, user_id, create_model).await?;

        tx.commit().await?;
        Ok(task)
    }

    async fn get(&self, id: TaskID, user_id: UserID) -> Result<TaskState<TaskModel>> {
        let mut conn = self.db.acquire().await?;

        let state = Self::fetch_task_row(&mut conn, id, user_id).await?;

        conn.close().await?;
        Ok(state)
    }

    async fn update(
        &self,
        id: TaskID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TaskState<TaskModel>> {
        let mut tx = self.db.begin().await?;

        if let Err(err) = Self::update_task_row(&mut tx, id, user_id, update_model).await
            && let Error::Constraint(ConstraintViolation::NotFound(Resource::Task)) = err
        {
            return Ok(TaskState::None);
        }
        let state = Self::fetch_task_row(&mut tx, id, user_id).await?;

        tx.commit().await?;
        if state.as_ref().is_none() {
            unreachable!();
        }
        Ok(state)
    }

    async fn delete(&self, id: TaskID, user_id: UserID) -> Result<()> {
        let mut tx = self.db.begin().await?;

        Self::delete_task_row(&mut tx, id, user_id).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn list_tags(&self, id: TaskID, user_id: UserID) -> Result<Vec<TagModel>> {
        let mut conn = self.db.acquire().await?;

        match Self::fetch_task_row(&mut conn, id, user_id).await? {
            TaskState::Deleted(_) => {
                return Err(Error::Constraint(ConstraintViolation::Deleted(
                    Resource::Task,
                )));
            }
            TaskState::None => {
                return Err(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Task,
                )));
            }
            _ => (),
        }
        let tags = Self::fetch_task_tags(&mut conn, id, user_id).await?;

        conn.close().await?;
        Ok(tags)
    }

    async fn add_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        let mut tx = self.db.begin().await?;

        match Self::fetch_task_row(&mut tx, id, user_id).await? {
            TaskState::Deleted(_) => {
                return Err(Error::Constraint(ConstraintViolation::Deleted(
                    Resource::Task,
                )));
            }
            TaskState::None => {
                return Err(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Task,
                )));
            }
            _ => (),
        }
        Self::add_tag_to_task(&mut tx, id, user_id, tag_id).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn remove_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        let mut tx = self.db.begin().await?;

        match Self::fetch_task_row(&mut tx, id, user_id).await? {
            TaskState::Deleted(_) => {
                return Err(Error::Constraint(ConstraintViolation::Deleted(
                    Resource::Task,
                )));
            }
            TaskState::None => {
                return Err(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Task,
                )));
            }
            _ => (),
        }
        Self::remove_tag_from_task(&mut tx, id, user_id, tag_id).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn set_tags(
        &self,
        id: TaskID,
        user_id: UserID,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<TagModel>> {
        let mut tx = self.db.begin().await?;

        match Self::fetch_task_row(&mut tx, id, user_id).await? {
            TaskState::Deleted(_) => {
                return Err(Error::Constraint(ConstraintViolation::Deleted(
                    Resource::Task,
                )));
            }
            TaskState::None => {
                return Err(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Task,
                )));
            }
            _ => (),
        }
        let tags = Self::replace_tags_on_task(&mut tx, id, user_id, tag_ids).await?;

        tx.commit().await?;
        Ok(tags)
    }
}
