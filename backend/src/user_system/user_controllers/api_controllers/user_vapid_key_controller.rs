use actix_web::{get, web, Responder, Result};
use serde::{Serialize, Deserialize};

use crate::app::app_state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VapidTokenResponse {
    success: bool,
    token: String,
}

#[get("user/vapid-token")]
pub(crate) async fn handler(app_data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(VapidTokenResponse {
        success: true,
        token: app_data.vapid_public_key.clone(),
    }))
}
