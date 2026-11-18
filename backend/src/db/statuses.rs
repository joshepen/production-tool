use serde;
use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewStatus {
    pub name: String,
}

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

pub async fn update_status(pool: &sqlx::MySqlPool, id: i32, name: &str) -> Result<(), sqlx::Error> {
    let result = query!("UPDATE statuses SET name = ? WHERE id = ?", name, id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(())
    }
}
