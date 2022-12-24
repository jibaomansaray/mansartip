use actix_web::{get, web, HttpRequest, Responder, Result};

use crate::country::{
    country_dtos::countries_api_response_dto::CountriesApiResponseDto,
    country_services::country_service::CountryService,
};

#[get("country/all")]
pub(crate) async fn handler(
    _req: HttpRequest,
    country_repo: web::Data<CountryService>,
) -> Result<impl Responder> {
    let country_entities = country_repo.as_ref().get_all_countries().await;

    Ok(web::Json(CountriesApiResponseDto::new(country_entities)))
}
