use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::{
    app::app_helpers::authenticated_user::AuthenticatedUser,
    user_system::{
        user_dtos::push_subscription_api_response_dto::PushSubscriptionResponseDto,
        user_services::push_subscription_service::PushSubscriptionService,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PushNotificationPayload {
    subscription: String, // @todo use the right data type/struct
}

#[post("/user/push-subscribe")]
pub(crate) async fn handler(
    payload: web::Json<PushNotificationPayload>,
    push_service: web::Data<PushSubscriptionService>,
    auth_user: AuthenticatedUser,
) -> Result<impl Responder> {
    match push_service
        .subscribe(&payload.subscription, auth_user.id)
        .await
    {
        Ok(entity) => Ok(web::Json(PushSubscriptionResponseDto::new(entity))),
        Err(e) => Ok(web::Json(PushSubscriptionResponseDto::new_not_found(
            Some(&e.message),
            Some(&e.code),
        ))),
    }
}
