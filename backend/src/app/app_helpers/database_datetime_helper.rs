use chrono::{DateTime, Utc};
use sqlx::Row;

use crate::app::app_state::DbRow;

type DateTimeFieldValue = Option<DateTime<Utc>>;
pub(crate) fn datetime_from_db_row_field(field: &str, row: &DbRow) -> DateTimeFieldValue {
    let dt: Option<DateTime<Utc>> = match row.try_get(field) {
        Ok(date) => Some(date),
        _ => None,
    };

    dt
}

pub(crate) fn created_at_field_value(row: &DbRow) -> DateTimeFieldValue {
    datetime_from_db_row_field("createdAt", row)
}

pub(crate) fn updated_at_field_value(row: &DbRow) -> DateTimeFieldValue {
    datetime_from_db_row_field("updatedAt", row)
}

pub(crate) fn deleted_at_field_value(row: &DbRow) -> DateTimeFieldValue {
    datetime_from_db_row_field("deletedAt", row)
}
