use actix_web::Scope;

use super::country_controllers::{
    get_all_countries_controller, get_country_by_id_controller, new_country_controller,
};

pub fn register_api_routes(scope: Scope) -> Scope {
    scope
        .service(get_all_countries_controller::handler)
        .service(get_country_by_id_controller::handler)
        .service(new_country_controller::handler)
}

pub fn register_public_routes(scope: Scope) -> Scope {
    scope
}
