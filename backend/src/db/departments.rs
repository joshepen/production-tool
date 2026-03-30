use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewDepartment {
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

pub async fn get_departments(pool: &sqlx::MySqlPool) -> Result<Vec<Department>, sqlx::Error> {
    let departments = query_as!(Department, "SELECT id, name FROM departments")
        .fetch_all(pool)
        .await?;
    return Ok(departments);
}

pub async fn get_department(pool: &sqlx::MySqlPool, id: i32) -> Result<Department, sqlx::Error> {
    let department = query_as!(
        Department,
        "SELECT d.id, d.name FROM departments d WHERE d.id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(department);
}

pub async fn create_department(
    pool: &sqlx::MySqlPool,
    d: &NewDepartment,
) -> Result<u64, sqlx::Error> {
    let result = query!("INSERT INTO departments (name) VALUES (?)", d.name,)
        .execute(pool)
        .await?;

    return Ok(result.last_insert_id());
}

pub async fn delete_department(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    query!("DELETE FROM departments WHERE id = ?", id)
        .execute(pool)
        .await?;
    return Ok(());
}
