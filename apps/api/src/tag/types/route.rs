use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct URLQueryOpts {
    pub page: Option<u32>,
    pub limit: Option<u32>,

    pub category: Option<String>,
}

#[derive(Debug)]
pub struct CreateRequest {
    pub label: String,
    pub category: Option<String>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateRequest {
    pub label: Option<String>,
    pub category: Option<Option<String>>,

    pub position_key: Option<String>,
}
