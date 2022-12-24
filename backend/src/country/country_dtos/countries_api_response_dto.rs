use super::country_entity_api_dto::CountryEntityApiDto;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CountryApiResponseDto {
  pub success: bool,
  pub countries: Vec<CountryEntityApiDto>,
  pub message: Option<String>
}
