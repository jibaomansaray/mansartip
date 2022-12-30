use crate::{
    app::app_state::AppState,
    user_system::{
        user_dtos::user_entity_api_response_dto::UserEntityApiResponseDto,
        user_entities::UserEntity, user_services::user_service::UserService,
    },
};
use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SignupPayload {
    username: String,
    email: String,
    password: String,
}

#[post("user/signup")]
pub(crate) async fn handler(
    payload: web::Json<SignupPayload>,
    user_service: web::Data<UserService>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder> {
    let result = user_service
        .create_user(&payload.username, &payload.email, &payload.password)
        .await;

    match result {
        Ok(user_entity) => Ok(web::Json(UserEntityApiResponseDto::new_auth_entity(
            user_entity,
            Some(app_data.vapid_public_key.clone()),
        ))),
        Err(e) => Ok(web::Json(UserEntityApiResponseDto::new_not_found(
            Some(e.message),
            Some(e.code),
        ))),
    }
}
