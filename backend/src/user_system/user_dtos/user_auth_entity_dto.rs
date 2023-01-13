use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::user_system::user_entities::{UserEntity, UserRole, UserType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAuthEntityDto {
    pub internal_id: String,
    pub role: UserRole,
    pub avater: String,
    #[serde(rename = "type")]
    pub user_type: UserType,
    pub username: String,
    pub email: String,
    pub token: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub push_vapid: String,
}

impl UserAuthEntityDto {
    pub fn new(entity: UserEntity, push_vapid: &str) -> Self {
        Self {
            internal_id: entity.internal_id,
            role: entity.role,
            avater: entity.avater,
            user_type: entity.user_type,
            username: entity.username,
            email: entity.email,
            token: entity.token,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
            push_vapid: push_vapid.to_string(),
        }
    }
}

impl From<UserEntity> for UserAuthEntityDto {
    fn from(entity: UserEntity) -> Self {
        Self::new(entity, "")
    }
}
