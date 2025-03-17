use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::UserStruct;

pub async fn get_user() -> impl Responder {
    let bodyResponse: UserStruct = UserStruct(
        name: String::from("anas"),
        age: 21
    )
    HttpResponse::ok().body(bodyResponse)
}