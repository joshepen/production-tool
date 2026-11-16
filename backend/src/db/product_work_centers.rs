use sqlx::{query, query_as};

#[derive(serde::Deserialize)]
pub struct NewProductWorkCenter {
    pub product_id: i32,
    pub step_order: i32,
    pub work_center_id: i32,
}

#[derive(serde::Serialize)]
pub struct PrettyProductWorkCenter {
    pub product_id: i32,
    pub step_order: i32,
    pub work_center_id: i32,
    pub work_center_name: String,
}

pub async fn get_product_work_centers(
    pool: &sqlx::MySqlPool,
    product_id: i32,
) -> Result<Vec<PrettyProductWorkCenter>, sqlx::Error> {
    let pws = query_as!(
        PrettyProductWorkCenter,
        "SELECT pwc.product_id, pwc.step_order, pwc.work_center_id, wc.name as work_center_name
         FROM product_work_centers pwc
         JOIN work_centers wc ON pwc.work_center_id = wc.id
         WHERE pwc.product_id = ?
         ORDER BY pwc.step_order",
        product_id
    )
    .fetch_all(pool)
    .await?;
    return Ok(pws);
}

pub async fn create_product_work_center(
    pool: &sqlx::MySqlPool,
    pwc: &NewProductWorkCenter,
) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (?, ?, ?)",
        pwc.product_id,
        pwc.step_order,
        pwc.work_center_id
    )
    .execute(pool)
    .await?;
    return Ok(());
}

pub async fn delete_product_work_center(
    pool: &sqlx::MySqlPool,
    product_id: i32,
    step_order: i32,
) -> Result<(), sqlx::Error> {
    let result = query!(
        "DELETE FROM product_work_centers WHERE product_id = ? AND step_order = ?",
        product_id,
        step_order
    )
    .execute(pool)
    .await?;

    match result.rows_affected() {
        0 => Err(sqlx::Error::RowNotFound),
        _ => Ok(()),
    }
}

pub async fn delete_all_for_product(
    pool: &sqlx::MySqlPool,
    product_id: i32,
) -> Result<(), sqlx::Error> {
    query!("DELETE FROM product_work_centers WHERE product_id = ?", product_id)
        .execute(pool)
        .await?;
    return Ok(());
}