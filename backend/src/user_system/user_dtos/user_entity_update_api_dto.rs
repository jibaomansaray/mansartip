use serde::{Deserialize, Serialize};

use crate::user_system::user_entities::{UserRole, UserType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntityUpdateApiDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub role: Option<UserRole>,
    pub user_type: Option<UserType>,
}

impl UserEntityUpdateApiDto {
    pub(crate) fn username_or<'a>(&'a self, default: &'a str) -> &str {
        if let Some(s) = self.username.as_ref() {
            s
        } else {
            default
        }
    }

    pub(crate) fn email_or<'a>(&'a self, default: &'a str) -> &str {
        if let Some(s) = self.email.as_ref() {
            s
        } else {
            default
        }
    }

    pub(crate) fn password_or<'a>(&'a self, default: &'a str) -> &str {
        if let Some(s) = self.password.as_ref() {
            s
        } else {
            default
        }
    }

    pub(crate) fn token_or<'a>(&'a self, default: &'a str) -> &str {
        if let Some(s) = self.token.as_ref() {
            s
        } else {
            default
        }
    }

    pub(crate) fn role_or(&self, default: UserRole) -> u32 {
        if let Some(s) = self.role.as_ref() {
            u32::from(s.clone())
        } else {
            u32::from(default)
        }
    }

    pub(crate) fn user_type_or(&self, default: UserType) -> u32 {
        if let Some(s) = self.user_type.as_ref() {
            u32::from(s.clone())
        } else {
            u32::from(default)
        }
    }
}
