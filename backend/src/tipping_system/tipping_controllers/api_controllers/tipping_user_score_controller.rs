use actix_web::{get, web, Responder, Result};

use crate::{
    app::{app_dtos::api_response_dto::ApiResponseDto, app_state::AppState},
    tipping_system::tipping_services::tipping_scoreboard_service::TippingScoreboardService,
};

#[get("tip/user-score/{user}")]
pub async fn handler(
    path: web::Path<String>,
    scoreboard_service: web::Data<TippingScoreboardService>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    let user_internal_id = path.into_inner();
    let response = scoreboard_service
        .get_by_user_internal_id(&user_internal_id, app_state.year)
        .await;

    match response {
        Some(score) => Ok(web::Json(ApiResponseDto::new(score))),
        _ => Ok(web::Json(ApiResponseDto::new_not_found(
            Some("No entry found"),
            Some("on_score_entry"),
        ))),
    }
}
