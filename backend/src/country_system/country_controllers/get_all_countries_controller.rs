use actix_web::{get, web, HttpRequest, Responder, Result};

use crate::{
    app::app_dtos::api_response_dto::ApiResponseDto,
    country_system::{
        country_dtos::country_entity_api_dto::CountryEntityApiDto,
        country_services::country_service::CountryService,
    },
};

#[get("country/all")]
pub(crate) async fn handler(
    _req: HttpRequest,
    country_repo: web::Data<CountryService>,
) -> Result<impl Responder> {
    let response = ApiResponseDto::new(CountryEntityApiDto::from_entities(
        country_repo.as_ref().get_all_countries().await,
    ));

    Ok(web::Json(response))
}
