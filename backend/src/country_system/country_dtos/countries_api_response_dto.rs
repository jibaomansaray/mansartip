use crate::country_system::country_entities::CountryEntity;

use super::country_entity_api_dto::CountryEntityApiDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CountriesApiResponseDto {
    pub success: bool,
    pub countries: Vec<CountryEntityApiDto>,
    pub message: Option<String>,
    pub code: Option<String>,
}

impl CountriesApiResponseDto {
    pub fn new(country_entities: Vec<CountryEntity>) -> Self {
        let mut countries = Vec::new();

        for a_country in country_entities {
            countries.push(CountryEntityApiDto::from(a_country));
        }
        Self {
            success: true,
            countries,
            message: None,
            code: None,
        }
    }
}
