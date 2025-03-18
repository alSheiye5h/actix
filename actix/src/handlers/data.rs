use actix_web::{get, web, App, HttpServer, Responder};


use crate::models::DataStruct;

pub async fn user_data(path: web::Path<(String, String)>, json: web::Json<DataStruct>) -> impl Responder {
    let path = path.into_inner();
    let data = json.into_inner(); 

    println!("{} {} {} {}", path.0, path.1, data.name, data.password);
    format!("{} {} {} {}", path.0, path.1, data.name, data.password)
}