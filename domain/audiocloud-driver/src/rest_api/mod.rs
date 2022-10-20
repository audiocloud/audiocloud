use actix_web::web;

mod v1;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("v1").configure(v1::configure));
}
