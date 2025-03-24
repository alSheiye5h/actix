
use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub json: MPJson<Metadata>
}

