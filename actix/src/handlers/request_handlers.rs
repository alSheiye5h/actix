use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder, web
};

pub async fn return_static_str(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

pub async fn return_string(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}

pub async fn return_bytes(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}
