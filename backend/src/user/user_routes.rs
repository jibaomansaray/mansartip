use actix_web::Scope;

use crate::user::user_controllers;

pub fn register_api_routes(scope: Scope) -> Scope {
    scope.service(user_controllers::user_my_information_controller::handler)
}

pub fn register_public_routes(scope: Scope) -> Scope {
    scope
}
