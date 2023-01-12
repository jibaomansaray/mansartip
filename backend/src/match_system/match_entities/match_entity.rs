use std::collections::HashMap;

use crate::app::app_state::DbRow;
use crate::country_system::country_dtos::country_entity_api_dto::CountryEntityApiDto;
use crate::country_system::country_entities::CountryEntity;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::time;
use sqlx::Row;
// use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MatchStatus {
    Pending,
    Open,
    Close,
    ScoreEntered,
    Completed,
}

impl From<&str> for MatchStatus {
    fn from(value: &str) -> Self {
        match value {
            "close" => MatchStatus::Close,
            "completed" => MatchStatus::Completed,
            "open" => MatchStatus::Open,
            "pending" => MatchStatus::Pending,
            "score_entered" => MatchStatus::ScoreEntered,
            _ => Self::default(),
        }
    }
}

impl Default for MatchStatus {
    fn default() -> Self {
        MatchStatus::Completed
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MatchRound {
    Group,
    #[serde(rename = "round_16")]
    Round16,
    #[serde(rename = "round_8")]
    Round8,
    #[serde(rename = "round_4")]
    Round4,
    ThirdPlace,
    Final,
}

impl From<&str> for MatchRound {
    fn from(value: &str) -> Self {
        match value {
            "group" => MatchRound::Group,
            "round_16" => MatchRound::Round16,
            "round_8" => MatchRound::Round8,
            "round_4" => MatchRound::Round4,
            "third_place" => MatchRound::ThirdPlace,
            "final" => MatchRound::Final,
            _ => Self::default(),
        }
    }
}

impl Default for MatchRound {
    fn default() -> Self {
        MatchRound::Group
    }
}

// to use date or time with sqlx, you need the time crate

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntity {
    pub id: u64,
    pub internal_id: String,
    pub status: MatchStatus,
    pub round: MatchRound,
    pub number: i32,
    pub date: Option<sqlx::types::time::Date>,
    pub time: Option<sqlx::types::time::Time>,
    pub penalty: bool,
    pub country_a_goals: i32,
    pub country_b_goals: i32,
    pub country_a_penalty_goals: i32,
    pub country_b_penalty_goals: i32,
    pub to_configure: Option<DateTime<Utc>>,
    pub winner_id: u64,
    pub country_a_id: u64,
    pub country_b_id: u64,
    pub country_a: Option<CountryEntityApiDto>,
    pub country_b: Option<CountryEntityApiDto>,
    pub winner: Option<CountryEntityApiDto>,
}

impl MatchEntity {
    pub fn from_row_ref(row: &DbRow) -> Self {
        let status: &str = row.try_get("status").unwrap_or_else(|e| {
            dbg!(e);
            "completed"
        });
        let round: &str = row.try_get("round").unwrap_or_else(|e| {
            dbg!(e);
            "group"
        });

        Self {
            id: row.try_get("id").unwrap_or_default(),
            internal_id: row.try_get("internalId").unwrap_or_default(),
            status: status.into(),
            round: round.into(),
            penalty: row.try_get("penalty").unwrap_or_default(),
            number: row.try_get("number").unwrap_or_default(),
            date: row.try_get("date").unwrap_or_else(|_| None),
            time: row.try_get("time").unwrap_or_else(|_| None),
            country_a_goals: row.try_get("countryAGoals").unwrap_or_default(),
            country_b_goals: row.try_get("countryBGoals").unwrap_or_default(),
            country_a_penalty_goals: row.try_get("countryAPenaltyGoals").unwrap_or_default(),
            country_b_penalty_goals: row.try_get("countryBPenaltyGoals").unwrap_or_default(),
            to_configure: row.try_get("toConfigureOn").unwrap_or_default(),
            winner_id: row.try_get("winnerId").unwrap_or_default(),
            country_a_id: row.try_get("countryAId").unwrap_or_default(),
            country_b_id: row.try_get("countryBId").unwrap_or_default(),
            country_a: None,
            country_b: None,
            winner: None,
        }
    }
}

impl From<DbRow> for MatchEntity {
    fn from(row: DbRow) -> Self {
        let status: &str = row.try_get("status").unwrap_or_else(|e| {
            dbg!(e);
            "completed"
        });
        let round: &str = row.try_get("round").unwrap_or_else(|e| {
            dbg!(e);
            "group"
        });

        Self {
            id: row.try_get("id").unwrap_or_default(),
            internal_id: row.try_get("internalId").unwrap_or_default(),
            status: status.into(),
            round: round.into(),
            penalty: row.try_get("penalty").unwrap_or_default(),
            number: row.try_get("number").unwrap_or_default(),
            date: row.try_get("date").unwrap_or_else(|_| None),
            time: row.try_get("time").unwrap_or_else(|_| None),
            country_a_goals: row.try_get("countryAGoals").unwrap_or_default(),
            country_b_goals: row.try_get("countryBGoals").unwrap_or_default(),
            country_a_penalty_goals: row.try_get("countryAPenaltyGoals").unwrap_or_default(),
            country_b_penalty_goals: row.try_get("countryBPenaltyGoals").unwrap_or_default(),
            to_configure: row.try_get("toConfigureOn").unwrap_or_default(),
            winner_id: row.try_get("winnerId").unwrap_or_default(),
            country_a_id: row.try_get("countryAId").unwrap_or_default(),
            country_b_id: row.try_get("countryBId").unwrap_or_default(),
            country_a: None,
            country_b: None,
            winner: None,
        }
    }
}
