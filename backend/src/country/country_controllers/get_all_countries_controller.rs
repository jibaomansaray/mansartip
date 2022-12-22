use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::country::country_services::country_service::CountryService;

#[get("country/all")]
pub(crate) async fn handler(
    _req: HttpRequest,
    country_repo: web::Data<CountryService>,
) -> impl Responder {
    let countries = country_repo.as_ref().get_all_countries().await;
    dbg!(countries);

    if let Some(a_country) = country_repo.get_country_by_id(1).await {
        dbg!(a_country);
    }

    HttpResponse::Ok().body("[{name: \"a1\"}, {name: \"a2\"}]")
}
