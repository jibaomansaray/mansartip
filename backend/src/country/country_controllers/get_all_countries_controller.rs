use actix_web::{get, web, HttpRequest, Responder, Result};

use crate::country::{
    country_dtos::{
        countries_api_response_dto::CountryApiResponseDto,
        country_entity_api_dto::CountryEntityApiDto,
    },
    country_services::country_service::CountryService,
};

#[get("country/all")]
pub(crate) async fn handler(
    _req: HttpRequest,
    country_repo: web::Data<CountryService>,
) -> Result<impl Responder> {
    let country_entities = country_repo.as_ref().get_all_countries().await;
    let mut countries = Vec::new();

    for a_country in country_entities {
        countries.push(CountryEntityApiDto::from(a_country));
    }

    let response = CountryApiResponseDto {
        success: true,
        countries,
        message: None,
        code: None,
    };

    Ok(web::Json(response))
}
