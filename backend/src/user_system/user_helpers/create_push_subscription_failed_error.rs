use serde::{Deserialize, Serialize};

use crate::app::app_helpers::app_error_helper::AppError;

pub const DEFAULT_CODE: &str = "create_push_subscription_failed";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePushSubscriptionFailedError;

impl CreatePushSubscriptionFailedError {
    pub fn new(message: Option<&str>) -> AppError {
        AppError::new(DEFAULT_CODE, message.unwrap_or_else(|| "could not create push subscription"))
    }

    pub fn to_delete_error(message: Option<&str>) -> AppError {
        AppError::new("delete_push_subscription_failed", message.unwrap_or_else(|| "could not delete push subscription"))
    }
}
