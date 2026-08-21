use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub name: String,

    pub position_key: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
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
