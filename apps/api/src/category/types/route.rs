use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "category/CreateRequest.ts")]
pub struct CreateRequest {
    pub name: String,

    pub position_key: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "category/UpdateRequest.ts")]
pub struct UpdateRequest {
    pub name: Option<String>,

    pub position_key: Option<String>,
}

impl UpdateRequest {
    pub fn is_noop(&self) -> bool {
        let Self { name, position_key } = self;

        name.is_none() && position_key.is_none()
    }
}
