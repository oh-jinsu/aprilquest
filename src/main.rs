use dotenv::dotenv;
use actix_web::{web, App, HttpServer };
use mysql::*;
use mysql::prelude::*;

use aprilquest::controller;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("Host is not defined.");

    let port = env::var("PORT").expect("Port is not defined.");

    let database_url = env::var("DATABASE_URL").expect("Database url is not defined.");

    let opts = Opts::from_url(&database_url).expect("Database url is not valid.");

    let pool = Pool::new(opts).expect("Failed to construct a pool.");

    let mut conn = pool.get_conn().expect("Failed to connect");

    if cfg!(debug_assertions) {
        conn.query_drop(
            r"DROP TABLE IF EXISTS users"
        ).unwrap();
    }

    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT NOT NULL AUTO_INCREMENT,
            email VARCHAR(255) NOT NULL,
            password VARCHAR(32) NOT NULL,
            PRIMARY KEY ( id )
        )"
    ).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/app")
                            .service(controller::app::hello)
                            .service(controller::app::health)
                    )
                    .service(
                        web::scope("/auth")
                            .service(controller::auth::signup)
                    )
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}