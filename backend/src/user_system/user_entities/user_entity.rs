use crate::app::app_state::DbRow;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use chrono::prelude::*;


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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntity {
    #[serde(skip)]
    pub id: i32,
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
        let created_at: Option<DateTime<Utc>> = match row.try_get("createdAt") {
            Ok(date) => {
                Some(date)
            }
            _ =>{
                None
            } ,
        };
        let updated_at: Option<DateTime<Utc>> = match row.try_get("updatedAt") {
            Ok(date) => Some(date),
            _ => None,
        };
        let deleted_at: Option<DateTime<Utc>> = match row.try_get("deletedAt") {
            Ok(date) => Some(date),
            _ => None,
        };

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
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
