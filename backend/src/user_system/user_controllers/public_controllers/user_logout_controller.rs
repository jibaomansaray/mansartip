use actix_web::{get, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LogoutRespnse {
    success: bool,
}


#[get("user/logout")]
pub(crate) async fn handler() -> Result<impl Responder> {
    // @todo clear cookie
    Ok(web::Json(LogoutRespnse{
      success: true
    }))
}
