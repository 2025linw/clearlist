use sqlx::{
    FromRow, PgConnection, Postgres,
    postgres::{PgArguments, PgRow},
    query,
};

use crate::{error::repo::Result, task::types::TaskID, user::types::UserID};

/// Wrapper around `sqlx::query_as` which assumes Postgres as database
pub fn query_as<'q, T>(sql: &'q str) -> sqlx::query::QueryAs<'q, Postgres, T, PgArguments>
where
    T: for<'r> FromRow<'r, PgRow>,
{
    sqlx::query_as(sql)
}

pub async fn set_updated_timestamp(
    conn: &mut PgConnection,
    id: TaskID,
    user_id: UserID,
) -> Result<()> {
    query(
        "UPDATE app.tasks SET
        updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND created_by = $2",
    )
    .bind(id)
    .bind(user_id)
    .execute(conn.as_mut())
    .await?;

    Ok(())
}
