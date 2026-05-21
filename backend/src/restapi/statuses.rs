use crate::{db::statuses, restapi::errors::sqlx_error_to_http};
use actix_web::{HttpResponse, Responder, get, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_status);
    cfg.service(get_statuses);
}

#[get("/statuses")]
async fn get_statuses(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    match statuses::get_statuses(&pool).await {
        Ok(statuses) => HttpResponse::Ok().json(statuses),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/status/{id}")]
async fn get_status(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match statuses::get_status(&pool, *id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => sqlx_error_to_http(e),
    }
}
