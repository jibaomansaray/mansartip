use std::collections::HashMap;

use crate::{
    app::app_state::DbRow,
    country_system::country_dtos::country_entity_api_dto::CountryEntityApiDto,
};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct CountryEntity {
    pub id: u64,
    pub internal_id: String,
    pub year: i16,
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

pub struct CountryJoinProcessor<'a> {
    pub fields: String,
    pub field_map: HashMap<&'a str, String>,
}

impl<'a> CountryJoinProcessor<'a> {
    pub fn transform(&self, row: &DbRow) -> Option<CountryEntityApiDto> {
        CountryEntity::new_for_join(row, &self.field_map)
    }
}

impl CountryEntity {
    pub fn new_for_join(row: &DbRow, map: &HashMap<&str, String>) -> Option<CountryEntityApiDto> {
        if let Some(entity) = Self::pluck_values(row, map) {
            Some(CountryEntityApiDto::from(entity))
        } else {
            None
        }
    }

    pub fn generate_join_fields(prefix: Option<&str>) -> CountryJoinProcessor {
        let table = prefix.unwrap_or_else(|| "country");
        let as_field = |name| format!("`{0}`.`{1}` as {0}_{1}", table, name);

        let map = HashMap::from([
            ("id", as_field("id")),
            ("internalId", as_field("internalId")),
            ("name", as_field("name")),
            ("short", as_field("short")),
            ("groupPoints", as_field("groupPoints")),
            ("image", as_field("image")),
            ("deletedAt", as_field("deletedAt")),
        ]);

        let mut fields = String::new();

        for entry in &map {
            if !fields.is_empty() {
                fields.push_str(",")
            }
            fields.push_str(entry.1.as_str())
        }

        CountryJoinProcessor {
            fields,
            field_map: map,
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

        let year_field = match map.get("year") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "year",
        };
        let short_field = match map.get("short") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "short",
        };

        let name_field = match map.get("name") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "name",
        };

        let group_points_field = match map.get("groupPoints") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "groupPoints",
        };
        let image_field = match map.get("image") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "image",
        };
        let deleted_at_field = match map.get("deletedAt") {
            Some(e) => e.split("as").last().unwrap().trim(),
            _ => "deletedAt",
        };

        let id: u64 = row.try_get(id_field).unwrap_or_default();

        if id > 0 {
            Some(Self {
                id,
                internal_id: row.try_get(internal_id_field).unwrap_or_default(),
                year: row.try_get(year_field).unwrap_or_default(),
                short: row.try_get(short_field).unwrap_or_default(),
                name: row.try_get(name_field).unwrap_or_default(),
                group_points: row.try_get(group_points_field).unwrap_or_default(),
                image: row.try_get(image_field).unwrap_or_default(),
                deleted_at: row.try_get(deleted_at_field).unwrap_or_default(),
            })
        } else {
            None
        }
    }
}

impl From<DbRow> for CountryEntity {
    fn from(row: DbRow) -> Self {
        let map = HashMap::new();
        Self::pluck_values(&row, &map).unwrap()
    }
}
