use chrono::{DateTime, Utc};
use serde;
use sqlx::{query, query_as};

/* Removed TemplateStepForQuery – we no longer have template based workflow */
#[derive(serde::Deserialize)]
pub struct NewWorkOrder {
    pub product_order_id: i32,
    pub step_order: i32,
    pub status_id: Option<i32>,
}
#[derive(serde::Serialize)]
pub struct WorkOrder {
    pub id: i32,
    pub product_order_id: i32,
    pub work_center_id: i32,
    pub step_order: i32,
    pub status_id: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize)]
pub struct PrettyWorkOrder {
    pub id: i32,
    pub product_order_id: i32,
    pub product_order_address: String,
    pub work_center_id: i32,
    pub work_center_name: String,
    pub step_order: i32,
    pub status_id: i32,
    pub status_name: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
}

pub async fn get_work_orders(
    pool: &sqlx::MySqlPool,
    product_order_id: i32,
) -> Result<Vec<WorkOrder>, sqlx::Error> {
    let work_orders = query_as!(
        WorkOrder,
        "SELECT wo.id, wo.product_order_id, pwc.work_center_id, wo.step_order, wo.status_id, wo.started_at, wo.completed_at
         FROM work_orders wo
         JOIN product_orders po ON wo.product_order_id = po.id
         JOIN product_work_centers pwc ON po.product_id = pwc.product_id AND wo.step_order = pwc.step_order
         WHERE wo.product_order_id = ? ORDER BY wo.step_order",
        product_order_id
    )
    .fetch_all(pool)
    .await?;
    return Ok(work_orders);
}

pub async fn get_pretty_work_orders(
    pool: &sqlx::MySqlPool,
    product_order_id: i32,
) -> Result<Vec<PrettyWorkOrder>, sqlx::Error> {
    let work_orders = query_as!(
        PrettyWorkOrder,
        "SELECT wo.id,
                wo.product_order_id,
                po.address as product_order_address,
                pwc.work_center_id,
                wc.name as work_center_name,
                wo.step_order,
                wo.status_id,
                s.name as status_name,
                wo.started_at,
                wo.completed_at,
                wo.estimated_hours
         FROM work_orders wo
         JOIN product_orders po ON wo.product_order_id = po.id
         JOIN product_work_centers pwc ON po.product_id = pwc.product_id AND wo.step_order = pwc.step_order
         JOIN work_centers wc ON pwc.work_center_id = wc.id
         JOIN statuses s ON wo.status_id = s.id
         WHERE wo.product_order_id = ?
         ORDER BY wo.step_order",
        product_order_id
    )
    .fetch_all(pool)
    .await?;
    return Ok(work_orders);
}

pub async fn get_work_order(pool: &sqlx::MySqlPool, id: i32) -> Result<WorkOrder, sqlx::Error> {
    let work_order = query_as!(
        WorkOrder,
        "SELECT wo.id, wo.product_order_id, pwc.work_center_id, wo.step_order, wo.status_id, wo.started_at, wo.completed_at
         FROM work_orders wo
         JOIN product_orders po ON wo.product_order_id = po.id
         JOIN product_work_centers pwc ON po.product_id = pwc.product_id AND wo.step_order = pwc.step_order
         WHERE wo.id = ?",
        id
    )
    .fetch_one(pool)
    .await?;
    return Ok(work_order);
}

pub async fn create_work_order(
    pool: &sqlx::MySqlPool,
    wo: &NewWorkOrder,
) -> Result<u64, sqlx::Error> {
    let status_id = wo.status_id.unwrap_or(1);
    let result = query!(
        "INSERT INTO work_orders (product_order_id, step_order, status_id) VALUES (?, ?, ?)",
        wo.product_order_id,
        wo.step_order,
        status_id
    )
    .execute(pool)
    .await?;

    return Ok(result.last_insert_id());
}

pub async fn update_work_order_status(
    pool: &sqlx::MySqlPool,
    id: i32,
    status_id: i32,
) -> Result<(), sqlx::Error> {
    query!(
        "UPDATE work_orders SET status_id = ? WHERE id = ?",
        status_id,
        id
    )
    .execute(pool)
    .await?;
    return Ok(());
}

pub async fn start_work_order(
    pool: &sqlx::MySqlPool,
    id: i32,
) -> Result<(), sqlx::Error> {
    query!(
        "UPDATE work_orders SET status_id = 4, started_at = CURRENT_TIMESTAMP WHERE id = ?",
        id
    )
    .execute(pool)
    .await?;
    return Ok(());
}

pub async fn complete_work_order(
    pool: &sqlx::MySqlPool,
    id: i32,
) -> Result<(), sqlx::Error> {
    // Ensure predecessor (same product order, previous step_order) is completed
    let current = query!(
        "SELECT product_order_id, step_order FROM work_orders WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await?;

    if current.step_order > 1 {
        let prev = query!(
            "SELECT status_id FROM work_orders WHERE product_order_id = ? AND step_order = ?",
            current.product_order_id,
            current.step_order - 1
        )
        .fetch_one(pool)
        .await?;

        if prev.status_id != 5 {
            return Err(sqlx::Error::Protocol(
                "cannot complete work order before predecessor is completed".to_string(),
            ));
        }
    }

    query!(
        "UPDATE work_orders SET status_id = 5, completed_at = CURRENT_TIMESTAMP WHERE id = ?",
        id
    )
    .execute(pool)
    .await?;

    // When every work order for the product order is completed, mark the
    // product order itself as completed.
    let remaining = query!(
        "SELECT COUNT(*) AS cnt FROM work_orders WHERE product_order_id = ? AND status_id != 5",
        current.product_order_id
    )
    .fetch_one(pool)
    .await?;

    if remaining.cnt == 0 {
        query!(
            "UPDATE product_orders SET status_id = 5 WHERE id = ?",
            current.product_order_id
        )
        .execute(pool)
        .await?;
    }

    return Ok(());
}

pub async fn delete_work_order(pool: &sqlx::MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    let result = query!("DELETE FROM work_orders WHERE id = ?", id)
        .execute(pool)
        .await?;

    match result.rows_affected() {
        0 => Err(sqlx::Error::RowNotFound),
        _ => Ok(()),
    }
}