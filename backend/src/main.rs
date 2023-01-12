mod app;
mod api_routes;
mod country_system;
mod general;
mod public_routes;
mod user_system;
mod match_system;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::web::init().await
}
