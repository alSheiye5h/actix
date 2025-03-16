use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::UserStruct

pub async get_user() -> impl Responder {
    let body = "user is you"
    HttpResponse::ok().body(body)
}