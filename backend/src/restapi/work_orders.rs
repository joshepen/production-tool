use crate::db::work_orders;
use crate::restapi::errors::sqlx_error_to_http;
use actix_web::{HttpResponse, Responder, delete, get, post, web};

#[derive(serde::Deserialize)]
pub struct WorkOrdersQuery {
    pub product_order_id: i32,
}

#[derive(serde::Deserialize)]
pub struct WorkOrderStatusPost {
    pub status_id: i32,
}

#[derive(serde::Deserialize)]
pub struct WorkOrderAssigneePost {
    pub assignee_id: Option<i32>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_work_orders);
    cfg.service(get_pretty_work_orders);
    cfg.service(get_work_order);
    cfg.service(delete_work_order);
    cfg.service(post_work_order);
    cfg.service(update_work_order_status);
    cfg.service(start_work_order);
    cfg.service(complete_work_order);
    cfg.service(update_work_order_assignee);
}

#[get("/work_orders")]
async fn get_work_orders(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<WorkOrdersQuery>,
) -> impl Responder {
    match work_orders::get_work_orders(&pool, query.product_order_id).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/pretty_work_orders")]
async fn get_pretty_work_orders(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<WorkOrdersQuery>,
) -> impl Responder {
    match work_orders::get_pretty_work_orders(&pool, query.product_order_id).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[get("/work_order/{id}")]
async fn get_work_order(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match work_orders::get_work_order(&pool, *id).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[delete("/work_order/{id}")]
async fn delete_work_order(pool: web::Data<sqlx::MySqlPool>, id: web::Path<i32>) -> impl Responder {
    match work_orders::delete_work_order(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/work_order")]
async fn post_work_order(
    pool: web::Data<sqlx::MySqlPool>,
    body: web::Json<work_orders::NewWorkOrder>,
) -> impl Responder {
    match work_orders::create_work_order(&pool, &body).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/work_order/{id}/status")]
async fn update_work_order_status(
    pool: web::Data<sqlx::MySqlPool>,
    id: web::Path<i32>,
    body: web::Json<WorkOrderStatusPost>,
) -> impl Responder {
    match work_orders::update_work_order_status(&pool, *id, body.status_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/work_order/{id}/start")]
async fn start_work_order(
    pool: web::Data<sqlx::MySqlPool>,
    id: web::Path<i32>,
) -> impl Responder {
    match work_orders::start_work_order(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/work_order/{id}/complete")]
async fn complete_work_order(
    pool: web::Data<sqlx::MySqlPool>,
    id: web::Path<i32>,
) -> impl Responder {
    match work_orders::complete_work_order(&pool, *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}

#[post("/work_order/{id}/assignee")]
async fn update_work_order_assignee(
    pool: web::Data<sqlx::MySqlPool>,
    id: web::Path<i32>,
    body: web::Json<WorkOrderAssigneePost>,
) -> impl Responder {
    match work_orders::update_work_order_assignee(&pool, *id, body.assignee_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => sqlx_error_to_http(e),
    }
}