use crate::db::product_orders;
use crate::restapi::errors::sqlx_error_to_http;
use actix_web::{HttpResponse, Responder, delete, get, post, web};

#[derive(serde::Deserialize)]
pub struct ProductOrderQuery {
    pub id: i32,
}

#[derive(serde::Deserialize)]
pub struct ProductOrderStatusQuery {
    pub status_id: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct ProductOrderStatusPost {
    pub id: i32,
    pub status_id: i32,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_product_orders);
    cfg.service(get_product_order);
    cfg.service(delete_product_order);
    cfg.service(post_product_order);
    cfg.service(set_po_status);
}

#[get("/product_orders")]
async fn get_product_orders(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<ProductOrderStatusQuery>,
) -> impl Responder {
    match query.status_id {
        Some(s) => match product_orders::get_product_orders_by_status(&pool, s).await {
            Ok(d) => HttpResponse::Ok().json(d),
            Err(e) => sqlx_error_to_http(e),
        },
        None => match product_orders::get_product_orders(&pool).await {
            Ok(d) => HttpResponse::Ok().json(d),
            Err(e) => sqlx_error_to_http(e),
        },
    }
}

#[get("/product_order")]
async fn get_product_order(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<ProductOrderQuery>,
) -> impl Responder {
    match product_orders::get_product_order(&pool, query.id).await {
        Ok(po) => HttpResponse::Ok().json(po),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/product_order")]
async fn delete_product_order(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<ProductOrderQuery>,
) -> impl Responder {
    match product_orders::delete_product_order(&pool, query.id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/product_order")]
async fn post_product_order(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<product_orders::NewProductOrder>,
) -> impl Responder {
    match product_orders::create_product_order(&pool, &query).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/product_order/status")]
async fn set_po_status(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<ProductOrderStatusPost>,
) -> impl Responder {
    match product_orders::set_po_status(&pool, query.id, query.status_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}
