use dotenv::dotenv;
use actix_web::{web, App, HttpServer };

use aprilquest::controller;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = match env::var("HOST") {
        Ok(v) => v,
        Err(_) => panic!("Host is undefined"),
    };

    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(_) => panic!("Port is undefined"),
    };

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/app")
                            .service(controller::app::hello)
                            .service(controller::app::health),
                    )
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}