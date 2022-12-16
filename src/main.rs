use actix_web::{web, dev::Service as _, App, HttpServer, Responder, HttpResponse};
use futures_util::future::FutureExt;

mod country;
mod game;
mod user;

use country::country_routes::{
    register_routes as register_country_routes
};
use game::game_routes::{
    register_routes as register_game_routes
};

use user::user_routes::{
    register_routes as register_user_routes
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut api_routes = web::scope("/api/v1");

        api_routes = register_country_routes(api_routes);
        api_routes = register_game_routes(api_routes);
        api_routes = register_user_routes(api_routes);

       let x = api_routes.wrap_fn(|req, srv| {
            println!("Hi from start. You requested: {}", req.path());
            srv.call(req).map(|res| {
                println!("Hi from response");
                res
            })
        });

        App::new().service(x).route("hello", web::get().to(say_hello_back))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn say_hello_back() -> impl Responder {
     HttpResponse::Ok().body("Hello back")
}
