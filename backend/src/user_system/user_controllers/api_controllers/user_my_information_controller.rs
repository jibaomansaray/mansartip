use actix_web::{get, web, Responder, Result};

use crate::{
    app::app_helpers::authenticated_user::AuthenticatedUser,
    user_system::user_dtos::user_entity_api_response_dto::UserEntityApiResponseDto,
};

/// Returns the current authenticated user's information
#[get("user/my-info")]
pub(crate) async fn handler(auth_user: AuthenticatedUser) -> Result<impl Responder> {
    Ok(web::Json(UserEntityApiResponseDto::new(auth_user.into_inner())))
}
