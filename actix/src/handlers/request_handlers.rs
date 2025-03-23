use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder, web
};

use crate::models::RequestImplResponderObj;

pub async fn return_static_str(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

pub async fn return_string(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}

pub async fn return_bytes(_req: HttpRequest) -> impl Responder {  // this downloads a file
    web::Bytes::from_static(b"Hello world!")  // this downloads a file
}

pub async fn get_RequestImplResponderObj(info: web::Query<RequestImplResponderObj>) -> impl Responder {
    // RequestImplResponderObj { name: "user" }
    info
}