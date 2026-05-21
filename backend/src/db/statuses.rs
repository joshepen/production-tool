use serde;
use sqlx::query_as;

#[derive(serde::Serialize)]
pub struct Status {
    pub id: i32,
    pub name: String,
}

pub async fn get_status(pool: &sqlx::MySqlPool, id: i32) -> Result<Status, sqlx::Error> {
    let status = query_as!(
        Status,
        "SELECT s.id, s.name FROM statuses s WHERE s.id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(status);
}

pub async fn get_statuses(pool: &sqlx::MySqlPool) -> Result<Vec<Status>, sqlx::Error> {
    let users = query_as!(Status, "SELECT s.id, s.name FROM statuses s",)
        .fetch_all(pool)
        .await?;

    return Ok(users);
}
