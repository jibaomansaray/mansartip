use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub(crate) async fn index_page() -> impl Responder {
    HttpResponse::Ok().body("Index Page")
}