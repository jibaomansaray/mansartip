use serde::{Deserialize, Serialize};

use crate::country::country_entities::CountryEntity;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CountryEntityApiDto {
    internal_id: Option<String>,
    name: Option<String>,
}

impl From<CountryEntity> for CountryEntityApiDto {
    fn from(entity: CountryEntity) -> Self {
        Self {
            internal_id: Some(entity.internal_id),
            name: Some(entity.name),
        }
    }
}
