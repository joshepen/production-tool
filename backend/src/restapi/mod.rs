use actix_web::web;

pub mod departments;
pub mod errors;
pub mod product_orders;
pub mod products;
pub mod users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(users::configure);
    cfg.configure(departments::configure);
    cfg.configure(products::configure);
    cfg.configure(product_orders::configure);
}
