use actix_web::{post, web, Responder, Result };
use serde::{Deserialize, Serialize};

use crate::user_system::{user_services::user_service::UserService, user_dtos::user_entity_api_response_dto::UserEntityApiResponseDto};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LoginPayload {
  pub username: String,
  pub password: String
}

#[post("user/login")]
pub(crate) async fn handler(payload: web::Json<LoginPayload>, user_service: web::Data<UserService>) -> Result<impl Responder> {
  let user = user_service.log_user_in(&payload.username, &payload.password).await;
  
  if let Some(auth_user) = user { 
    // @todo send token cookie
    // @todo auth user response must inclue the push notication token
    Ok(web::Json(UserEntityApiResponseDto::new(auth_user)))
  } else {
    Ok(web::Json(UserEntityApiResponseDto::new_not_found(Some(String::from("posted data is incorrect")), Some(String::from("posted_data_incorrect")))))
  }
}