use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseDto<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub code: Option<String>,
}

impl<T: Serialize> ApiResponseDto<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            code: None,
            message: None,
        }
    }

    pub fn new_not_found(message: Option<&str>, code: Option<&str>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.unwrap_or_else(|| "Entity not found").to_owned()),
            code: Some(code.unwrap_or_else(|| "entity_not_found").to_owned()),
        }
    }
}
