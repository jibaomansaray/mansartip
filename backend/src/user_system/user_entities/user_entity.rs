use crate::app::{
    app_helpers::database_datetime_helper::{
        created_at_field_value, deleted_at_field_value, updated_at_field_value,
    },
    app_state::DbRow,
};
use chrono::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserType {
    #[serde(rename = "human")]
    Human,
    #[serde(rename = "bot")]
    Bot,
}

impl From<UserRole> for u64 {
    fn from(role: UserRole) -> u64 {
        match role {
            UserRole::User => 1,
            UserRole::Admin => 2,
        }
    }
}

impl From<UserType> for u64 {
    fn from(user_type: UserType) -> u64 {
        match user_type {
            UserType::Human => 1,
            UserType::Bot => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntity {
    #[serde(skip)]
    pub id: u64,
    pub internal_id: String,
    pub role: UserRole,
    pub avater: String,
    #[serde(rename = "type")]
    pub user_type: UserType,
    pub username: String,
    pub email: String,
    pub password: String,
    pub token: String,
    pub data: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<DbRow> for UserEntity {
    fn from(row: DbRow) -> Self {
        let username: String = row.try_get("username").unwrap_or_default();
        Self {
            id: row.try_get("id").unwrap_or_default(),
            internal_id: row.try_get("internalId").unwrap_or_default(),
            role: match row
                .try_get("role")
                .unwrap_or_else(|_| String::from("user"))
                .as_str()
            {
                "admin" => UserRole::Admin,
                _ => UserRole::User,
            },
            user_type: match row
                .try_get("type")
                .unwrap_or_else(|_| String::from("user"))
                .as_str()
            {
                "human" => UserType::Human,
                _ => UserType::Bot,
            },
            username: row.try_get("username").unwrap_or_default(),
            email: row.try_get("email").unwrap_or_default(),
            password: row.try_get("password").unwrap_or_default(),
            token: row.try_get("token").unwrap_or_default(),
            data: row.try_get("data").unwrap_or_default(),
            avater: format!("/static/user/{}.png", username),
            created_at: created_at_field_value(&row),
            updated_at: updated_at_field_value(&row),
            deleted_at: deleted_at_field_value(&row),
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        Self {
            id: 0,
            internal_id: Uuid::new_v4().to_string(),
            role: UserRole::User,
            avater: "".to_owned(),
            user_type: UserType::Human,
            username: "".to_owned(),
            email: "".to_owned(),
            password: "".to_string(),
            token: Self::generate_token(),
            data: "".to_owned(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

impl UserEntity {
    pub fn reset_token(&mut self) {
        self.token = Self::generate_token()
    }

    pub fn generate_token() -> String {
        let bytes = rand::thread_rng().gen::<[u8; 32]>();
        let mut hex = "".to_owned();

        for e in &bytes {
            hex += format!("{:x?}", e).as_str()
        }

        hex
    }
}
