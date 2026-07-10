#[cfg(test)]
pub mod tests;

mod types;

use std::collections::{HashMap, HashSet, hash_map::Entry};

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, QueryBuilder, query, query_scalar};
use uuid::Uuid;

use super::types::{Model, TaskID, TaskTag};
use crate::{
    error::repo::{ConstraintViolation, Error, Resource, Result},
    tag::types::{Model as TagModel, TagID},
    types::date::StartPrecision,
    user::types::UserID,
    utils::repo::{query_as, set_updated_timestamp},
};

#[async_trait]
pub trait TaskRepository: Send + Sync + Clone {
    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<Model>>;
    async fn create(&self, user_id: UserID, create_task: CreateModel) -> Result<Model>;
    async fn get(&self, id: TaskID, user_id: UserID) -> Result<Option<Model>>;
    async fn update(&self, id: TaskID, user_id: UserID, update_task: UpdateModel) -> Result<Model>;
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

    async fn task_exists(conn: &mut PgConnection, id: TaskID, user_id: UserID) -> Result<bool> {
        let exists: bool = query_scalar(
                "SELECT EXISTS(SELECT 1 FROM app.tasks WHERE id = $1 AND created_by = $2 AND deleted_at IS NULL)"
            )
            .bind(id)
            .bind(user_id)
            .fetch_one(conn.as_mut())
            .await?;

        Ok(exists)
    }

    async fn list_inner(
        conn: &mut PgConnection,
        user_id: UserID,
        query: Option<QueryOpts>,
    ) -> Result<Vec<Model>> {
        let mut builder = QueryBuilder::new(
            "SELECT t.* FROM app.tasks t LEFT JOIN app.task_tags tt ON t.id = tt.task_id WHERE created_by = ",
        );
        builder.push_bind(user_id);
        if let Some(opts) = query {
            opts.add_to_builder(&mut builder);
        } else {
            builder.push(" GROUP BY t.id");
        }

        let query = builder.build_query_as::<Model>();

        let mut tasks = query.fetch_all(conn.as_mut()).await?;

        // Get tags
        let task_ids: Vec<TaskID> = tasks.iter().map(|task| task.id).collect();
        let tags = query_as::<TaskTag>(
            "SELECT tt.task_id, tg.*
            FROM app.task_tags tt
            LEFT JOIN app.tags tg ON tt.tag_id = tg.id
            WHERE tt.task_id = ANY($1)",
        )
        .bind(task_ids)
        .fetch_all(conn.as_mut())
        .await?;

        let mut task_tag_map: HashMap<TaskID, Vec<TagModel>> = HashMap::new();
        for TaskTag { task_id, tag } in tags {
            if let Entry::Vacant(e) = task_tag_map.entry(task_id) {
                e.insert(vec![tag]);
            } else {
                task_tag_map.get_mut(&task_id).unwrap().push(tag);
            }
        }
        for task in tasks.iter_mut() {
            if let Some(tags) = task_tag_map.remove(&task.id) {
                task.tags = tags;
            }
        }

        Ok(tasks)
    }

    async fn create_inner(
        conn: &mut PgConnection,
        user_id: UserID,
        create_task: CreateModel,
    ) -> Result<Model> {
        let mut task = query_as::<Model>(
            "INSERT INTO app.tasks (id, title, notes, start_dt, has_time, deadline, position_key, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(create_task.title)
        .bind(create_task.notes)
        .bind(create_task.start)
        .bind(matches!(
            create_task.start_precision,
            StartPrecision::DateTime
        ))
        .bind(create_task.deadline)
        .bind(create_task.position_key)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await.map_err(|err| {
            if let Some(pg_err) = err.as_database_error() && pg_err.is_foreign_key_violation() {
                return Error::Constraint(ConstraintViolation::NotFound(Resource::User));
            }

            err.into()
        })?;

        if !create_task.tags.is_empty() {
            task.tags = Self::set_tags_inner(conn, task.id, user_id, create_task.tags).await?;
        }

        Ok(task)
    }

    async fn get_inner(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
    ) -> Result<Option<Model>> {
        let task_opt = query_as::<Model>(
            "SELECT * FROM app.tasks
            WHERE id = $1 AND created_by = $2 AND deleted_at IS NULL",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(conn.as_mut())
        .await?;

        match task_opt {
            Some(mut task) => {
                task.tags = Self::list_tags_inner(conn, id, user_id).await?;

                Ok(Some(task))
            }
            None => Ok(None),
        }
    }

    async fn update_inner(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        update_task: UpdateModel,
    ) -> Result<Model> {
        let tag_ids_opt = update_task.tags.clone();

        let mut builder = QueryBuilder::new("UPDATE app.tasks SET ");
        update_task.add_to_builder(&mut builder);
        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" AND created_by = ");
        builder.push_bind(user_id);
        builder.push(" AND deleted_at IS NULL RETURNING *");

        let query = builder.build_query_as::<Model>();

        let mut task = query.fetch_one(conn.as_mut()).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                return Error::Constraint(ConstraintViolation::NotFound(Resource::Task));
            }

            err.into()
        })?;

        if let Some(tag_ids) = tag_ids_opt
            && !tag_ids.is_empty()
        {
            task.tags = Self::set_tags_inner(conn, id, user_id, tag_ids).await?;
        }

        Ok(task)
    }

    async fn delete_inner(conn: &mut PgConnection, id: TaskID, user_id: UserID) -> Result<()> {
        query_as::<Model>(
            "DELETE FROM app.tasks
            WHERE id = $1 AND created_by = $2 AND deleted_at IS NULL
            RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await
        .map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                return Error::Constraint(ConstraintViolation::NotFound(Resource::Task));
            }

            err.into()
        })?;

        Ok(())
    }

    async fn list_tags_inner(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
    ) -> Result<Vec<TagModel>> {
        if !Self::task_exists(conn, id, user_id).await? {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        let tags = query_as::<TagModel>(
            "SELECT tg.*
            FROM app.tags tg
            JOIN app.task_tags tt ON tg.id = tt.tag_id
            JOIN app.tasks t ON tt.task_id = t.id
            WHERE tt.task_id = $1 AND t.created_by = $2
            ORDER BY tg.category ASC NULLS FIRST, tg.label ASC, tg.updated_at DESC, tg.id ASC",
        )
        .bind(id)
        .bind(user_id)
        .fetch_all(conn)
        .await?;

        Ok(tags)
    }

    async fn add_tag_inner(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        tag_id: TagID,
    ) -> Result<()> {
        if !Self::task_exists(conn, id, user_id).await? {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        let res = query(
            "INSERT INTO app.task_tags (task_id, tag_id)
            VALUES ($1, $2) RETURNING *",
        )
        .bind(id)
        .bind(tag_id)
        .fetch_one(conn.as_mut())
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
            } else if matches!(err, sqlx::Error::RowNotFound) {
                error = Some(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Tag,
                )));
            }

            return Err(error.unwrap_or(err.into()));
        }

        set_updated_timestamp(conn, id, user_id).await?;

        Ok(())
    }

    async fn remove_tag_inner(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        tag_id: TagID,
    ) -> Result<()> {
        if !Self::task_exists(conn, id, user_id).await? {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        query(
            "DELETE FROM app.task_tags
            WHERE task_id = $1 AND tag_id = $2
            RETURNING *",
        )
        .bind(id)
        .bind(tag_id)
        .execute(conn.as_mut())
        .await?;

        set_updated_timestamp(conn, id, user_id).await?;

        Ok(())
    }

    async fn set_tags_inner(
        conn: &mut PgConnection,
        id: TaskID,
        user_id: UserID,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<TagModel>> {
        if !Self::task_exists(conn, id, user_id).await? {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        let existing_tags: Vec<TagID> =
            query_scalar("SELECT tag_id FROM app.task_tags WHERE task_id = $1")
                .bind(id)
                .fetch_all(conn.as_mut())
                .await?;

        let current: HashSet<TagID> = existing_tags.into_iter().collect();
        let desired: HashSet<TagID> = tag_ids.iter().copied().collect();
        if current == desired {
            return Self::list_tags_inner(conn, id, user_id).await;
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

        Self::list_tags_inner(conn, id, user_id).await
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<Model>> {
        let mut conn = self.db.acquire().await?;
        let tasks = Self::list_inner(&mut conn, user_id, query).await?;
        conn.close().await?;

        Ok(tasks)
    }

    async fn create(&self, user_id: UserID, task: CreateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let task = Self::create_inner(&mut tx, user_id, task).await?;
        tx.commit().await?;

        Ok(task)
    }

    async fn get(&self, id: TaskID, user_id: UserID) -> Result<Option<Model>> {
        let mut conn = self.db.acquire().await?;
        let task_opt = Self::get_inner(&mut conn, id, user_id).await?;
        conn.close().await?;

        Ok(task_opt)
    }

    async fn update(&self, id: TaskID, user_id: UserID, task: UpdateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let task = Self::update_inner(&mut tx, id, user_id, task).await?;
        tx.commit().await?;

        Ok(task)
    }

    async fn delete(&self, id: TaskID, user_id: UserID) -> Result<()> {
        let mut tx = self.db.begin().await?;
        Self::delete_inner(&mut tx, id, user_id).await?;
        tx.commit().await?;

        Ok(())
    }

    async fn list_tags(&self, id: TaskID, user_id: UserID) -> Result<Vec<TagModel>> {
        let mut conn = self.db.acquire().await?;
        let tags = Self::list_tags_inner(&mut conn, id, user_id).await?;
        conn.close().await?;

        Ok(tags)
    }

    async fn add_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        let mut tx = self.db.begin().await?;
        Self::add_tag_inner(&mut tx, id, user_id, tag_id).await?;
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
        let tags = Self::set_tags_inner(&mut tx, id, user_id, tag_ids).await?;
        tx.commit().await?;

        Ok(tags)
    }

    async fn remove_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        let mut tx = self.db.begin().await?;
        Self::remove_tag_inner(&mut tx, id, user_id, tag_id).await?;
        tx.commit().await?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct QueryOpts {
    pagination: types::Pagination,
    sort: types::Sort,
    filter: types::Filter,
}

impl QueryOpts {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        // Filter
        if let Some(filter) = self.filter.start {
            builder.push(" AND ");
            filter.add_to_builder(builder);
        }
        if let Some(filter) = self.filter.deadline {
            builder.push(" AND ");
            filter.add_to_builder(builder);
        }
        if let Some(completed) = self.filter.completed {
            builder.push(" AND ");
            if completed {
                builder.push("completed_at IS NOT NULL");
            } else {
                builder.push("completed_at IS NULL");
            }
        }
        if let Some(deleted) = self.filter.deleted {
            builder.push(" AND ");
            if deleted {
                builder.push("deleted_at IS NOT NULL");
            } else {
                builder.push("deleted_at IS NULL");
            }
        }
        if let Some(tags) = self.filter.tags
            && !tags.is_empty()
        {
            // NOTE: Make sure this is last as it will contain `HAVING` clauses
            builder.push(" AND tt.tag_id = ANY(");
            builder.push_bind(tags.clone());
            builder.push(") GROUP BY t.id HAVING COUNT(DISTINCT tt.tag_id) = cardinality(");
            builder.push_bind(tags.clone());
            builder.push(")");
        } else {
            builder.push(" GROUP BY t.id");
        }

        // Sort
        if let Some((by, order)) = self.sort.sort {
            builder.push(format!(" ORDER BY {} {} NULLS LAST", by, order));
        } else {
            builder.push(" ORDER BY id ASC");
        }

        // Pagination
        if let Some(limit) = self.pagination.limit {
            builder.push(format!(" LIMIT {limit}"));
        }
        if let Some(offset) = self.pagination.offset {
            builder.push(format!(" OFFSET {offset}"));
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Default, Clone))]
pub struct CreateModel {
    pub title: String,
    pub notes: Option<String>,
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    pub start_precision: StartPrecision,
    pub deadline: Option<chrono::NaiveDate>,
    pub tags: Vec<TagID>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub title: Option<String>,
    pub notes: Option<Option<String>>,
    pub start: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub start_precision: Option<StartPrecision>,
    pub deadline: Option<Option<chrono::NaiveDate>>,
    pub tags: Option<Vec<TagID>>,

    pub completed: Option<bool>,
    pub deleted: Option<bool>,

    pub position_key: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(title) = self.title {
            separated.push("title = ");
            separated.push_bind_unseparated(title);
        }
        if let Some(notes) = self.notes {
            separated.push("notes = ");
            separated.push_bind_unseparated(notes);
        }
        if let Some(start) = self.start {
            separated.push("start_dt = ");
            separated.push_bind_unseparated(start);
        }
        if let Some(start_precision) = self.start_precision {
            separated.push("has_time = ");
            separated.push_bind_unseparated(matches!(start_precision, StartPrecision::DateTime));
        }
        if let Some(deadline) = self.deadline {
            separated.push("deadline = ");
            separated.push_bind_unseparated(deadline);
        }

        if let Some(completed) = self.completed {
            if completed {
                separated.push("completed_at = CURRENT_TIMESTAMP");
            } else {
                separated.push("completed_at = NULL");
            }
        }
        if let Some(deleted) = self.deleted {
            if deleted {
                separated.push("deleted_at = CURRENT_TIMESTAMP");
            } else {
                separated.push("deleted_at = NULL");
            }
        }

        if let Some(position_key) = self.position_key {
            separated.push("position_key = ");
            separated.push_bind_unseparated(position_key);
        }
    }
}
