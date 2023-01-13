use serde::{Deserialize, Serialize};
use std::fmt;

pub const DEFAULT_CODE: &str = "create_user_failed";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserFailedError {
    pub code: String,
    pub message: String,
}

impl CreateUserFailedError {
    pub fn new(message: &str) -> Self {
        Self {
            code: DEFAULT_CODE.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl Default for CreateUserFailedError {
    fn default() -> Self {
        Self {
            code: DEFAULT_CODE.to_owned(),
            message: "Could not create User".to_owned(),
        }
    }
}

impl fmt::Display for CreateUserFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message =
            serde_json::to_string(&self).unwrap_or_else(|_| String::from("could not create user"));
        write!(f, "{message}")
    }
}
