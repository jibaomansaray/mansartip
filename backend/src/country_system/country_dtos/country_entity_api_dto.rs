use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::country_system::country_entities::CountryEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryEntityApiDto {
    internal_id: String,
    name: String,
    year: i16,
    short: String,
    group_points: u8,
    image: String,
    deleted_at: Option<DateTime<Utc>>,
    image_source: String,
}

impl From<CountryEntity> for CountryEntityApiDto {
    fn from(entity: CountryEntity) -> Self {
        Self {
            internal_id: entity.internal_id,
            name: entity.name,
            year: entity.year,
            short: entity.short,
            group_points: entity.group_points,
            image: entity.image.clone(),
            deleted_at: entity.deleted_at,
            image_source: format!("/static/flag/{}", entity.image),
        }
    }
}


impl CountryEntityApiDto {
    pub fn from_entities(entities: Vec<CountryEntity>) -> Vec<Self> {
        entities.into_iter().map(|e|e.into()).collect()
    }
}