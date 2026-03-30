use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewProduct {
    pub name: String,
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
    let result = query!("INSERT INTO products (name) VALUES (?)", p.name,)
        .execute(pool)
        .await?;

    return Ok(result.last_insert_id());
}

pub async fn delete_product(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    query!("DELETE FROM products WHERE id = ?", id)
        .execute(pool)
        .await?;
    return Ok(());
}
