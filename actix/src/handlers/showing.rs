use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::{self, AppStateStruct};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("you are in index")
}


pub async fn get_app_name() -> impl Responder {
    HttpResponse::Ok().body("Your App Name Here")
}