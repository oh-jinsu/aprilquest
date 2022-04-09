use super::{message, status};

use actix_web::{get, Responder};

#[get("")]
pub async fn hello() -> impl Responder {
  status(200).json(message("Hello, world!"))
}

#[get("/health")]
pub async fn health() -> impl Responder {
  status(200).json(message("I am healty."))
}