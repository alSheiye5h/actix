use actix_web::{get, post, web, App, HttpServer};

use crate::models::QueryStruct;
use crate::models::QueryBodyStruct;

// #[get("/query")]
// pub async fn query_extract(query: web::Query<String>) -> String {
    
//     println!("query =>username: {}", query);
 
//     format!("query =>username: {}", query)
// }

// // #[post("/query")]
// pub async fn query_body_extract(info: web::Form<QueryStruct>, body: web::Json<QueryBodyStruct>) -> String {

//     println!("query => year: {}, method: {}", info.year, info.method);
//     println!("body => username: {}, age: {}, rank: {}", body.username, body.age, body.rank);

//     format!("query => year: {}, method: {}\nbody => username: {}, age: {}, rank: {}", info.year, info.method, body.username, body.age, body.rank)
// }

// pub async fn url_encoded(info: web::Form<QueryStruct>) -> String {
    
//     println!("query => year: {}, method: {}", info.year, info.method);
 
//     format!("query => year: {}, method: {}", info.year, info.method)
// }
