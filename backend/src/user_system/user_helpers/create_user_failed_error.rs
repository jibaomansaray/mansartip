use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserFailedError {
    pub code: String,
    pub message: String,
}

impl Default for CreateUserFailedError {
    fn default() -> Self {
        Self {
            code: "create_user_field".to_owned(),
            message: "Could not create User".to_owned(),
        }
    }
}

impl fmt::Display for CreateUserFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("user is not authenticated"));
        write!(f, "{message}")
    }
}
