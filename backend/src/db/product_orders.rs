use chrono::{DateTime, Utc};
use sqlx::{query, query_as};

pub struct NewProductOrder {
    pub address: String,
    pub product_id: i32,
}

pub struct ProductOrder {
    pub id: i32,
    pub address: String,
    pub product_id: i32,
    pub status_id: i32,
    pub created_at: DateTime<Utc>,
}

pub async fn get_product_order(
    conn: &mut sqlx::MySqlConnection,
    id: i32,
) -> Result<ProductOrder, sqlx::Error> {
    let product_order = query_as!(
        ProductOrder,
        "SELECT o.id, o.address, o.product_id, o.status_id, o.created_at FROM product_orders o WHERE o.id = ?",
        id
    )
    .fetch_one(conn)
    .await?;
    return Ok(product_order);
}

pub async fn get_product_orders(
    conn: &mut sqlx::MySqlConnection,
) -> Result<Vec<ProductOrder>, sqlx::Error> {
    let product_orders = query_as!(
        ProductOrder,
        "SELECT o.id, o.address, o.product_id, o.status_id, o.created_at FROM product_orders o ORDER BY o.status_id ASC"
    )
    .fetch_all(conn)
    .await?;
    return Ok(product_orders);
}

pub async fn get_product_orders_by_status(
    conn: &mut sqlx::MySqlConnection,
    status_id: i32,
) -> Result<Vec<ProductOrder>, sqlx::Error> {
    let product_orders = query_as!(
        ProductOrder,
        "SELECT o.id, o.address, o.product_id, o.status_id, o.created_at FROM product_orders o WHERE o.status_id = ?", status_id
    )
    .fetch_all(conn)
    .await?;
    return Ok(product_orders);
}

pub async fn create_product_order(
    conn: &mut sqlx::MySqlConnection,
    po: &NewProductOrder,
) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO product_orders (address, product_id, status_id) VALUES (?, ?, 1)",
        po.address,
        po.product_id
    )
    .execute(conn)
    .await?;
    return Ok(());
}

pub async fn delete_product_order(
    conn: &mut sqlx::MySqlConnection,
    id: i32,
) -> Result<(), sqlx::Error> {
    query!("DELETE FROM product_orders WHERE id = ?", id)
        .execute(conn)
        .await?;
    return Ok(());
}
