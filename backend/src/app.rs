use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

use crate::{api_routes, country, public_routes};

use self::app_state::AppState;

pub mod app_state;

pub async fn app_init() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("HTTP_PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .unwrap_or(8080);

    let ip4_address = env::var("HTTP_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let static_assets_path = env::var("PUBLIC_DIRECTORY").unwrap_or_else(|_| String::from("."));

    let app_data = web::Data::new(AppState::get_instance().await);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api_routes::configure)
            .configure(public_routes::configure)
            .configure(|config| {
                country::country_services::country_actix_config_service::configure(
                    config,
                    app_data.clone(),
                )
            })
            .service(fs::Files::new("/static", &static_assets_path).index_file("index.html"))
    })
    .bind((ip4_address, port))?
    .run()
    .await
}
