#[cfg(test)]
mod tests;

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, QueryBuilder};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    utils::repo::query_as,
};

use super::types::{
    Model, UserID,
    repo::{CreateModel, UpdateModel},
};

#[async_trait]
pub trait UserRepository: Send + Sync + Clone {
    async fn create(&self, create_user: CreateModel) -> Result<Model>;
    async fn get(&self, id: UserID) -> Result<Option<Model>>;
    async fn update(&self, id: UserID, update_user: UpdateModel) -> Result<Model>;
}

#[derive(Clone)]
pub struct PgUserRepository {
    db: PgPool,
}

impl PgUserRepository {
    pub fn init(db: PgPool) -> Self {
        Self { db }
    }

    async fn create_user(conn: &mut PgConnection, create_user: CreateModel) -> Result<Model> {
        Ok(query_as::<Model>(
            "INSERT INTO app.users (id, display_name, preferred_timezone, completed_task_retention, updated_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $5)
            RETURNING *",
        )
        .bind(create_user.id)
        .bind(create_user.display_name)
        .bind(create_user.preferred_timezone)
        .bind(create_user.completed_task_retention)
        .bind(create_user.created_at)
        .fetch_one(conn.as_mut())
        .await?)
    }

    async fn fetch_user(conn: &mut PgConnection, id: UserID) -> Result<Option<Model>> {
        let user_opt = query_as::<Model>(
            "SELECT * FROM app.users
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn.as_mut())
        .await?;

        Ok(user_opt)
    }

    async fn update_user(
        conn: &mut PgConnection,
        id: UserID,
        update_user: UpdateModel,
    ) -> Result<Model> {
        let mut builder = QueryBuilder::new("UPDATE app.users SET ");
        update_user.add_to_builder(&mut builder);
        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" RETURNING *");

        let query = builder.build_query_as::<Model>();

        query.fetch_one(conn.as_mut()).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                return Error::Constraint(ConstraintViolation::NotFound(Resource::User));
            }

            err.into()
        })
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, create_user: CreateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let user = Self::create_user(&mut tx, create_user).await?;
        tx.commit().await?;

        Ok(user)
    }

    async fn get(&self, id: UserID) -> Result<Option<Model>> {
        let mut conn = self.db.acquire().await?;
        let user_opt = Self::fetch_user(&mut conn, id).await?;
        conn.close().await?;

        Ok(user_opt)
    }

    async fn update(&self, id: UserID, update_user: UpdateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let user = Self::update_user(&mut tx, id, update_user).await?;
        tx.commit().await?;

        Ok(user)
    }
}
