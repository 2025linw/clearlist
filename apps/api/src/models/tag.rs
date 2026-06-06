//! Tag Model
//!
//! This module contains the tag model

use chrono::Utc;
use serde::Deserialize;
use sqlx::FromRow;
use ts_rs::TS;

/// Tag Model Type
///
/// This is the ground truth model for tag; it exactly matches database schema
#[allow(dead_code)]
#[derive(Debug, FromRow, TS)]
#[cfg_attr(test, derive(Clone, PartialEq))]
#[ts(export, rename = "Tag")]
pub struct Model {
    pub id: uuid::Uuid,

    pub label: String,
    pub category: Option<String>,

    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,

    pub created_by: uuid::Uuid,
}

/// Tag DTO Model
///
/// This represents the fields a client is able to create/modify for a Tag
#[derive(Debug, Deserialize, TS)]
#[cfg_attr(test, derive(Default, Clone))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[ts(export, rename = "TagDTO")]
pub struct DtoModel {
    #[serde(default)]
    pub label: String,
    pub category: Option<String>,
}
