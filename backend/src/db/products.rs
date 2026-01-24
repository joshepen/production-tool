use sqlx::{query, query_as};

pub struct NewProduct {
    pub name: String,
}

pub struct Product {
    pub id: i32,
    pub name: String,
}

pub async fn get_product(
    conn: &mut sqlx::MySqlConnection,
    id: i32,
) -> Result<Product, sqlx::Error> {
    let product = query_as!(
        Product,
        "SELECT p.id, p.name FROM products p WHERE p.id = ?",
        id
    )
    .fetch_one(conn)
    .await?;
    return Ok(product);
}

pub async fn get_products(conn: &mut sqlx::MySqlConnection) -> Result<Vec<Product>, sqlx::Error> {
    let products = query_as!(Product, "SELECT p.id, p.name FROM products p")
        .fetch_all(conn)
        .await?;

    return Ok(products);
}

pub async fn create_product(
    conn: &mut sqlx::MySqlConnection,
    p: &NewProduct,
) -> Result<(), sqlx::Error> {
    query!("INSERT INTO products (name) VALUES (?)", p.name,)
        .execute(conn)
        .await?;

    return Ok(());
}

pub async fn delete_product(conn: &mut sqlx::MySqlConnection, id: i32) -> Result<(), sqlx::Error> {
    query!("DELETE FROM products WHERE id = ?", id)
        .execute(conn)
        .await?;
    return Ok(());
}
