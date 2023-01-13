use actix_web::Scope;

use super::match_controllers;

pub fn register_api_routes(scope: Scope) -> Scope {
    scope.service(match_controllers::api_controllers::match_today_matches_controller::handler)
}

pub fn register_public_routes(scope: Scope) -> Scope {
    scope
}
