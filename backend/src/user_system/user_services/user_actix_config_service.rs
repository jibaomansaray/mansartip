use actix_web::web;

use crate::app::app_state::AppState;

use super::{
    push_subscription_repo_service::PushSubscriptionRepoService,
    push_subscription_service::PushSubscriptionService, user_repo_service::UserRepoService,
    user_service::UserService,
};

pub fn configure(config: &mut web::ServiceConfig, app_data: web::Data<AppState>) {
    // user
    let user_repo = UserRepoService::new(app_data.db_pool.clone());
    let user_service = web::Data::new(UserService::new(user_repo));

    // push subscription
    let push_subscription_repo = PushSubscriptionRepoService::new(app_data.db_pool.clone());
    let push_subscription = web::Data::new(PushSubscriptionService::new(push_subscription_repo));

    config.app_data(user_service);
    config.app_data(push_subscription);
}
