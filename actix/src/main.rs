use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod handlers;
mod models;

use handlers::{hello, echo, manual_hello, index, get_app_name};
use models::AppStateStruct;


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/app")
            .route("/index", web::get().to(index))
            .route("/info", web::get().to(get_app_name)),
        )
        .app_data(
            web::Data::new(
                AppStateStruct {
                    app_name: String::from("Actix_web"),
                }
            )
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}