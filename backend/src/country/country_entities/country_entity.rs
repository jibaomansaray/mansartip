use crate::app::app_state::DbRow;
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug)]
pub struct CountryEntity {
    pub id: i32,
    pub internal_id: String,
    pub year: u16,
    pub name: String,
    pub short: String,
    pub group_points: u8,
    pub image: String,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Default for CountryEntity {
    fn default() -> Self {
        Self {
            id: 0,
            internal_id: "".to_owned(),
            year: 0,
            name: "".to_owned(),
            short: "".to_owned(),
            group_points: 0,
            image: "".to_owned(),
            deleted_at: None,
        }
    }
}

impl CountryEntity {
    pub fn new (
        id: i32,
        internal_id: String,
        name: String,
        year: u16,
        short: String,
        group_points: u8,
        image: String,
    ) -> Self {
        Self {
            id,
            internal_id,
            year,
            name,
            short,
            group_points,
            image,
            deleted_at: None,
        }
    }
}

impl From<DbRow> for CountryEntity {
    fn from(row: DbRow) -> Self {
        Self {
            id: row.try_get("id").unwrap_or_default(),
            internal_id: row.try_get("internalId").unwrap_or_default(),
            year: row.try_get("year").unwrap_or_default(),
            short: row.try_get("short").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            group_points: row.try_get("groupPoints").unwrap_or_default(),
            image: row.try_get("image").unwrap_or_default(),
            deleted_at: row.try_get("deletatedAt").unwrap_or_default(),
        }
    }
}
