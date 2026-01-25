use chrono::{self, DateTime, Utc};
use serde;
use sqlx::{query, query_as};

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

pub async fn create_user(pool: &sqlx::MySqlPool, u: &NewUser) -> Result<(), sqlx::Error> {
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
    q.execute(pool).await?;

    return Ok(());
}

pub async fn delete_user(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await?;
    return Ok(());
}
