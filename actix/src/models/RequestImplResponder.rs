use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestImplResponderObj<'a> {
    pub name: &'a str,
}

impl Responder for RequestImplResponderObj<'_> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    }
}


impl<'a> RequestImplResponderObj<'a> {
    // Implement the `from` function that takes a reference with lifetime `'a`
    fn from(strr: &'a str) -> Self {
        RequestImplResponderObj { name: strr }
    }
}
