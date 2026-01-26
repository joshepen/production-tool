use chrono::{DateTime, Utc};
use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewProductOrder {
    pub address: String,
    pub product_id: i32,
}

#[derive(serde::Serialize)]
pub struct ProductOrder {
    pub id: i32,
    pub address: String,
    pub product_id: i32,
    pub status_id: i32,
    pub created_at: DateTime<Utc>,
}

pub async fn get_product_order(
    pool: &sqlx::MySqlPool,
    id: i32,
) -> Result<ProductOrder, sqlx::Error> {
    let product_order = query_as!(
        ProductOrder,
        "SELECT o.id, o.address, o.product_id, o.status_id, o.created_at FROM product_orders o WHERE o.id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(product_order);
}

pub async fn get_product_orders(pool: &sqlx::MySqlPool) -> Result<Vec<ProductOrder>, sqlx::Error> {
    let product_orders = query_as!(
        ProductOrder,
        "SELECT o.id, o.address, o.product_id, o.status_id, o.created_at FROM product_orders o ORDER BY o.status_id ASC"
    )
    .fetch_all(pool)
    .await?;
    return Ok(product_orders);
}

pub async fn get_product_orders_by_status(
    pool: &sqlx::MySqlPool,
    status_id: i32,
) -> Result<Vec<ProductOrder>, sqlx::Error> {
    let product_orders = query_as!(
        ProductOrder,
        "SELECT o.id, o.address, o.product_id, o.status_id, o.created_at FROM product_orders o WHERE o.status_id = ?", status_id
    )
    .fetch_all(pool)
    .await?;
    return Ok(product_orders);
}

pub async fn create_product_order(
    pool: &sqlx::MySqlPool,
    po: &NewProductOrder,
) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO product_orders (address, product_id, status_id) VALUES (?, ?, 1)",
        po.address,
        po.product_id
    )
    .execute(pool)
    .await?;
    return Ok(());
}

pub async fn delete_product_order(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    query!("DELETE FROM product_orders WHERE id = ?", id)
        .execute(pool)
        .await?;
    return Ok(());
}

pub async fn set_po_status(
    pool: &sqlx::MySqlPool,
    id: i32,
    status_id: i32,
) -> Result<(), sqlx::Error> {
    query!(
        "UPDATE product_orders SET status_id = ? WHERE id = ?",
        status_id,
        id
    )
    .execute(pool)
    .await?;
    return Ok(());
}
