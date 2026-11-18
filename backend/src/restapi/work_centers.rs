use crate::db::work_centers;
use crate::restapi::errors::sqlx_error_to_http;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_work_centers);
    cfg.service(get_work_center);
    cfg.service(delete_work_center);
    cfg.service(post_work_center);
    cfg.service(put_work_center);
}

#[get("/work_centers")]
async fn get_work_centers(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    match work_centers::get_work_centers(&pool).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/work_center/{id}")]
async fn get_work_center(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match work_centers::get_work_center(&pool, *id).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/work_center/{id}")]
async fn delete_work_center(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match work_centers::delete_work_center(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/work_center")]
async fn post_work_center(
    pool: web::Data<sqlx::MySqlPool>,
    body: web::Json<work_centers::NewWorkCenter>,
) -> impl Responder {
    match work_centers::create_work_center(&pool, &body).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[put("/work_center/{id}")]
async fn put_work_center(
    pool: web::Data<sqlx::MySqlPool>,
    id: web::Path<i32>,
    body: web::Json<work_centers::NewWorkCenter>,
) -> impl Responder {
    match work_centers::update_work_center(&pool, *id, &body).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}