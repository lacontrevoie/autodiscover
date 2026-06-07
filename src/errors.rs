use std::fmt;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::http::header::ContentType;
use actix_web::{error, http::StatusCode};

#[derive(Debug)]
pub enum ServerError {
    TemplateRenderFailAutoconf,
    TemplateRenderFailAdJson,
    TemplateRenderFailAdXmlGet,
    TemplateRenderFailAdXmlPost,
    TemplateRenderFailMbc,
    DecodeXmlFail,
    ReadXmlFail,
    IllegalCharsDetected
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        eprintln!("Encountered {self}");
        
        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::html())
            .body("The server encountered an unexpected error. Please check your inputs or contact an administrator.")
    }
}
