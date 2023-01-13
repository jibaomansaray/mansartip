use actix_web::{delete, web, Responder, Result};

use crate::{
    app::{
        app_dtos::api_response_dto::ApiResponseDto,
        app_helpers::authenticated_user::AuthenticatedUser,
    },
    user_system::user_services::user_service::UserService,
};

#[delete("user/my-account")]
pub(crate) async fn handler(
    auth_user: AuthenticatedUser,
    user_service: web::Data<UserService>,
) -> Result<impl Responder> {
    let user = auth_user.into_inner();
    let result = user_service.delete_my_account(&user.internal_id).await;
    match result {
        Ok(user) => Ok(web::Json(ApiResponseDto::new(user))),
        Err(e) => Ok(web::Json(ApiResponseDto::new_not_found(
            Some(&e.message),
            Some(&e.code),
        ))),
    }
}
