use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::StatusCode;

pub struct Redirect(String);

impl Redirect {
    pub const STATUS_CODE: StatusCode = StatusCode::TEMPORARY_REDIRECT;

    pub fn new(to: String) -> Self {
        Self(to)
    }
}

impl Responder for Redirect {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(Redirect::STATUS_CODE)
            .insert_header(("Location", self.0))
            .finish()
    }
}
