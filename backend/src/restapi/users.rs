use crate::{db::users, restapi::errors::sqlx_error_to_http};
use actix_web::{HttpResponse, Responder, delete, get, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(delete_user);
    cfg.service(get_users);
    cfg.service(post_user);
}

#[get("/users")]
async fn get_users(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    match users::get_users(&pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/user/{id}")]
async fn get_user(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match users::get_user(&pool, *id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/user/{id}")]
async fn delete_user(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match users::delete_user(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/user")]
async fn post_user(
    pool: web::Data<sqlx::MySqlPool>,
    body: web::Json<users::NewUser>,
) -> impl Responder {
    match users::create_user(&pool, &body).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => sqlx_error_to_http(e),
    }
}
