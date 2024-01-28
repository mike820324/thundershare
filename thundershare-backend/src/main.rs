mod domain;
mod presentation;
mod pgsql;

use actix_web::middleware::Logger;
use actix_web::web::{self, Data};
use actix_web::{App, HttpServer};
use domain::service::ServerService;
use env_logger::Env;
use pgsql::{connection_builder, ServerRepositories};
use presentation::customer::view::{customer_signin_v1, customer_signup_v1, customer_signout_v1};

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    // NOTE: customer auth related endpoints
    cfg.route("/api/v1/customer/signup", web::post().to(customer_signup_v1))
    .route("/api/v1/customer/signin", web::post().to(customer_signin_v1))
    .route("/api/v1/customer/signout", web::post().to(customer_signout_v1));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server_host = std::env::var("SERVER_HOST").unwrap();
    let server_port = std::env::var("SERVER_PORT").unwrap();
    let server_location = server_host + ":" + &server_port;

    let db_pool = connection_builder().await.unwrap();

    HttpServer::new(move || {
        let server_repositories = ServerRepositories::new(db_pool.clone());
        let server_domain_services = ServerService::new(
            server_repositories.customer_repository,
            server_repositories.used_token_repository
        );
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(server_domain_services))
            .configure(register_routes)
    })
    .bind(&server_location)?
    .run()
    .await
}
