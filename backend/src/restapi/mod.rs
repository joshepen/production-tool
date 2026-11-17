use actix_web::web;

pub mod departments;
pub mod errors;
pub mod product_orders;
pub mod product_work_centers;
pub mod products;
pub mod statuses;
pub mod user_types;
pub mod users;
pub mod work_centers;
pub mod work_orders;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(users::configure);
    cfg.configure(departments::configure);
    cfg.configure(products::configure);
    cfg.configure(product_orders::configure);
    cfg.configure(statuses::configure);
    cfg.configure(work_centers::configure);
    cfg.configure(work_orders::configure);
    cfg.configure(product_work_centers::configure);
    cfg.configure(user_types::configure);
}
