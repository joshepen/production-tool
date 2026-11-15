use serde;
use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewWorkCenter {
    pub name: String,
    pub description: Option<String>,
}

#[derive(serde::Serialize)]
pub struct WorkCenter {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

pub async fn get_work_centers(pool: &sqlx::MySqlPool) -> Result<Vec<WorkCenter>, sqlx::Error> {
    let work_centers = query_as!(WorkCenter, "SELECT id, name, description FROM work_centers")
        .fetch_all(pool)
        .await?;
    return Ok(work_centers);
}

pub async fn get_work_center(pool: &sqlx::MySqlPool, id: i32) -> Result<WorkCenter, sqlx::Error> {
    let work_center = query_as!(
        WorkCenter,
        "SELECT id, name, description FROM work_centers WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(work_center);
}

pub async fn create_work_center(
    pool: &sqlx::MySqlPool,
    wc: &NewWorkCenter,
) -> Result<u64, sqlx::Error> {
    let result = query!(
        "INSERT INTO work_centers (name, description) VALUES (?, ?)",
        wc.name,
        wc.description
    )
    .execute(pool)
    .await?;

    return Ok(result.last_insert_id());
}

pub async fn delete_work_center(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    let result = query!("DELETE FROM work_centers WHERE id = ?", id)
        .execute(pool)
        .await?;

    match result.rows_affected() {
        0 => Err(sqlx::Error::RowNotFound),
        _ => Ok(()),
    }
}