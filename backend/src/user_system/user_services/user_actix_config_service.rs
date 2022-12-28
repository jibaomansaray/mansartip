use actix_web::web;

use crate::app::app_state::AppState;

use super::{user_service::UserService, user_repo_service::UserRepoService};

pub fn configure(config: &mut web::ServiceConfig, app_data: web::Data<AppState>) {
  let user_repo = UserRepoService::new(app_data.db_pool.clone());
  let user_service = web::Data::new(UserService::new(user_repo));

  config.app_data(user_service);
}