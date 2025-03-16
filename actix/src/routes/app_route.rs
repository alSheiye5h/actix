use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod crate::handlers::user;
use user::get_user

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
        .route(web::get().to(get_user))
    )
}