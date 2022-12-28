
use actix_web::{delete, web, HttpRequest , Responder, Result};

#[delete("user/my-account")]
pub(crate) async fn handler() -> impl Responder {
    HttpResponse::Ok().body("user delete my account")
}