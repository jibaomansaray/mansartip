use actix_web::{get, web, Responder, Result};

use crate::app::{
    app_dtos::api_response_dto::ApiResponseDto, app_helpers::authenticated_user::AuthenticatedUser,
};

/// Returns the current authenticated user's information
#[get("user/my-info")]
pub(crate) async fn handler(auth_user: AuthenticatedUser) -> Result<impl Responder> {
    Ok(web::Json(ApiResponseDto::new(auth_user.into_inner())))
}
