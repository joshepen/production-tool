use sqlx::query_as;

pub struct Department {
    pub id: i32,
    pub name: String,
}

pub async fn get_departments(
    conn: &mut sqlx::MySqlConnection,
) -> Result<Vec<Department>, sqlx::Error> {
    let departments = query_as!(Department, "SELECT id, name FROM departments")
        .fetch_all(conn)
        .await?;
    return Ok(departments);
}
