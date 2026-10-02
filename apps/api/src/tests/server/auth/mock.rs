use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, HeaderValue},
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, SubsecRound, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    tests::server::{TEST_COOKIE_KEY, TEST_WEBHOOK_SECRET},
    types::extract::Json,
    utils::route::{WEBHOOK_ID, WEBHOOK_SIGNATURE, WEBHOOK_TIMESTAMP, create_webhook_signature},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MockUserData {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl MockUserData {
    fn json(&self) -> serde_json::Value {
        json!({
            "user": {
                "id": self.id,
                "name": self.name,
                "email": self.email,
                "createdAt": self.created_at,
            },
            "session": {
                "token": self.token,
                "expiresAt": self.expires_at,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct MockAuthAppState {
    user_auth: Arc<RwLock<HashMap<(String, String), Uuid>>>,
    user_map: Arc<RwLock<HashMap<Uuid, MockUserData>>>,
    api_url: String,
}

impl MockAuthAppState {
    pub fn init(api_ip: SocketAddr) -> Self {
        Self {
            user_auth: Arc::new(RwLock::new(HashMap::new())),
            user_map: Arc::new(RwLock::new(HashMap::new())),
            api_url: format!("http://{}", api_ip),
        }
    }
}

pub fn build_mock_server() -> Router<MockAuthAppState> {
    Router::new()
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/get-session", get(get_session))
}

#[derive(Deserialize)]
struct RegisterBody {
    email: String,
    password: String,
    name: String,
}

async fn register_handler(
    State(app_state): State<MockAuthAppState>,
    Json(body): Json<RegisterBody>,
) -> (StatusCode, String) {
    let mut user_auth = app_state.user_auth.write().await;
    if user_auth
        .iter()
        .find(|((email, _), _)| email == &body.email)
        .is_some()
    {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            "User already exists. Use another email.".to_string(),
        );
    }
    let id = Uuid::new_v4();
    user_auth.insert((body.email.clone(), body.password), id);

    let mut user_map = app_state.user_map.write().await;
    let user_data = MockUserData {
        id,
        name: body.name,
        email: body.email,
        created_at: Utc::now().trunc_subsecs(6),
        token: Uuid::new_v4().to_string(),
        expires_at: DateTime::parse_from_rfc3339("2999-01-01T12:34:56Z")
            .unwrap()
            .to_utc(),
    };
    user_map.insert(id, user_data.clone());

    // provision user
    let webhook_id = Uuid::new_v4();
    let timestamp = Utc::now().timestamp().to_string();
    let body = json!({
        "id": user_data.id,
        "displayName": user_data.name,
        "createdAt": user_data.created_at,
    });

    let signature = create_webhook_signature(
        TEST_WEBHOOK_SECRET.as_bytes(),
        webhook_id.as_bytes(),
        timestamp.as_bytes(),
        &serde_json::to_vec(&body).unwrap(),
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        WEBHOOK_ID,
        HeaderValue::from_str(&webhook_id.to_string()).unwrap(),
    );
    headers.insert(
        WEBHOOK_TIMESTAMP,
        HeaderValue::from_str(&timestamp.to_string()).unwrap(),
    );
    headers.insert(
        WEBHOOK_SIGNATURE,
        HeaderValue::from_str(&signature).unwrap(),
    );

    assert!(
        reqwest::Client::new()
            .post(format!("{}/internal/users/provision", app_state.api_url))
            .headers(headers)
            .json(&body)
            .send()
            .await
            .unwrap()
            .status()
            .is_success()
    );

    (StatusCode::OK, user_data.token)
}

async fn get_session(
    cookies: CookieJar,
    State(app_state): State<MockAuthAppState>,
) -> (StatusCode, Json<serde_json::Value>) {
    let token = if let Some(token) = cookies
        .get(TEST_COOKIE_KEY)
        .map(|cookie| cookie.value().to_owned())
    {
        token
    } else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::Value::Null));
    };

    let user_map = app_state.user_map.read().await;
    let user_id = if let Some((user_id, _)) = user_map
        .iter()
        .find(|(_, user_data)| user_data.token == token)
    {
        user_id
    } else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::Value::Null));
    };
    if let Some(user_data) = user_map.get(user_id) {
        (StatusCode::OK, Json(user_data.json()))
    } else {
        (StatusCode::UNAUTHORIZED, Json(serde_json::Value::Null))
    }
}
