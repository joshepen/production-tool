use crate::db::users;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use serde;

#[derive(serde::Deserialize)]
struct GetUserQuery {
    id: i32,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
}

#[get("/user")]
async fn get_user(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<GetUserQuery>,
) -> impl Responder {
    match users::get_user(&pool, query.id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}
