use sqlx::query_as;
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    hired_at: sqlx::types::time::Date,
    department: String,
}

// pub fn get_user(first_name: String, last_name: String) -> User {}

pub async fn get_users(conn: &mut sqlx::MySqlConnection) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(
        User,
        "SELECT u.id, u.first_name, u.last_name, u.hired_at, d.name as department FROM users u JOIN departments d ON d.id = u.department_id"
    )
    .fetch_all(conn)
    .await?;

    return Ok(users);
}
