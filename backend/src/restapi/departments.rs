use crate::db::departments;
use crate::restapi::errors::sqlx_error_to_http;
use actix_web::{HttpResponse, Responder, delete, get, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_departments);
    cfg.service(get_department);
    cfg.service(delete_department);
    cfg.service(post_department);
}

#[get("/departments")]
async fn get_departments(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    match departments::get_departments(&pool).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/department/{id}")]
async fn get_department(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match departments::get_department(&pool, *id).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/department/{id}")]
async fn delete_department(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match departments::delete_department(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/department")]
async fn post_department(
    pool: web::Data<sqlx::MySqlPool>,
    body: web::Json<departments::NewDepartment>,
) -> impl Responder {
    match departments::create_department(&pool, &body).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => sqlx_error_to_http(e),
    }
}
