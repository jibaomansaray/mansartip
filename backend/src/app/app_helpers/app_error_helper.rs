use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppError {
  pub code: String,
  pub message: String,
}

impl AppError {
    pub fn new(code: &str,message: &str) -> Self {
        Self {
            code: code.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("could not create pub subscription"));
        write!(f, "{message}")
    }
}

