use crate::user_system::user_entities::UserEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserEntityApiResponseDto<T = UserEntity> {
    pub success: bool,
    pub user: Option<T>,
    pub message: Option<String>,
    pub code: Option<String>
}

impl<T> UserEntityApiResponseDto<T> {
    pub fn new(entity: T) -> Self {
      Self {
        success: true,
        user: Some(entity),
        message: None,
        code: None
      }
    }

    pub fn new_not_found(message: Option<String>, code: Option<String>) -> Self {
      Self {
        success: false,
        user: None,
        message: message.or_else(|| Some(String::from("User not found"))),
        code: code.or_else(|| Some(String::from("user_not_found")))
      }
    }
}
