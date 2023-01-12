use actix_web::web;

use crate::app::app_state::AppState;

use super::{match_match_repo_service::MatchRepoService, match_service::MatchService};

pub fn configure(config: &mut web::ServiceConfig, app_data: web::Data<AppState>) {
    let match_repo = MatchRepoService::new(app_data.db_pool.clone());
    let match_service = web::Data::new(MatchService::new(match_repo));

    config.app_data(match_service);
}
