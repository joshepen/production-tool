use crate::{db::product_work_centers, restapi::errors::sqlx_error_to_http};
use actix_web::{HttpResponse, Responder, delete, get, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_product_work_centers);
    cfg.service(post_product_work_center);
    cfg.service(delete_product_work_center);
    cfg.service(delete_all_product_work_centers);
}

#[get("/product_work_centers/{product_id}")]
async fn get_product_work_centers(
    pool: web::Data<sqlx::MySqlPool>,
    product_id: web::Path<i32>,
) -> impl Responder {
    match product_work_centers::get_product_work_centers(&pool, *product_id).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/product_work_center")]
async fn post_product_work_center(
    pool: web::Data<sqlx::MySqlPool>,
    body: web::Json<product_work_centers::NewProductWorkCenter>,
) -> impl Responder {
    match product_work_centers::create_product_work_center(&pool, &body).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/product_work_center/{product_id}/{step_order}")]
async fn delete_product_work_center(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (product_id, step_order) = path.into_inner();
    match product_work_centers::delete_product_work_center(&pool, product_id, step_order).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/product_work_centers/{product_id}")]
async fn delete_all_product_work_centers(
    pool: web::Data<sqlx::MySqlPool>,
    product_id: web::Path<i32>,
) -> impl Responder {
    match product_work_centers::delete_all_for_product(&pool, *product_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}