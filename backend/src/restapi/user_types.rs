use crate::{db::user_types, restapi::errors::sqlx_error_to_http};
use actix_web::{HttpResponse, Responder, delete, get, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_types);
    cfg.service(get_user_type);
    cfg.service(delete_user_type);
    cfg.service(post_user_type);
}

#[get("/user_types")]
async fn get_user_types(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    match user_types::get_user_types(&pool).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/user_type/{id}")]
async fn get_user_type(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match user_types::get_user_type(&pool, *id).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/user_type/{id}")]
async fn delete_user_type(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match user_types::delete_user_type(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/user_type")]
async fn post_user_type(
    pool: web::Data<sqlx::MySqlPool>,
    body: web::Json<user_types::NewUserType>,
) -> impl Responder {
    match user_types::create_user_type(&pool, &body).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => sqlx_error_to_http(e),
    }
}