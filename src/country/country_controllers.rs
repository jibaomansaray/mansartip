use actix_web::{get, post, web, HttpResponse, Responder};

#[get("country/all")]
pub(crate) async fn get_all_countries() -> impl Responder {
    HttpResponse::Ok().body("[{name: \"abcd\"}, {name: \"b\"}]")
}


#[get("country/{country_id}")]
pub(crate) async fn get_country_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("{{name: '{}'}}", id))
}