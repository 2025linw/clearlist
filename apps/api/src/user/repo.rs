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
    UserID, UserModel,
    repo::{CreateModel, UpdateModel},
};

#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    async fn create(&self, create_model: CreateModel) -> Result<UserModel>;
    async fn get(&self, id: UserID) -> Result<Option<UserModel>>;
    async fn update(&self, id: UserID, update_model: UpdateModel) -> Result<UserModel>;
}

#[derive(Clone)]
pub struct PgUserRepository {
    db: PgPool,
}

impl PgUserRepository {
    pub fn init(db: PgPool) -> Self {
        Self { db }
    }

    async fn create_model(conn: &mut PgConnection, create_model: CreateModel) -> Result<UserModel> {
        Ok(query_as::<UserModel>(
            "INSERT INTO app.users (id, display_name, updated_at, created_at)
            VALUES ($1, $2, $3, $3)
            RETURNING *",
        )
        .bind(create_model.id)
        .bind(create_model.display_name)
        .bind(create_model.created_at)
        .fetch_one(conn.as_mut())
        .await?)
    }

    async fn fetch_user(conn: &mut PgConnection, id: UserID) -> Result<Option<UserModel>> {
        let user_opt = query_as::<UserModel>(
            "SELECT * FROM app.users
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn.as_mut())
        .await?;

        Ok(user_opt)
    }

    async fn update_model(
        conn: &mut PgConnection,
        id: UserID,
        update_model: UpdateModel,
    ) -> Result<UserModel> {
        let mut builder = QueryBuilder::new("UPDATE app.users SET ");
        update_model.add_to_builder(&mut builder);
        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" RETURNING *");

        let query = builder.build_query_as::<UserModel>();

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
    async fn create(&self, create_model: CreateModel) -> Result<UserModel> {
        let mut tx = self.db.begin().await?;
        let user = Self::create_model(&mut tx, create_model).await?;
        tx.commit().await?;

        Ok(user)
    }

    async fn get(&self, id: UserID) -> Result<Option<UserModel>> {
        let mut conn = self.db.acquire().await?;
        let user_opt = Self::fetch_user(&mut conn, id).await?;
        conn.close().await?;

        Ok(user_opt)
    }

    async fn update(&self, id: UserID, update_model: UpdateModel) -> Result<UserModel> {
        let mut tx = self.db.begin().await?;
        let user = Self::update_model(&mut tx, id, update_model).await?;
        tx.commit().await?;

        Ok(user)
    }
}
