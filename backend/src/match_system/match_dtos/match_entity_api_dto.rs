use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    country_system::country_dtos::country_entity_api_dto::CountryEntityApiDto,
    match_system::match_entities::match_entity::{MatchEntity, MatchRound, MatchStatus},
};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MatchEntityDto {
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
    pub country_a: Option<CountryEntityApiDto>,
    pub country_b: Option<CountryEntityApiDto>,
    pub winner: Option<CountryEntityApiDto>,
}

impl From<MatchEntity> for MatchEntityDto {
    fn from(entity: MatchEntity) -> Self {
        Self {
            internal_id: entity.internal_id,
            status: entity.status,
            round: entity.round,
            number: entity.number,
            date: entity.date,
            time: entity.time,
            penalty: entity.penalty,
            country_a_goals: entity.country_a_goals,
            country_b_goals: entity.country_b_goals,
            country_a_penalty_goals: entity.country_a_penalty_goals,
            country_b_penalty_goals: entity.country_b_penalty_goals,
            to_configure: entity.to_configure,
            country_a: entity.country_a,
            country_b: entity.country_b,
            winner: entity.winner,
        }
    }
}
impl From<&MatchEntity> for MatchEntityDto {
    fn from(entity: &MatchEntity) -> Self {
        Self::from(entity.clone())
    }
}

impl MatchEntityDto {
    pub fn from_entities(entities: Vec<MatchEntity>) -> Vec<Self> {
        entities.iter().map(|e| e.into()).collect()
    }
}
