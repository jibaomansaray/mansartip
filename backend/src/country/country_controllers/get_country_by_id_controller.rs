
use actix_web::{get, web, HttpResponse, Responder };

use crate::country::country_services::country_repo_service::CountryRepoService;


#[get("country/{country_id}")]
pub(crate) async fn handler(path: web::Path<i32>, country_repo: web::Data<CountryRepoService>) -> impl Responder {
    let id = path.into_inner();
    if let Some(a_country) = country_repo.find_one_by_id(id).await {
        dbg!(a_country);
    }

    HttpResponse::Ok().body(format!("{{name: '{}'}}", id))
}
