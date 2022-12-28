use actix_web::web;


use crate::country::country_routes;
use crate::game::game_routes;
use crate::general::general_routes;
use crate::user_system::user_routes;


pub fn configure(config: &mut web::ServiceConfig ) {
  let mut public_routes = web::scope("");

  public_routes = game_routes::register_public_routes(public_routes);
  public_routes = general_routes::register_public_routes(public_routes);
  public_routes = country_routes::register_public_routes(public_routes);
  public_routes = user_routes::register_public_routes(public_routes);

  config.service(public_routes);
}