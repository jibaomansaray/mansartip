use crate::app::app_state::AppState;
use actix_web::web;

use super::{
    tipping_scoreboard_repo_service::ScoreboardRepo,
    tipping_scoreboard_service::TippingScoreboardService,
};

pub fn configure(config: &mut web::ServiceConfig, app_data: web::Data<AppState>) {
    let scoreboard_repo = ScoreboardRepo::new(app_data.db_pool.clone());
    let tipping_scoreboard_service = web::Data::new(TippingScoreboardService::new(scoreboard_repo));

    config.app_data(tipping_scoreboard_service);
}
