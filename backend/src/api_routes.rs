use actix_web::web;

pub(crate) mod auth_middleware;

use crate::country::country_routes;
use crate::game::game_routes;
use crate::user_system::user_routes;

pub fn configure(config: &mut web::ServiceConfig) {
    let mut api_routes = web::scope("/api/v1");

    api_routes = country_routes::register_api_routes(api_routes);
    api_routes = game_routes::register_api_routes(api_routes);
    api_routes = user_routes::register_api_routes(api_routes);

    // api routes middleware
    let api_service = api_routes.wrap(auth_middleware::Auth {});

    config.service(api_service);
}
