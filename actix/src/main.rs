use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod handlers;
mod models;
mod routes;
mod utils;

use handlers::{hello, echo, manual_hello, index, get_app_name, check_username};
use models::AppStateStruct;
use routes::{get_the_user, get_user_data};
use handlers::request_handlers::return_static_str;
use handlers::request_handlers::return_string;
use handlers::request_handlers::return_bytes;
use handlers::request_handlers::get_RequestImplResponderObj;



// use handlers::query_extract; // will trigger error
// use handlers::query_body_extract;   // will trigger error

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

    let mut builder = match SslAcceptor::mozilla_intermediate(SslMethod::tls()) {
        Ok(builder) => builder,
        Err(e) => {
            eprintln!("Error creating SSL builder: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "SSL setup failed"));
        }
    };

    // Set private key file
    if let Err(e) = builder.set_private_key_file("/home/al-shaye5h/Dev/rust/actix/configuration/key.pem", SslFiletype::PEM) {
        eprintln!("Error setting private key: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to set private key"));
    }

    // Set certificate chain file
    if let Err(e) = builder.set_certificate_chain_file("/home/al-shaye5h/Dev/rust/actix/configuration/cert.pem") {
        eprintln!("Error setting certificate chain: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to set certificate chain"));
    }

    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/app")
            .route("/index", web::get().to(index))
            .route("/info", web::get().to(get_app_name)),
        )
        .service(
            web::scope("/reqhandle")
            .route("/one", web::get().to(return_static_str))
            .route("/two", web::get().to(return_string))
            .route("/three", web::get().to(return_bytes)) // this downloads a file
            .route("/four", web::get().to(get_RequestImplResponderObj))
        )
        .service(check_username)
        // .service(query_extract)   // will trigger error
        // .service(query_body_extract)   // will trigger error
        .app_data(
            web::Data::new(
                AppStateStruct {
                    app_name: String::from("Actix_web"),
                }
            )
        )
        // .configure(get_the_user)
        // .configure(get_user_data)
    })
    .workers(1) // workers are system threads number to handle requests
    .bind("127.0.0.1:8080")?
    // .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}