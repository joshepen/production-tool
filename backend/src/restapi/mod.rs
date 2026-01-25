use actix_web::web;

pub mod users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(users::configure);
}
