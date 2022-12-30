use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserFailedError {
  pub code: String,
  pub message: String,
}

impl Default for UpdateUserFailedError {
  fn default() -> Self {
    Self {
      code: "update_user_failed".to_owned(),
      message: "Could not update User".to_owned(),
    }
  }
}

impl fmt::Display for UpdateUserFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("Could not update User"));
        write!(f, "{message}")
    }
}