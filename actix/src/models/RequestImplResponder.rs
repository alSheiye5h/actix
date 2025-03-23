use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestImplResponderObj {
    pub name: &'static str,
}

impl Responder for RequestImplResponderObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    }
}
