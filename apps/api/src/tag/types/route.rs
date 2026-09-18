use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct URLQueryOpts {
    pub page: Option<u32>,
    pub limit: Option<u32>,

    pub category: Option<String>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "tag/CreateRequest.ts")]
pub struct CreateRequest {
    pub label: String,
    pub category: Option<String>,

    pub position_key: String,
}

#[derive(Debug, Default, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "tag/UpdateRequest.ts")]
pub struct UpdateRequest {
    pub label: Option<String>,
    #[ts(type = "string | null")]
    pub category: Option<Option<String>>,

    pub position_key: Option<String>,
}

impl UpdateRequest {
    pub fn is_noop(&self) -> bool {
        let Self {
            label,
            category,
            position_key,
        } = self;

        label.is_none() && category.is_none() && position_key.is_none()
    }
}
