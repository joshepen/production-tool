use crate::{db::users, restapi::errors::sqlx_error_to_http};
use actix_web::{HttpResponse, Responder, delete, get, post, web};

#[derive(serde::Deserialize)]
pub struct UserQuery {
    pub id: i32,
}

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

#[get("/user")]
async fn get_user(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<UserQuery>,
) -> impl Responder {
    match users::get_user(&pool, query.id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/user")]
async fn delete_user(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<UserQuery>,
) -> impl Responder {
    match users::delete_user(&pool, query.id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/user")]
async fn post_user(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<users::NewUser>,
) -> impl Responder {
    match users::create_user(&pool, &query).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}
