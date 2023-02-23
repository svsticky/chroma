use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;

pub struct Empty;

impl Responder for Empty {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().finish()
    }
}