use actix_web::{get, web, Responder, Result};

use crate::{
    app::app_dtos::api_response_dto::ApiResponseDto,
    tipping_system::tipping_services::tipping_scoreboard_service::TippingScoreboardService,
};

#[get("tip/scoreboard")]
pub async fn handler(
    scoreboard_service: web::Data<TippingScoreboardService>,
) -> Result<impl Responder> {
    let response = scoreboard_service.entries(2022, 25).await;

    Ok(web::Json(ApiResponseDto::new(response)))
}
