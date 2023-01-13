use actix_web::{get, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::app::{app_dtos::api_response_dto::ApiResponseDto, app_state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VapidTokenResponse {
    token: String,
}

#[get("user/vapid-token")]
pub(crate) async fn handler(app_data: web::Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(ApiResponseDto::new(VapidTokenResponse {
        token: app_data.vapid_public_key.clone(),
    })))
}
