use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::user_system::user_entities::UserEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntityDto {
    pub internal_id: String,
    pub avater: String,
    #[serde(rename = "type")]
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl UserEntityDto {
    pub fn new(entity: UserEntity) -> Self {
        Self {
            internal_id: entity.internal_id,
            avater: entity.avater,
            username: entity.username,
            created_at: entity.created_at,
        }
    }
}

impl From<UserEntity> for UserEntityDto {
    fn from(entity: UserEntity) -> Self {
        Self::new(entity)
    }
}
