use chrono::{DateTime, Utc};

use crate::{
    app::{
        app_helpers::database_datetime_helper::{
            created_at_field_value, deleted_at_field_value, updated_at_field_value,
        },
        app_state::DbRow,
    },
    country_system::country_dtos::country_entity_api_dto::CountryEntityApiDto,
    match_system::match_dtos::match_entity_api_dto::MatchEntityDto,
    user_system::user_dtos::user_entity_api_dto::UserEntityDto,
};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct TipEntity {
    pub id: u64,
    pub year: i32,
    pub user: Option<UserEntityDto>,
    pub the_match: Option<MatchEntityDto>,
    pub to_win: Option<CountryEntityApiDto>,
    pub is_level: bool,
    pub to_penalty: bool,
    pub country_a_to_score: i32,
    pub country_b_to_score: i32,
    pub counter_a_penalty_score: i32,
    pub counter_b_penalty_score: i32,
    pub entry_by_bot: bool,
    pub points: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<DbRow> for TipEntity {
    fn from(row: DbRow) -> Self {
        Self {
            id: row.try_get("id").unwrap_or_default(),
            year: row.try_get("year").unwrap_or_default(),
            user: None,
            the_match: None,
            to_win: None,
            is_level: row.try_get("isLevel").unwrap_or_default(),
            to_penalty: row.try_get("isPenalty").unwrap_or_default(),
            country_a_to_score: row.try_get("countryAToScore").unwrap_or_default(),
            country_b_to_score: row.try_get("countryBToScore").unwrap_or_default(),
            counter_a_penalty_score: row.try_get("countryAPenaltyToScore").unwrap_or_default(),
            counter_b_penalty_score: row.try_get("countryBPenaltyToScore").unwrap_or_default(),
            entry_by_bot: row.try_get("entryByBot").unwrap_or_default(),
            points: row.try_get("points").unwrap_or_default(),
            created_at: created_at_field_value(&row),
            updated_at: updated_at_field_value(&row),
            deleted_at: deleted_at_field_value(&row),
        }
    }
}
