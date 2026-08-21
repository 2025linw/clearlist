use async_trait::async_trait;
use sqlx::{PgPool, QueryBuilder, query, query_scalar};

use crate::{
    error::{
        Resource,
        repo::{Error, Result},
    },
    user::types::UserID,
    utils::repo::query_as,
};

use super::types::{
    CategoryID, CategoryModel,
    repo::{CreateModel, UpdateModel},
};

#[async_trait]
pub trait CategoryRepository: Send + Sync + Clone + 'static {
    async fn get_id_by_name(&self, user_id: UserID, name: String) -> Result<CategoryID>;

    async fn list(&self, user_id: UserID) -> Result<Vec<CategoryModel>>;
    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<CategoryModel>;
    async fn get(&self, id: CategoryID, user_id: UserID) -> Result<Option<CategoryModel>>;
    async fn update(
        &self,
        id: CategoryID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<CategoryModel>;
    async fn delete(&self, id: CategoryID, user_id: UserID) -> Result<()>;
}

#[derive(Clone)]
pub struct PgCategoryRepository {
    db: PgPool,
}

impl PgCategoryRepository {
    pub fn init(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CategoryRepository for PgCategoryRepository {
    async fn get_id_by_name(&self, user_id: UserID, name: String) -> Result<CategoryID> {
        let mut conn = self.db.acquire().await?;

        let id = query_scalar(
            "SELECT id FROM app.categories
            WHERE name = $1 AND created_by = $2",
        )
        .bind(name)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Category))?;

        conn.close().await?;
        Ok(id)
    }

    async fn list(&self, user_id: UserID) -> Result<Vec<CategoryModel>> {
        let mut conn = self.db.acquire().await?;

        let categories = query_as::<CategoryModel>(
            "SELECT * FROM app.categories
            WHERE created_by = $1
            ORDER BY position_key, id",
        )
        .bind(user_id)
        .fetch_all(conn.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Category))?;

        conn.close().await?;
        Ok(categories)
    }

    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<CategoryModel> {
        let mut tx = self.db.begin().await?;

        let id = query_as::<CategoryModel>(
            "INSERT INTO app.categories (id, name, position_key, created_by)
            VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(CategoryID::new_random())
        .bind(create_model.name)
        .bind(create_model.position_key)
        .bind(user_id)
        .fetch_one(tx.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Category))?;

        tx.commit().await?;
        Ok(id)
    }

    async fn get(&self, id: CategoryID, user_id: UserID) -> Result<Option<CategoryModel>> {
        let mut conn = self.db.acquire().await?;

        let task_opt = query_as::<CategoryModel>(
            "SELECT * FROM app.categories
            WHERE id = $1 AND created_by = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(conn.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Category))?;

        conn.close().await?;
        return Ok(task_opt);
    }

    async fn update(
        &self,
        id: CategoryID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<CategoryModel> {
        let mut tx = self.db.begin().await?;

        let mut builder = QueryBuilder::new("UPDATE app.categories SET ");
        update_model.add_to_builder(&mut builder);
        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" AND created_by = ");
        builder.push_bind(user_id);
        builder.push(" RETURNING *");

        let category = builder
            .build_query_as::<CategoryModel>()
            .fetch_one(tx.as_mut())
            .await
            .map_err(|err| Error::from_sqlx(err, Resource::Category))?;

        tx.commit().await?;
        Ok(category)
    }

    async fn delete(&self, id: CategoryID, user_id: UserID) -> Result<()> {
        let mut tx = self.db.begin().await?;

        query(
            "DELETE FROM app.categories
            WHERE id = $1 AND created_by = $2
            RETURNING id",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(tx.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Category))?;

        tx.commit().await?;
        Ok(())
    }
}
