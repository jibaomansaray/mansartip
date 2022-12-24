use actix_web::{get, web, HttpRequest , Responder, Result};


#[get("user/tip/{match_id}")]
pub(crate) async fn handler() -> impl Responder {
    HttpResponse::Ok().body("user get match tip")
}