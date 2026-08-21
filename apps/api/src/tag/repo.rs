use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, QueryBuilder, query, query_scalar};

use crate::{
    error::{
        Resource,
        repo::{Error, Result},
    },
    user::types::UserID,
    utils::repo::query_as,
};

use super::types::{
    TagID, TagModel,
    repo::{CreateModel, QueryOpts, UpdateModel},
};

#[async_trait]
pub trait TagRepository: Send + Sync + Clone + 'static {
    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<TagModel>>;
    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<TagModel>;
    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<TagModel>>;
    async fn update(
        &self,
        id: TagID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TagModel>;
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

    async fn fetch_all_tag_rows(
        conn: &mut PgConnection,
        user_id: UserID,
        query: Option<QueryOpts>,
    ) -> Result<Vec<TagModel>> {
        let mut builder = QueryBuilder::new(
            "SELECT t.*, tc.name as category_name, tc.position_key as category_position_key
            FROM app.tags t
            LEFT JOIN app.categories tc ON t.category_id = tc.id
            WHERE t.created_by = ",
        );
        builder.push_bind(user_id);
        if let Some(opts) = query {
            opts.add_to_builder(&mut builder);
        }

        let query = builder.build_query_as::<TagModel>();

        let tasks = query
            .fetch_all(conn.as_mut())
            .await
            .map_err(|err| Error::from_sqlx(err, Resource::Tag))?;
        Ok(tasks)
    }

    async fn create_tag_row(
        conn: &mut PgConnection,
        user_id: UserID,
        create_model: CreateModel,
    ) -> Result<TagModel> {
        let id = query_scalar(
            "INSERT INTO app.tags (id, label, category_id, position_key, created_by)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id",
        )
        .bind(TagID::new_random())
        .bind(create_model.label)
        .bind(create_model.category_id)
        .bind(create_model.position_key)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Tag))?;

        Ok(Self::fetch_tag_row(conn, id, user_id)
            .await?
            .expect("tag was just created"))
    }

    async fn fetch_tag_row(
        conn: &mut PgConnection,
        id: TagID,
        user_id: UserID,
    ) -> Result<Option<TagModel>> {
        let tag_opt = query_as::<TagModel>(
            "SELECT t.*, tc.name as category_name, tc.position_key as category_position_key FROM app.tags t
            LEFT JOIN app.categories tc ON t.category_id = tc.id
            WHERE t.id = $1 AND t.created_by = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(conn.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Tag))?;

        Ok(tag_opt)
    }

    async fn update_tag_row(
        conn: &mut PgConnection,
        id: TagID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TagModel> {
        let mut builder = QueryBuilder::new("UPDATE app.tags SET ");
        update_model.add_to_builder(&mut builder);
        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" AND created_by = ");
        builder.push_bind(user_id);
        builder.push(" RETURNING *");

        builder
            .build()
            .fetch_one(conn.as_mut())
            .await
            .map_err(|err| Error::from_sqlx(err, Resource::Tag))?;

        Ok(Self::fetch_tag_row(conn, id, user_id)
            .await
            .expect("should not error: tag was just updated")
            .expect("tag was just updated"))
    }

    async fn delete_tag_row(conn: &mut PgConnection, id: TagID, user_id: UserID) -> Result<()> {
        query(
            "DELETE FROM app.tags
            WHERE id = $1 AND created_by = $2
            RETURNING id",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(conn.as_mut())
        .await
        .map_err(|err| Error::from_sqlx(err, Resource::Tag))?;

        Ok(())
    }
}

#[async_trait]
impl TagRepository for PgTagRepository {
    async fn list(&self, user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<TagModel>> {
        let mut conn = self.db.acquire().await?;

        let tags = Self::fetch_all_tag_rows(&mut conn, user_id, query).await?;

        conn.close().await?;
        Ok(tags)
    }

    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<TagModel> {
        let mut tx = self.db.begin().await?;

        let tag = Self::create_tag_row(&mut tx, user_id, create_model).await?;

        tx.commit().await?;
        Ok(tag)
    }

    async fn get(&self, id: TagID, user_id: UserID) -> Result<Option<TagModel>> {
        let mut conn = self.db.acquire().await?;

        let tag_opt = Self::fetch_tag_row(&mut conn, id, user_id).await?;

        conn.close().await?;
        Ok(tag_opt)
    }

    async fn update(
        &self,
        id: TagID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TagModel> {
        let mut tx = self.db.begin().await?;

        let tag = Self::update_tag_row(&mut tx, id, user_id, update_model).await?;

        tx.commit().await?;
        Ok(tag)
    }

    async fn delete(&self, id: TagID, user_id: UserID) -> Result<()> {
        let mut tx = self.db.begin().await?;

        Self::delete_tag_row(&mut tx, id, user_id).await?;

        tx.commit().await?;
        Ok(())
    }
}
