use std::collections::HashMap;

use crate::{
    app::{app_helpers::database_datetime_helper::datetime_from_db_row_field, app_state::DbRow},
    user_system::user_dtos::user_entity_api_dto::UserEntityDto,
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

impl From<&UserRole> for u64 {
    fn from(role: &UserRole) -> u64 {
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

impl From<&UserType> for u64 {
    fn from(user_type: &UserType) -> u64 {
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
        let map = HashMap::new();
        Self::pluck_values(&row, &map).unwrap()
    }
}

impl From<&DbRow> for UserEntity {
    fn from(row: &DbRow) -> Self {
        let map = HashMap::new();
        Self::pluck_values(row, &map).unwrap()
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

pub struct UserJoinProcessor<'a> {
    pub fields: String,
    pub field_map: HashMap<&'a str, String>,
}

impl<'a> UserJoinProcessor<'a> {
    pub fn transform(&self, row: &DbRow) -> Option<UserEntityDto> {
        UserEntity::new_for_join(row, &self.field_map)
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

    pub fn new_for_join(row: &DbRow, map: &HashMap<&str, String>) -> Option<UserEntityDto> {
        if let Some(entity) = Self::pluck_values(row, map) {
            Some(UserEntityDto::new(entity))
        } else {
            None
        }
    }

    fn pluck_values(row: &DbRow, map: &HashMap<&str, String>) -> Option<Self> {
        let id_field = match map.get("id") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "id",
        };
        let internal_id_field = match map.get("internalId") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "internalId",
        };

        let role_field = match map.get("role") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "role",
        };
        let type_field = match map.get("type") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "type",
        };

        let username_field = match map.get("username") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "username",
        };

        let data_field = match map.get("data") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "data",
        };

        let email_field = match map.get("email") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "email",
        };
        let password_field = match map.get("password") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "password",
        };

        let token_field = match map.get("token") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "token",
        };

        let created_at_field = match map.get("createdAt") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "careatedAt",
        };

        let updated_at_field = match map.get("updatedAt") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "updatedAt",
        };
        let deleted_at_field = match map.get("deletedAt") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "deletedAt",
        };

        let id: u64 = row.try_get(id_field).unwrap_or_default();
        let username: String = row.try_get(username_field).unwrap_or_default();

        if id > 0 {
            Some(Self {
                id,
                internal_id: row.try_get(internal_id_field).unwrap_or_default(),
                role: match row
                    .try_get(role_field)
                    .unwrap_or_else(|_| String::from("user"))
                    .as_str()
                {
                    "admin" => UserRole::Admin,
                    _ => UserRole::User,
                },
                user_type: match row
                    .try_get(type_field)
                    .unwrap_or_else(|_| String::from("user"))
                    .as_str()
                {
                    "human" => UserType::Human,
                    _ => UserType::Bot,
                },
                username: username.clone(),
                email: row.try_get(email_field).unwrap_or_default(),
                password: row.try_get(password_field).unwrap_or_default(),
                token: row.try_get(token_field).unwrap_or_default(),
                data: row.try_get(data_field).unwrap_or_default(),
                avater: format!("/static/user/{}.png", username),
                created_at: datetime_from_db_row_field(created_at_field, &row),
                updated_at: datetime_from_db_row_field(updated_at_field, &row),
                deleted_at: datetime_from_db_row_field(deleted_at_field, &row),
            })
        } else {
            None
        }
    }

    pub fn generate_join_fields(prefix: Option<&str>) -> UserJoinProcessor {
        let table = prefix.unwrap_or_else(|| "user");
        let as_field = |name| format!("`{0}`.`{1}` as {0}_{1}", table, name);

        let map = HashMap::from([
            ("id", as_field("id")),
            ("internalId", as_field("internalId")),
            ("role", as_field("role")),
            ("type", as_field("type")),
            ("username", as_field("username")),
            ("email", as_field("email")),
            ("password", as_field("password")),
            ("token", as_field("token")),
            ("createdAt", as_field("createdAt")),
            ("updatedAt", as_field("updatedAt")),
            ("deletedAt", as_field("deletedAt")),
        ]);

        let mut fields = String::new();

        for entry in &map {
            if !fields.is_empty() {
                fields.push_str(",")
            }
            fields.push_str(entry.1.as_str())
        }

        UserJoinProcessor {
            fields,
            field_map: map,
        }
    }
}
