use crate::{
    app::app_dtos::api_response_dto::ApiResponseDto,
    country_system::{
        country_dtos::country_entity_api_dto::CountryEntityApiDto,
        country_services::country_service::CountryService,
    },
};
use actix_web::{get, web, Responder, Result};

#[get("country/{country_id}")]
pub(crate) async fn handler(
    path: web::Path<String>,
    country_service: web::Data<CountryService>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut response =
        ApiResponseDto::new_not_found(Some("record not found"), Some("record_not_found"));

    if let Some(country) = country_service.get_country_by_internal_id(&id).await {
        response = ApiResponseDto::new(CountryEntityApiDto::from(country));
    }

    Ok(web::Json(response))
}
