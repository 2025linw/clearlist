#[cfg(test)]
pub mod tests;

mod types;

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, QueryBuilder};
use uuid::Uuid;

use super::types::{Model, TagID};
use crate::{
    error::repo::{ConstraintViolation, Error, Resource, Result},
    user::types::UserID,
    utils::repo::query_as,
};

#[async_trait]
pub trait TagRepository: Send + Sync + Clone {
    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<Model>>;
    async fn create(&self, user_id: UserID, create_tag: CreateModel) -> Result<Model>;
    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<Model>>;
    async fn update(&self, id: TagID, user_id: UserID, update_tag: UpdateModel) -> Result<Model>;
    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()>;
}

#[derive(Clone)]
pub struct PgTagRepository {
    db: PgPool,
}

impl PgTagRepository {
    pub fn init(db: PgPool) -> Self {
        Self { db }
    }

    async fn list_inner(
        conn: &mut PgConnection,
        user_id: UserID,
        query: Option<QueryOpts>,
    ) -> Result<Vec<Model>> {
        let mut builder = QueryBuilder::new("SELECT * FROM app.tags WHERE created_by = ");
        builder.push_bind(user_id);
        if let Some(opts) = query {
            opts.add_to_builder(&mut builder);
        }

        let query = builder.build_query_as::<Model>();

        Ok(query.fetch_all(conn.as_mut()).await?)
    }

    async fn create_inner(
        conn: &mut PgConnection,
        user_id: UserID,
        create_tag: CreateModel,
    ) -> Result<Model> {
        query_as::<Model>(
            "INSERT INTO app.tags (id, label, category, position_key, created_by)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(create_tag.label)
        .bind(create_tag.category)
        .bind(create_tag.position_key)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await
        .map_err(|err| {
            if let Some(pg_err) = err.as_database_error()
                && pg_err.is_foreign_key_violation()
            {
                return Error::Constraint(ConstraintViolation::NotFound(Resource::User));
            }

            err.into()
        })
    }

    async fn get_inner(
        conn: &mut PgConnection,
        id: TagID,
        user_id: UserID,
    ) -> Result<Option<Model>> {
        let tag_opt = query_as::<Model>(
            "SELECT * FROM app.tags
            WHERE id = $1 AND created_by = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(conn.as_mut())
        .await?;

        Ok(tag_opt)
    }

    async fn update_inner(
        conn: &mut PgConnection,
        id: TagID,
        user_id: UserID,
        update_tag: UpdateModel,
    ) -> Result<Model> {
        let mut builder = QueryBuilder::new("UPDATE app.tags SET ");
        update_tag.add_to_builder(&mut builder);
        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" AND created_by = ");
        builder.push_bind(user_id);
        builder.push(" RETURNING *");

        let query = builder.build_query_as::<Model>();

        query.fetch_one(conn.as_mut()).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                return Error::Constraint(ConstraintViolation::NotFound(Resource::Tag));
            }

            err.into()
        })
    }

    async fn delete_inner(conn: &mut PgConnection, id: TagID, user_id: UserID) -> Result<()> {
        query_as::<Model>(
            "DELETE FROM app.tags
            WHERE id = $1 AND created_by = $2
            RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await
        .map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                return Error::Constraint(ConstraintViolation::NotFound(Resource::Tag));
            }

            err.into()
        })?;

        Ok(())
    }
}

#[async_trait]
impl TagRepository for PgTagRepository {
    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<Model>> {
        let mut conn = self.db.acquire().await?;
        let tags = Self::list_inner(&mut conn, user_id, query).await?;
        conn.close().await?;

        Ok(tags)
    }

    async fn create(&self, user_id: UserID, tag: CreateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let tag = Self::create_inner(&mut tx, user_id, tag).await?;
        tx.commit().await?;

        Ok(tag)
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<Model>> {
        let mut conn = self.db.acquire().await?;
        let tag_opt = Self::get_inner(&mut conn, id, user_id).await?;
        conn.close().await?;

        Ok(tag_opt)
    }

    async fn update(&self, id: TagID, user_id: UserID, tag: UpdateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let tag = Self::update_inner(&mut tx, id, user_id, tag).await?;
        tx.commit().await?;

        Ok(tag)
    }

    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        let mut tx = self.db.begin().await?;
        Self::delete_inner(&mut tx, id, user_id).await?;
        tx.commit().await?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct QueryOpts {
    filter: types::Filter,
    sort: types::Sort,
    pagination: types::Pagination,
}

impl QueryOpts {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        // Filter
        if let Some(filter) = self.filter.category {
            builder.push(" AND category = ");
            builder.push_bind(filter);
        }

        // Sort
        if let Some((by, order)) = self.sort.sort {
            builder.push(format!(" ORDER BY {} {}", by, order));
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
    pub label: String,
    pub category: Option<String>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub label: Option<String>,
    pub category: Option<Option<String>>,

    pub position_key: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(label) = self.label {
            separated.push("label = ");
            separated.push_bind_unseparated(label);
        }
        if let Some(category) = self.category {
            separated.push("category = ");
            separated.push_bind_unseparated(category);
        }

        if let Some(position_key) = self.position_key {
            separated.push("position_key = ");
            separated.push_bind_unseparated(position_key);
        }
    }
}
