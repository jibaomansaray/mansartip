use actix_web::Scope;

use super::tipping_controllers;

pub fn register_api_routes(scope: Scope) -> Scope {
    scope
        .service(tipping_controllers::api_controllers::tipping_scoreboard_controller::handler)
        .service(tipping_controllers::api_controllers::tipping_my_score_controller::handler)
        .service(tipping_controllers::api_controllers::tipping_user_score_controller::handler)
}

pub fn register_public_routes(scope: Scope) -> Scope {
    scope
}
