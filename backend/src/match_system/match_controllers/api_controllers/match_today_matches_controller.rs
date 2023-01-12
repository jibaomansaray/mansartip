use actix_web::{get, web, Responder, Result};

use crate::{app::app_state::AppState, match_system::match_services::match_service::MatchService};

#[get("match/todays")]
pub(crate) async fn handler(
    match_service: web::Data<MatchService>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder> {
    Ok(web::Json(
        match_service
            .get_today_matches(Some(app_data.year))
            .await
            .unwrap(),
    ))
}
