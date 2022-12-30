use actix_web::{get, web, Responder, Result, HttpResponse};
use cookie::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LogoutRespnse {
    success: bool,
}

#[get("user/logout")]
pub(crate) async fn handler() -> Result<impl Responder> {
    let cookie = Cookie::build("_t", "")
        .permanent()
        .secure(true)
        .path("/")
        .finish();

    let response = HttpResponse::Ok()
        .cookie(cookie)
        .json(web::Json(LogoutRespnse { success: true }));

    Ok(response)
}
