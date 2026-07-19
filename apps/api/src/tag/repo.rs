#[cfg(test)]
mod tests;

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, QueryBuilder};
use uuid::Uuid;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    user::types::UserID,
    utils::repo::query_as,
};

use super::types::{
    Model, TagID,
    repo::{CreateModel, QueryOpts, UpdateModel},
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

    async fn fetch_all_tags(
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

    async fn create_tag(
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
                return Error::Constraint(ConstraintViolation::MissingUser);
            }

            err.into()
        })
    }

    async fn fetch_tag(
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

    async fn update_tag(
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

    async fn delete_tag(conn: &mut PgConnection, id: TagID, user_id: UserID) -> Result<()> {
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
        let tags = Self::fetch_all_tags(&mut conn, user_id, query).await?;
        conn.close().await?;

        Ok(tags)
    }

    async fn create(&self, user_id: UserID, create_tag: CreateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let tag = Self::create_tag(&mut tx, user_id, create_tag).await?;
        tx.commit().await?;

        Ok(tag)
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<Model>> {
        let mut conn = self.db.acquire().await?;
        let tag_opt = Self::fetch_tag(&mut conn, id, user_id).await?;
        conn.close().await?;

        Ok(tag_opt)
    }

    async fn update(&self, id: TagID, user_id: UserID, update_tag: UpdateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let tag = Self::update_tag(&mut tx, id, user_id, update_tag).await?;
        tx.commit().await?;

        Ok(tag)
    }

    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        let mut tx = self.db.begin().await?;
        Self::delete_tag(&mut tx, id, user_id).await?;
        tx.commit().await?;

        Ok(())
    }
}
