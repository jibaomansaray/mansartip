use actix_web::{post, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use crate::country::country_services::country_service::CountryService;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NewCountryPayload {
    name: String,
}

#[post("country")]
pub(crate) async fn handler(
    payload: web::Json<NewCountryPayload>,
    _country_repo: web::Data<CountryService>,
) -> impl Responder {
    dbg!(payload);
    HttpResponse::Ok().body(format!("{{name: '{}'}}", 55))
}
