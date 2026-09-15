use axum::{http::StatusCode, response::IntoResponse};
use serde_json::{Map, Value};

use crate::types::extract::Json;

pub struct Response {
    code: StatusCode,
    message: Option<String>,
    data: Option<Value>,
    custom: Map<String, Value>,
}

impl Response {
    pub fn new(code: StatusCode) -> Self {
        Self {
            code,
            message: None,
            data: None,
            custom: Map::new(),
        }
    }

    pub fn message(mut self, msg: String) -> Self {
        self.message = Some(msg);

        self
    }

    pub fn data(mut self, data: Value) -> Self {
        self.data = Some(data);

        self
    }

    pub(crate) fn add_kv(mut self, key: &str, value: Value) -> Self {
        self.custom.insert(key.to_string(), value);

        self
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        if self.code == StatusCode::NO_CONTENT {
            (self.code).into_response()
        } else {
            let mut body: Map<String, Value> = Map::new();
            body.insert(
                "status".to_string(),
                Value::Number(self.code.as_u16().into()),
            );

            if let Some(message) = self.message {
                body.insert("message".to_string(), Value::String(message));
            }

            if let Some(data) = self.data {
                body.insert("data".to_string(), data);
            }

            if !self.custom.is_empty() {
                for (key, value) in self.custom {
                    body.insert(key, value);
                }
            }

            (self.code, Json(Value::Object(body))).into_response()
        }
    }
}
