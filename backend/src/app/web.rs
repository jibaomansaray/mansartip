use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::{env, sync::Arc};

use crate::{
    api_routes, app::app_state::AppState, country_system, match_system, public_routes, user_system,
};

pub async fn init() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("HTTP_PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .unwrap_or(8080);

    let ip4_address = env::var("HTTP_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let static_assets_path = env::var("PUBLIC_DIRECTORY").unwrap_or_else(|_| String::from("."));

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("mysql://root:dbpassword@db/rs_worldcup"));
    let year: i32 = env::var("WORLD_CUP_YEAR")
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();

    let db_pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("could not create database pool");

    let pool = Arc::new(db_pool);

    let app_data = web::Data::new(AppState::get_instance(pool, year).await);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api_routes::configure)
            .configure(public_routes::configure)
            .configure(|config| {
                country_system::country_services::country_actix_config_service::configure(
                    config,
                    app_data.clone(),
                );
                user_system::user_services::user_actix_config_service::configure(
                    config,
                    app_data.clone(),
                );
                match_system::match_services::match_actix_config_service::configure(
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
