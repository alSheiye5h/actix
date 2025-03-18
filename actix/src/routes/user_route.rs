use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};



use crate::handlers::user::get_user;
use crate::handlers::data::user_data;

pub fn get_the_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
        .route(web::get().to(get_user))
    );
}

pub fn get_user_data(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{name}/{password}")
        .route(web::post().to(user_data))
    );
}