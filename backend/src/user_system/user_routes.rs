use actix_web::Scope;

use crate::user_system::user_controllers;

pub fn register_api_routes(scope: Scope) -> Scope {
    scope
        .service(user_controllers::api_controllers::user_my_information_controller::handler)
        .service(user_controllers::api_controllers::user_vapid_key_controller::handler)
        .service(user_controllers::api_controllers::user_delete_my_account_controller::handler)
        .service(user_controllers::api_controllers::user_update_controller::handler)
        .service(
            user_controllers::api_controllers::user_subscribe_to_notification_controller::handler,
        )
        .service(
            user_controllers::api_controllers::user_unsubscribe_to_notification_controller::handler,
        )
}

pub fn register_public_routes(scope: Scope) -> Scope {
    scope
        .service(user_controllers::public_controllers::user_login_controller::handler)
        .service(user_controllers::public_controllers::user_logout_controller::handler)
        .service(user_controllers::public_controllers::user_signup_controller::handler)
}
