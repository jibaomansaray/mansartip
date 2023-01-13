mod api_routes;
mod app;
mod country_system;
mod general;
mod match_system;
mod public_routes;
mod user_system;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::web::init().await
}
