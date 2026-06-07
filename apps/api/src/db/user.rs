use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{Result, query_as_wrapper},
    models::user::Model,
};

pub trait UserRepository {
    async fn create(&self, user: Model) -> Result<Model>;
    async fn get(&self, id: Uuid) -> Result<Option<Model>>;
    async fn update(&self, id: Uuid, user: Model) -> Result<Option<Model>>;
}

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    async fn create(&self, user: Model) -> Result<Model> {
        let user = query_as_wrapper::<Model>(
            "INSERT INTO app.users
            (id, display_name, created_at) VALUES
            ($1, $2, $3)
            RETURNING *",
        )
        .bind(user.id)
        .bind(user.display_name)
        .bind(user.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get(&self, id: Uuid) -> Result<Option<Model>> {
        let user = query_as_wrapper::<Model>(
            "SELECT *
            FROM app.users
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, id: Uuid, user: Model) -> Result<Option<Model>> {
        let user = query_as_wrapper::<Model>(
            "UPDATE app.users
            SET (display_name)
            = ($2)
            WHERE id = $1
            RETURNING *",
        )
        .bind(id)
        .bind(user.display_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
