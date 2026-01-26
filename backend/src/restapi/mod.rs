use actix_web::web;

pub mod errors;
pub mod users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(users::configure);
}
