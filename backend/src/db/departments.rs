use sqlx::{query, query_as};

pub struct NewDepartment {
    pub name: String,
}

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

pub async fn get_department(
    conn: &mut sqlx::MySqlConnection,
    id: i32,
) -> Result<Department, sqlx::Error> {
    let department = query_as!(
        Department,
        "SELECT d.id, d.name FROM departments d WHERE d.id = ?",
        id
    )
    .fetch_one(conn)
    .await?;
    return Ok(department);
}

pub async fn create_department(
    conn: &mut sqlx::MySqlConnection,
    d: &NewDepartment,
) -> Result<(), sqlx::Error> {
    query!("INSERT INTO departments (name) VALUES (?)", d.name,)
        .execute(conn)
        .await?;

    return Ok(());
}

pub async fn delete_department(
    conn: &mut sqlx::MySqlConnection,
    id: i32,
) -> Result<(), sqlx::Error> {
    query!("DELETE FROM departments WHERE id = ?", id)
        .execute(conn)
        .await?;
    return Ok(());
}
