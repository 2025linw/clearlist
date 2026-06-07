use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{Result, query_as_wrapper},
    models::user::Model,
};

pub trait UserRepository {
    async fn create(&self) -> Result<Model>;
    async fn get(&self, id: Uuid) -> Result<Option<Model>>;
    async fn list(&self) -> Result<Vec<Model>>;
    async fn update(&self) -> Result<Model>;
    async fn delete(&self) -> Result<()>;
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
    async fn create(&self) -> Result<Model> {
        todo!()
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

    async fn list(&self) -> Result<Vec<Model>> {
        todo!()
    }

    async fn update(&self) -> Result<Model> {
        todo!()
    }

    async fn delete(&self) -> Result<()> {
        todo!()
    }
}
