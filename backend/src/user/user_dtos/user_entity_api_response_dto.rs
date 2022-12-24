use crate::user::user_entities::UserEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserEntityApiResponseDto {
  pub success: bool,
  pub user: Option<UserEntity>,
  pub message: Option<String>
}
