use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewUserType {
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct UserType {
    pub id: i32,
    pub name: String,
}

pub async fn get_user_types(pool: &sqlx::MySqlPool) -> Result<Vec<UserType>, sqlx::Error> {
    let user_types = query_as!(UserType, "SELECT id, name FROM user_types")
        .fetch_all(pool)
        .await?;
    return Ok(user_types);
}

pub async fn get_user_type(pool: &sqlx::MySqlPool, id: i32) -> Result<UserType, sqlx::Error> {
    let user_type = query_as!(
        UserType,
        "SELECT id, name FROM user_types WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(user_type);
}

pub async fn create_user_type(pool: &sqlx::MySqlPool, ut: &NewUserType) -> Result<u64, sqlx::Error> {
    let result = query!("INSERT INTO user_types (name) VALUES (?)", ut.name)
        .execute(pool)
        .await?;

    return Ok(result.last_insert_id());
}

pub async fn delete_user_type(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    let result = query!("DELETE FROM user_types WHERE id = ?", id)
        .execute(pool)
        .await?;

    match result.rows_affected() {
        0 => Err(sqlx::Error::RowNotFound),
        _ => Ok(()),
    }
}

pub async fn update_user_type(pool: &sqlx::MySqlPool, id: i32, name: &str) -> Result<(), sqlx::Error> {
    let result = query!("UPDATE user_types SET name = ? WHERE id = ?", name, id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(())
    }
}