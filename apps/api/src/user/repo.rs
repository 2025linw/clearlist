// mod types;

#[cfg(test)]
pub mod tests;

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, QueryBuilder};

use super::types::{Model, UserID};
use crate::{
    error::repo::{ConstraintViolation, Error, Resource, Result},
    utils::repo::query_as,
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

    async fn create_inner(conn: &mut PgConnection, create_user: CreateModel) -> Result<Model> {
        Ok(query_as::<Model>(
            "INSERT INTO app.users (id, display_name, created_at)
            VALUES ($1, $2, $3)
            RETURNING *",
        )
        .bind(create_user.id)
        .bind(create_user.display_name)
        .bind(create_user.created_at)
        .fetch_one(conn.as_mut())
        .await?)
    }

    async fn get_inner(conn: &mut PgConnection, id: UserID) -> Result<Option<Model>> {
        let user_opt = query_as::<Model>(
            "SELECT * FROM app.users
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn.as_mut())
        .await?;

        Ok(user_opt)
    }

    async fn update_inner(
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
    async fn create(&self, user: CreateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let user = Self::create_inner(&mut tx, user).await?;
        tx.commit().await?;

        Ok(user)
    }

    async fn get(&self, id: UserID) -> Result<Option<Model>> {
        let mut conn = self.db.acquire().await?;
        let user_opt = Self::get_inner(&mut conn, id).await?;
        conn.close().await?;

        Ok(user_opt)
    }

    async fn update(&self, id: UserID, user: UpdateModel) -> Result<Model> {
        let mut tx = self.db.begin().await?;
        let user = Self::update_inner(&mut tx, id, user).await?;
        tx.commit().await?;

        Ok(user)
    }
}

#[derive(Debug)]
pub struct CreateModel {
    pub id: UserID,

    pub display_name: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub display_name: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(display_name) = self.display_name {
            separated.push("display_name = ");
            separated.push_bind_unseparated(display_name);
        }
    }
}
