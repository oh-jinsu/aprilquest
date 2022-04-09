pub mod app;
use serde::Serialize;
use actix_web::{HttpResponse, HttpResponseBuilder, http::StatusCode};

#[derive(Serialize)]
pub struct PlainResponse {
    message: String,
}

pub fn message(message: &str) -> PlainResponse {
    PlainResponse {
        message: String::from(message),
    }
}

pub fn status(uint: u16) -> HttpResponseBuilder {
    HttpResponse::build(StatusCode::from_u16(uint).unwrap())
}