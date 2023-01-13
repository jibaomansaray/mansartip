use crate::country_system::country_entities::CountryEntity;

use super::country_entity_api_dto::CountryEntityApiDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CountryApiResponseDto {
    pub success: bool,
    pub country: Option<CountryEntityApiDto>,
    pub message: Option<String>,
    pub code: Option<String>,
}

impl CountryApiResponseDto {
    pub fn new_not_found() -> Self {
        Self {
            success: false,
            country: None,
            message: Some("record not found".to_owned()),
            code: Some("record_not_found".to_owned()),
        }
    }

    pub fn new(country: CountryEntity) -> Self {
        Self {
            success: true,
            code: None,
            message: None,
            country: Some(CountryEntityApiDto::from(country)),
        }
    }
}
