use crate::{db::statuses, restapi::errors::sqlx_error_to_http};
use actix_web::{HttpResponse, Responder, get, put, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_status);
    cfg.service(get_statuses);
    cfg.service(put_status);
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

#[put("/status/{id}")]
async fn put_status(
    pool: web::Data<sqlx::MySqlPool>,
    id: web::Path<i32>,
    body: web::Json<statuses::NewStatus>,
) -> impl Responder {
    match statuses::update_status(&pool, *id, &body.name).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}
