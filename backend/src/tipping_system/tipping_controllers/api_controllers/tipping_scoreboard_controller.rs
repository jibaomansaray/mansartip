use actix_web::{get, web, Responder, Result};

use crate::{
    app::{app_dtos::api_response_dto::ApiResponseDto, app_state::AppState},
    tipping_system::tipping_services::tipping_scoreboard_service::TippingScoreboardService,
};

#[get("tip/scoreboard")]
pub async fn handler(
    scoreboard_service: web::Data<TippingScoreboardService>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    let response = scoreboard_service.entries(app_state.year, 25).await;

    Ok(web::Json(ApiResponseDto::new(response)))
}
