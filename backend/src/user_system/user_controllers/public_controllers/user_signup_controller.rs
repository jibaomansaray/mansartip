use crate::user_system::{user_entities::UserEntity, user_services::user_service::UserService};
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
    _user_service: web::Data<UserService>,
) -> Result<impl Responder> {
    let mut user_entity = UserEntity::default();
    user_entity.username = payload.username.clone();
    user_entity.email = payload.email.clone();
    user_entity.password = payload.password.clone();

    Ok(web::Json(user_entity))
}
