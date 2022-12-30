use actix_web::{get, HttpResponse, Responder};

#[get("match/todays")]
pub(crate) async fn get_today_matches() -> impl Responder {
    HttpResponse::Ok().body("[{match: \"abcd\"}, {match: \"b\"}]")
}