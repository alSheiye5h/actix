use actix_web::{get, web, HttpResponse, Responder};
use std::error::Error;
use crate::utils::check_regex_username;


#[get("/users/{username}")]
pub async fn check_username(path: web::Path<String>) -> Result<HttpResponse, Box<dyn Error>> {
    let username = path.into_inner();
    println!("{}", username);
    
    match check_regex_username(username.clone()).await {
        Ok(true) => Ok(HttpResponse::Ok().body(format!("{} is a good username", username))),
        Ok(false) => Ok(HttpResponse::BadRequest().body(format!("{} is a bad username", username))),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Internal error: {}", e))),
    }
}