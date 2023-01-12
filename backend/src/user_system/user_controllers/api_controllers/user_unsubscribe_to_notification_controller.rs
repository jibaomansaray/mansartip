use actix_web::{get, web, Responder, Result};

use crate::{
    app::app_helpers::authenticated_user::AuthenticatedUser,
    user_system::{
        user_dtos::push_subscription_api_response_dto::PushSubscriptionResponseDto,
        user_services::push_subscription_service::PushSubscriptionService,
    },
};

#[get("/user/push-unsubscribe/{internal_id}")]
pub(crate) async fn handler(
    internal_id: web::Path<String>,
    push_service: web::Data<PushSubscriptionService>,
    auth_user: AuthenticatedUser,
) -> Result<impl Responder> {
    match push_service.unsubscribe(internal_id.as_str(), &auth_user).await {
        Ok(entity) => Ok(web::Json(PushSubscriptionResponseDto::new(entity))),
        Err(e)=> Ok(web::Json(PushSubscriptionResponseDto::new_not_found(Some(&e.message), Some(&e.code)))),
    }
}
