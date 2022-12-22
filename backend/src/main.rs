mod app;
mod api_routes;
mod country;
mod game;
mod general;
mod public_routes;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::web::init().await
}
