use crate::{app::app_state::DbRow, user_system::user_dtos::user_entity_api_dto::UserEntityDto};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScoreboardEntity {
    pub user: Option<UserEntityDto>,
    pub total_points: i64,
    pub positon: i64,
    pub year: i32,
    pub total_tips: i32,
}

impl From<DbRow> for ScoreboardEntity {
    fn from(row: DbRow) -> Self {
        Self {
            user: None,
            total_points: row.try_get("totalPoints").unwrap_or_default(),
            positon: row.try_get("position").unwrap_or_default(),
            year: row.try_get("year").unwrap_or_default(),
            total_tips: row.try_get("totalTips").unwrap_or_default(),
        }
    }
}

impl ScoreboardEntity {
    pub fn from_row_ref(row: &DbRow) -> Self {
        Self {
            user: None,
            total_points: row.try_get("totalPoints").unwrap_or_default(),
            positon: row.try_get("position").unwrap_or_default(),
            year: row.try_get("year").unwrap_or_default(),
            total_tips: row.try_get("totalTips").unwrap_or_default(),
        }
    }
}
