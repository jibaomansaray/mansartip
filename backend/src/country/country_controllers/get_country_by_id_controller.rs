use crate::country::{
    country_dtos::{
        country_api_response_dto::CountryApiResponseDto
    },
    country_services::country_service::CountryService,
};
use actix_web::{get, web, Responder, Result};

#[get("country/{country_id}")]
pub(crate) async fn handler(
    path: web::Path<i32>,
    country_service: web::Data<CountryService>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut response = CountryApiResponseDto::new_not_found();

    if let Some(country) = country_service.get_country_by_id(id).await {
        response = CountryApiResponseDto::new(country);
    }

    Ok(web::Json(response))
}
