use actix_web::{get, web, App, HttpServer};

use crate::models::QueryStruct;
use crate::models::QueryBodyStruct;

#[get("/query")]
pub async fn query_extract(info: web::Query<QueryStruct>, body: web::Json<QueryBodyStruct>) -> String {
    
    println!("query => year: {}, localisation: {}", info.year, info.localisation);
    println!("body => username: {}, age: {}, rank: {}", body.username, body.age, body.rank)
 
    format!("query => year: {}, localisation: {}", info.year, info.localisation)
    format!("body => username: {}, age: {}, rank: {}", body.username, body.age, body.rank)
}