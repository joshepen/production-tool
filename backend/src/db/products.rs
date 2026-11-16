use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub work_center_ids: Vec<i32>,
}

#[derive(serde::Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
}

pub async fn get_product(pool: &sqlx::MySqlPool, id: i32) -> Result<Product, sqlx::Error> {
    let product = query_as!(
        Product,
        "SELECT p.id, p.name FROM products p WHERE p.id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(product);
}

pub async fn get_products(pool: &sqlx::MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
    let products = query_as!(Product, "SELECT p.id, p.name FROM products p")
        .fetch_all(pool)
        .await?;

    return Ok(products);
}

pub async fn create_product(pool: &sqlx::MySqlPool, p: &NewProduct) -> Result<u64, sqlx::Error> {
    let mut conn = pool.begin().await?;

    let result = query!("INSERT INTO products (name) VALUES (?)", p.name,)
        .execute(&mut *conn)
        .await?;

    let product_id = result.last_insert_id();

    // Insert work center steps (recipe)
    for (index, work_center_id) in p.work_center_ids.iter().enumerate() {
        let step_order = (index + 1) as i32;
        query!(
            "INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (?, ?, ?)",
            product_id,
            step_order,
            work_center_id
        )
        .execute(&mut *conn)
        .await?;
    }

    conn.commit().await?;

    return Ok(product_id);
}

pub async fn delete_product(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    let result = query!("DELETE FROM products WHERE id = ?", id)
        .execute(pool)
        .await?;

    match result.rows_affected() {
        0 => Err(sqlx::Error::RowNotFound),
        _ => Ok(()),
    }
}
