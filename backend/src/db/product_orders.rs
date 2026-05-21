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

#[derive(serde::Serialize)]
pub struct PrettyProductOrder {
    pub id: i32,
    pub address: String,
    pub product_id: i32,
    pub product_name: String,
    pub status_id: i32,
    pub status_name: String,
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

pub async fn get_pretty_product_orders(
    pool: &sqlx::MySqlPool,
) -> Result<Vec<PrettyProductOrder>, sqlx::Error> {
    let product_orders = query_as!(
        PrettyProductOrder,
        "SELECT o.id, o.address, o.product_id, p.name as product_name, o.status_id, s.name as status_name, o.created_at FROM product_orders o JOIN products p on o.product_id = p.id JOIN statuses s ON o.status_id = s.id ORDER BY o.status_id ASC"
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

pub async fn get_pretty_product_orders_by_status(
    pool: &sqlx::MySqlPool,
    status_id: i32,
) -> Result<Vec<PrettyProductOrder>, sqlx::Error> {
    let product_orders = query_as!(
        PrettyProductOrder,
        "SELECT o.id, o.address, o.product_id, p.name as product_name, o.status_id, s.name as status_name, o.created_at FROM product_orders o JOIN products p on o.product_id = p.id JOIN statuses s ON o.status_id = s.id WHERE o.status_id = ? ORDER BY o.status_id ASC", status_id
    )
    .fetch_all(pool)
    .await?;
    return Ok(product_orders);
}

pub async fn create_product_order(
    pool: &sqlx::MySqlPool,
    po: &NewProductOrder,
) -> Result<u64, sqlx::Error> {
    let result = query!(
        "INSERT INTO product_orders (address, product_id, status_id) VALUES (?, ?, 1)",
        po.address,
        po.product_id
    )
    .execute(pool)
    .await?;
    return Ok(result.last_insert_id());
}

pub async fn delete_product_order(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    let result = query!("DELETE FROM product_orders WHERE id = ?", id)
        .execute(pool)
        .await?;

    match result.rows_affected() {
        0 => Err(sqlx::Error::RowNotFound),
        _ => Ok(()),
    }
}

pub async fn set_po_status(
    pool: &sqlx::MySqlPool,
    id: i32,
    status_id: i32,
) -> Result<(), sqlx::Error> {
    println!("STATUS ID {}", status_id);
    query!(
        "UPDATE product_orders SET status_id = ? WHERE id = ?",
        status_id,
        id
    )
    .execute(pool)
    .await?;
    return Ok(());
}
