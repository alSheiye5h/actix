use actix_web::{post, App, HttpServer, Responder};
use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};

use crate::models::UploadForm;

#[post("/videos")]
pub async fn post_video(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    println!(
        "uploaded file {}, with size {}",
        form.json.name, form.file.size
    );
   
    format!(
        "uploaded file {}, with size {}",
        form.json.name, form.file.size
    )
}