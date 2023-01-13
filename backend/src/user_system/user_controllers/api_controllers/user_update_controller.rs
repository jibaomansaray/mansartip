use crate::{
    app::{
        app_dtos::api_response_dto::ApiResponseDto,
        app_helpers::authenticated_user::AuthenticatedUser,
    },
    user_system::{
        user_dtos::user_entity_update_api_dto::UserEntityUpdateApiDto,
        user_services::user_service::UserService,
    },
};
use actix_web::{put, web, Responder, Result};

#[put("user/my-data")]
pub(crate) async fn handler(
    payload: web::Json<UserEntityUpdateApiDto>,
    user_service: web::Data<UserService>,
    auth_user: AuthenticatedUser,
) -> Result<impl Responder> {
    match user_service
        .update_user(&auth_user.internal_id, payload.into_inner(), false)
        .await
    {
        Ok(user) => Ok(web::Json(ApiResponseDto::new(user))),
        Err(e) => Ok(web::Json(ApiResponseDto::new_not_found(
            Some(&e.message),
            Some(&e.code),
        ))),
    }
}
