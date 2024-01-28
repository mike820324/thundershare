mod domain;
mod presentation;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server_host = std::env::var("SERVER_HOST").unwrap();
    let server_port = std::env::var("SERVER_PORT").unwrap();
    let server_location = server_host + ":" + &server_port;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(register_routes)
    })
    .bind(&server_location)?
    .run()
    .await
}
