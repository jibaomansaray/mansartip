use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::app::{app_state::DbRow, app_helpers::database_datetime_helper::{created_at_field_value, updated_at_field_value, deleted_at_field_value}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushSubscriptionEntity {
    #[serde(skip)]
    pub id: u64,
    pub internal_id: String,
    pub user_id: u64,
    pub subscription: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<DbRow> for PushSubscriptionEntity {
    fn from(row: DbRow) -> Self {
        Self {
            id: row.try_get("id").unwrap_or_default(),
            internal_id: row.try_get("internalId").unwrap_or_default(),
            user_id: row.try_get("userId").unwrap_or_default(),
            subscription: row.try_get("subscription").unwrap_or_default(),
            created_at: created_at_field_value(&row),
            updated_at: updated_at_field_value(&row),
            deleted_at: deleted_at_field_value(&row),
        }
    }
}

impl PushSubscriptionEntity {
  pub fn new(user_id: u64, subscription: &str) -> Self {
      Self {
        id: 0,
        internal_id: Uuid::new_v4().to_string(),
        user_id,
        subscription: subscription.to_string(),
        created_at: None,
        updated_at: None,
        deleted_at: None
      }
  }
}