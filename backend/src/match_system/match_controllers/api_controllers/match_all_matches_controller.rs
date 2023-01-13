use crate::{
    app::{app_dtos::api_response_dto::ApiResponseDto, app_state::AppState},
    match_system::{
        match_dtos::match_entity_api_dto::MatchEntityDto,
        match_services::match_service::MatchService,
    },
};
use actix_web::{get, web, Responder, Result};

#[get("/match/all")]
pub(crate) async fn handler(
    match_service: web::Data<MatchService>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder> {
    let response = ApiResponseDto::new(MatchEntityDto::from_entities(
        match_service.get_all_matches(app_data.year).await,
    ));

    Ok(web::Json(response))
}
