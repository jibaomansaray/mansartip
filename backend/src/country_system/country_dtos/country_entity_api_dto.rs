use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::country_system::country_entities::CountryEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryEntityApiDto {
    #[serde(skip)]
    id: u64,
    internal_id: String,
    name: String,
    year: i16,
    short: String,
    group_points: u8,
    image: String,
    deleted_at: Option<DateTime<Utc>>,
}

impl From<CountryEntity> for CountryEntityApiDto {
    fn from(entity: CountryEntity) -> Self {
        Self {
            id: entity.id,
            internal_id: entity.internal_id,
            name: entity.name,
            year: entity.year,
            short: entity.short,
            group_points: entity.group_points,
            image: entity.image,
            deleted_at: entity.deleted_at
        }
    }
}
