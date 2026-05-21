use chrono::{self, DateTime, Utc};
use serde;
use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub hired_at: Option<DateTime<Utc>>,
    pub department_id: i32,
}

#[derive(serde::Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub hired_at: DateTime<Utc>,
    pub department_id: i32,
}

#[derive(serde::Serialize)]
pub struct PrettyUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub hired_at: DateTime<Utc>,
    pub department_id: i32,
    pub department_name: String,
}

pub async fn get_user(pool: &sqlx::MySqlPool, id: i32) -> Result<User, sqlx::Error> {
    let user = query_as!(
        User,
        "SELECT u.id, u.first_name, u.last_name, u.hired_at, u.department_id FROM users u WHERE u.id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(user);
}

pub async fn get_users(pool: &sqlx::MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        "SELECT u.id, u.first_name, u.last_name, u.hired_at, u.department_id FROM users u"
    )
    .fetch_all(pool)
    .await?;

    return Ok(users);
}

pub async fn get_pretty_users(pool: &sqlx::MySqlPool) -> Result<Vec<PrettyUser>, sqlx::Error> {
    let users = query_as!(
        PrettyUser,
        "SELECT u.id, u.first_name, u.last_name, u.hired_at, u.department_id, d.name as department_name FROM users u JOIN departments d ON u.department_id = d.id"
    )
    .fetch_all(pool)
    .await?;

    return Ok(users);
}

pub async fn create_user(pool: &sqlx::MySqlPool, u: &NewUser) -> Result<u64, sqlx::Error> {
    // I know this doesn't scale well but the query builder is so verbose that
    // I'd say it isn't worth it for one Option
    let q = match u.hired_at {
        Some(h) => query!(
            "INSERT INTO users (first_name, last_name, department_id, hired_at) VALUES (?, ?, ?, ?)",
            u.first_name,
            u.last_name,
            u.department_id,
            h
        ),
        None => query!(
            "INSERT INTO users (first_name, last_name, department_id) VALUES (?, ?, ?)",
            u.first_name,
            u.last_name,
            u.department_id,
        ),
    };
    let result = q.execute(pool).await?;

    return Ok(result.last_insert_id());
}

pub async fn delete_user(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    let result = query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await?;
    match result.rows_affected() {
        0 => Err(sqlx::Error::RowNotFound),
        _ => Ok(()),
    }
}
