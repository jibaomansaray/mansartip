use crate::user_system::user_entities::UserEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserEntityApiResponseDto<T = UserEntity> {
    pub success: bool,
    pub user: Option<T>,
    pub message: Option<String>,
    pub code: Option<String>,
    pub push_vapid: Option<String>,
}

impl<T> UserEntityApiResponseDto<T> {
    pub fn new(entity: T) -> Self {
        Self {
            success: true,
            user: Some(entity),
            message: None,
            code: None,
            push_vapid: None,
        }
    }

    pub fn new_auth_entity(entity: T, push_vapid: Option<&str>) -> Self {
        Self {
            success: true,
            user: Some(entity),
            message: None,
            code: None,
            push_vapid: Some(push_vapid.unwrap_or_else(|| "").to_owned()),
        }
    }

    pub fn new_not_found(message: Option<&str>, code: Option<&str>) -> Self {
        Self {
            success: false,
            user: None,
            message: Some(message.unwrap_or_else(||"User not found").to_owned()),
            code: Some(code.unwrap_or_else(|| "user_not_found").to_owned()),
            push_vapid: None,
        }
    }
}
