use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


// use crate::handlers::{query_extract, query_body_extract, url_encoded};
// use crate::models::QueryStruct;
// use crate::models::QueryBodyStruct;

// pub async fn info_getter(cfg: &mut web::ServiceConfig) {

//     cfg.service(
//         web::resource("/query")
//         .route(web::post().to(post_handle))
//         .route(web::get().to(get_handle))
//     );
// }


// async fn post_handle(info: web::Form<QueryStruct>, body: web::Json<QueryBodyStruct>, query: web::Query<String>) -> String {
//     let query_response = query_extract(query).await;
//     let info_response = url_encoded(info).await;
//     let body_response = query_body_extract(info, body).await;

//     format!(
//         "{}\n{} \n{}",
//         query_response,
//         info_response,
//         body_response
//     )

// }

// async fn get_handle(query: web::Query<String>, info: web::Form<QueryStruct>) -> String {
    
//     let query_response = query_extract(query).await;
//     let info_response = url_encoded(info).await;

//     format!(
//         "{}\n{} \n",
//         query_response,
//         info_response,
//     )
// }

// pub async fn query_extract(query: web::Query<String>) -> String {
    
//     println!("query =>username: {}", query.username);
 
//     format!("query =>username: {}", query.username)
// }

// #[post("/query")]
// pub async fn query_body_extract(info: web::Query<QueryStruct>, body: web::Json<QueryBodyStruct>) -> String {

//     println!("query => year: {}, method: {}", info.year, info.method);
//     println!("body => username: {}, age: {}, rank: {}", body.username, body.age, body.rank);

//     format!("query => year: {}, method: {}\nbody => username: {}, age: {}, rank: {}", info.year, info.method, body.username, body.age, body.rank)
// }

// pub async fn url_encoded(info: web::Form<QueryStruct>) -> String {
    
//     println!("query => year: {}, method: {}", info.year, info.method);
 
//     format!("query => year: {}, method: {}", info.year, info.method)
// }
