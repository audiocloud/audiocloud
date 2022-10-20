use actix_web::web;

mod streaming;
mod tasks;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/streams").configure(streaming::configure))
        .service(web::scope("/tasks").configure(tasks::configure));
}
