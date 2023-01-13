use serde::{Deserialize, Serialize};

use crate::user_system::user_entities::PushSubscriptionEntity;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PushSubscriptionResponseDto<T = PushSubscriptionEntity> {
    pub success: bool,
    pub push_subscription: Option<T>,
    pub code: Option<String>,
    pub message: Option<String>,
}

impl<T> PushSubscriptionResponseDto<T> {
    pub fn new(entity: T) -> Self {
        Self {
            success: true,
            push_subscription: Some(entity),
            message: None,
            code: None,
        }
    }

    pub fn new_not_found(message: Option<&str>, code: Option<&str>) -> Self {
        Self {
            success: false,
            push_subscription: None,
            message: Some(
                message
                    .unwrap_or_else(|| "push subscription not found")
                    .to_owned(),
            ),
            code: Some(
                code.unwrap_or_else(|| "push_subscription_not_found")
                    .to_owned(),
            ),
        }
    }
}
