use crate::{db::products, restapi::errors::sqlx_error_to_http};
use actix_web::{HttpResponse, Responder, delete, get, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_products);
    cfg.service(get_product);
    cfg.service(delete_product);
    cfg.service(post_product);
}

#[get("/products")]
async fn get_products(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    match products::get_products(&pool).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/product/{id}")]
async fn get_product(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match products::get_product(&pool, *id).await {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/product/{id}")]
async fn delete_product(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match products::delete_product(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/product")]
async fn post_product(
    pool: web::Data<sqlx::MySqlPool>,
    body: web::Json<products::NewProduct>,
) -> impl Responder {
    match products::create_product(&pool, &body).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}
