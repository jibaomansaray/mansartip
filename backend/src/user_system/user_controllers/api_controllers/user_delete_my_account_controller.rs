use actix_web::{delete, web, Responder, Result};

use crate::{
    app::app_helpers::authenticated_user::AuthenticatedUser,
    user_system::{
        user_dtos::user_entity_api_response_dto::UserEntityApiResponseDto,
        user_services::user_service::UserService,
    },
};

#[delete("user/my-account")]
pub(crate) async fn handler(
    auth_user: AuthenticatedUser,
    user_service: web::Data<UserService>,
) -> Result<impl Responder> {
    let user = auth_user.into_inner();
    let result = user_service.delete_my_account(&user.internal_id).await;
    match result {
        Ok(user) => Ok(web::Json(UserEntityApiResponseDto::new(user))),
        Err(e) => Ok(web::Json(UserEntityApiResponseDto::new_not_found(
            Some(&e.message),
            Some(&e.code),
        ))),
    }
}
