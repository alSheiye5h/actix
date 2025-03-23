use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};



use crate::handlers::get_user;
use crate::handlers::user_data;

pub async fn get_the_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
        .route(web::get().to(get_user))
    );
}

pub async fn get_user_data(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{name}/{password}")
        .route(web::post().to(user_data))
    );
}