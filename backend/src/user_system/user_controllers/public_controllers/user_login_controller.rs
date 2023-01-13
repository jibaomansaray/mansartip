use actix_web::{cookie::Cookie, post, web, HttpResponse, Responder, Result};

use serde::{Deserialize, Serialize};

use crate::{
    app::{app_dtos::api_response_dto::ApiResponseDto, app_state::AppState},
    user_system::{
        user_dtos::user_auth_entity_dto::UserAuthEntityDto,
        user_services::user_service::UserService,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[post("user/login")]
pub(crate) async fn handler(
    payload: web::Json<LoginPayload>,
    user_service: web::Data<UserService>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder> {
    let user = user_service
        .log_user_in(&payload.username, &payload.password)
        .await;

    if let Some(auth_user) = user {
        // @todo secure cookie
        let cookie = Cookie::build("_t", &auth_user.token)
            .permanent()
            .secure(true)
            .path("/")
            .finish();

        let response = HttpResponse::Ok()
            .cookie(cookie)
            .json(web::Json(ApiResponseDto::new(UserAuthEntityDto::new(
                auth_user,
                &app_data.vapid_public_key,
            ))));

        Ok(response)
    } else {
        let response = HttpResponse::Ok().json(web::Json(
            ApiResponseDto::<UserAuthEntityDto>::new_not_found(
                Some("posted data is incorrect"),
                Some("posted_data_incorrect"),
            ),
        ));

        Ok(response)
    }
}
