pub mod auth;

use std::{net::SocketAddr, sync::OnceLock};

use axum::Router;
use reqwest::Url;
use serde_json::json;
use tokio::net::TcpListener;

use crate::{
    Config, GenericAppState,
    tests::mocks::{MockAppState, MockTagRepository, MockTaskRepository, MockUserRepository},
};

use auth::start_server as start_auth_server;

pub const TEST_COOKIE_KEY: &str = "test_cookie";
pub const TEST_WEBHOOK_SECRET: &str = "test_secret";

pub static TEST_USER_INFO: OnceLock<serde_json::Value> = OnceLock::new();
pub fn get_test_user_info() -> &'static serde_json::Value {
    TEST_USER_INFO.get_or_init(|| {
        json!({
            "email": "testuser@email.com",
            "password": "testpass",
            "name": "Test User",
        })
    })
}

// pub const TEST_USER_INFO: serde_json::Value = json!({
//     "email": "testuser@email.com",
//     "password": "testpass",
//     "name": "Test User",
// });

pub struct TestServerIP {
    pub api_addr: SocketAddr,
    pub auth_addr: SocketAddr,
}

pub async fn start_test_servers(
    create_router: fn() -> Router<
        GenericAppState<MockUserRepository, MockTaskRepository, MockTagRepository>,
    >,
) -> TestServerIP {
    let api_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let api_addr = api_listener.local_addr().unwrap();

    let auth_addr = start_auth_server(api_addr).await;

    let config = Config::init(
        TEST_COOKIE_KEY.to_string(),
        Url::parse(&format!("http://{auth_addr}")).unwrap(),
        TEST_WEBHOOK_SECRET.to_string(),
    );
    let app_state = MockAppState::init(config);
    let app = create_router().with_state(app_state);

    tokio::spawn(async move {
        axum::serve(api_listener, app)
            .await
            .expect("server failed to start");
    });

    TestServerIP {
        api_addr,
        auth_addr,
    }
}
