use chrono;
use chrono::NaiveDate;
use sqlx::{query, query_as};
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub hired_at: NaiveDate,
    pub department: String,
}

pub async fn get_user(conn: &mut sqlx::MySqlConnection, id: i32) -> Result<User, sqlx::Error> {
    let user = query_as!(
        User,
        "SELECT u.id, u.first_name, u.last_name, u.hired_at, d.name as department FROM users u JOIN departments d ON d.id = u.department_id WHERE u.id = ?",
        id
    ).fetch_one(conn).await?;
    return Ok(user);
}

pub async fn get_users(conn: &mut sqlx::MySqlConnection) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        "SELECT u.id, u.first_name, u.last_name, u.hired_at, d.name as department FROM users u JOIN departments d ON d.id = u.department_id"
    )
    .fetch_all(conn)
    .await?;

    return Ok(users);
}

pub async fn create_user(
    conn: &mut sqlx::MySqlConnection,
    first_name: String,
    last_name: String,
    department_id: i32,
    hired_at: Option<NaiveDate>,
) -> Result<(), sqlx::Error> {
    let hired: NaiveDate = match hired_at {
        Some(h) => h,
        None => chrono::Utc::now().date_naive(),
    };
    query!(
        "INSERT INTO users (first_name, last_name, department_id, hired_at) VALUES (?, ?, ?, ?)",
        first_name,
        last_name,
        department_id,
        hired
    )
    .execute(conn)
    .await?;
    return Ok(());
}
